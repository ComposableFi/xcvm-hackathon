use crate::{
    abi::{Bridge, BridgeEvents, SpawnFilter as Spawn},
    ingress::{SpawnEventRow, StorableEvent, TransferRow},
    storage::Store,
};
use ethers::{
    abi::Address,
    contract::ContractError,
    prelude::{Http, LocalWallet, Provider, Signer, SignerMiddleware},
};
use futures::{stream::FuturesUnordered, StreamExt};
use futures_util::TryStreamExt;
use rand::{Rng, SeedableRng};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use storage::PollingQueue;
use structopt::StructOpt;
use xcvm_core::XCVMNetwork;

mod abi;
mod dali;
mod egress;
mod ingress;
mod storage;
mod worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = Command::from_args();

    tracing_subscriber::fmt::init();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&cmd.database_url)
        .await?;

    let address = cmd.eth_bridge_contract.parse::<Address>()?;
    let provider = Provider::<Http>::try_from(cmd.ethereum_host)
        .expect("creating a provider from ethereum host");
    let wallet: LocalWallet = cmd.eth_wallet.parse()?;
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(5555u64),
    ));

    let eth_event_worker = {
        let bridge = abi::Bridge::new(address, client.clone());
        let db = db.clone();
        tokio::spawn(async move {
            bridge
                .events()
                .stream()
                .await
                .unwrap()
                .try_for_each(move |event| {
                    let pool = db.clone();
                    async move {
                        let event = handle_event(event).await;
                        let mut tx = pool.begin().await.unwrap();
                        tracing::info!(event = debug(&event), "store event");
                        for e in event {
                            e.unwrap().store(&mut tx).await.unwrap();
                        }
                        tx.commit().await.unwrap();
                        Ok(())
                    }
                })
                .await
                .unwrap();

            Ok(())
        })
    };

    // let eth_worker = {
    //     let bridge = abi::Bridge::new(address, client.clone());
    //     tokio::spawn(worker::listen_and_store(
    //         db.clone(),
    //         ingress::eth::Listener::stream(bridge),
    //     ))
    // };

    let poll_fn = move |db| {
        let bridge = abi::Bridge::new(address, client.clone());
        storage::poll_for_event(bridge, db)
    };
    let event_job_queue = PollingQueue::new(db.clone(), poll_fn).into_stream();
    let transfer_job_queue =
        PollingQueue::new(db.clone(), storage::poll_for_transfer).into_stream();

    let event_executor = tokio::spawn(worker::dequeue_and_handle(db.clone(), event_job_queue));
    let transfer_executor =
        tokio::spawn(worker::dequeue_and_handle(db.clone(), transfer_job_queue));

    let pica_worker = tokio::spawn(worker::listen_and_store(
        db,
        ingress::dali::Listener::stream(),
    ));

    let mut workers: FuturesUnordered<_> = [
        eth_event_worker,
        pica_worker,
        event_executor,
        transfer_executor,
    ]
    .into_iter()
    .collect();

    // First error that we encounter, we log and then exit, as we need all workers operational.
    let result = workers.next().await;

    if result.is_none() {
        tracing::error!("all workers exited without errors");
    } else {
        tracing::error!(exit = debug(result), "worker exited");
    }

    Ok(())
}

async fn handle_event(event: BridgeEvents) -> Vec<anyhow::Result<StorableEvent, ()>> {
    let mut rng = rand::rngs::StdRng::from_entropy();
    match event {
        BridgeEvents::TransferFilter(item) => {
            tracing::info!("{:?}", item);
            vec![Ok(StorableEvent::TransferFilter(item))]
        }
        BridgeEvents::CallFilter(item) => {
            tracing::info!("{:?}", item);
            vec![Ok(StorableEvent::CallFilter(item))]
        }
        BridgeEvents::SucceededFilter(item) => {
            tracing::info!("{:?}", item);
            vec![Ok(StorableEvent::SucceededFilter(item))]
        }
        BridgeEvents::SpawnFilter(item) => {
            tracing::info!("{:?}", item);
            let mut fin = vec![];
            let mut transfers = Vec::new();
            for (asset_id, asset_amount) in item.assets {
                let mut id = [0u8; 32];
                rng.fill(&mut id);
                transfers.push(id);
                fin.push(Ok(StorableEvent::Transfer(TransferRow {
                    id,
                    from: [0u8; 32],
                    to_address: [0u8; 20],
                    asset_id: 0,
                    network_id: XCVMNetwork::PICASSO,
                    remote_asset_id: [&[0u8; 16][..], &asset_id.to_be_bytes()[..]]
                        .concat()
                        .try_into()
                        .expect("impossible"),
                    amount: asset_amount,
                })))
            }
            fin.push(Ok(StorableEvent::Spawn(SpawnEventRow {
                caller: item.origin.1.to_string(),
                data: item.program.as_bytes().to_vec().into(),
                salt: item.salt.0.into(),
                destination: XCVMNetwork::PICASSO,
                source: XCVMNetwork::ETHEREUM,
                transfers,
            })));
            fin
        }
        _ => vec![],
    }
}

#[derive(Debug, StructOpt)]
struct Command {
    /// The Postgresql database used for event persistence.
    #[structopt(
        short,
        long,
        default_value = "postgres://postgres:postgrespassword@0.0.0.0:5432/postgres",
        env = "DATABASE_URL"
    )]
    database_url: String,

    #[structopt(
        short,
        long,
        default_value = "0x8fa39dC9BBF9f0C5E9a8C683dc238df411b192Be",
        env = "ETH_CONTRACT_ADDRESS"
    )]
    eth_bridge_contract: String,

    #[structopt(short, long, default_value = "http://localhost:8178", env = "ETH_HOST")]
    ethereum_host: String,

    #[structopt(
        long,
        default_value = "0d0b4c455973c883bb0fa584f0078178aa90c571a8f1d40f28d2339f4e757dde",
        env = "ETH_WALLET"
    )]
    eth_wallet: String,
}
