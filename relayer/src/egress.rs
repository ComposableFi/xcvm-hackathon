use std::marker::PhantomData;

use crate::egress::dali_runtime::runtime_types::xcvm_core::asset::{
    Displayed, XCVMAsset, XCVMTransfer,
};
use crate::{abi::Bridge, dali::dali_runtime};
use crate::{
    abi::{AssetsAmount, Origin},
    ingress::{SpawnEventRow, TransferRow},
    storage::MarkHandled,
};
use async_trait::async_trait;
use ethers::prelude::{LocalWallet, Middleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use sp_keyring::AccountKeyring;
use sqlx::{Acquire, Postgres, Transaction};
use subxt::{ClientBuilder, DefaultConfig, PairSigner, PolkadotExtrinsicParams};
use xcvm_core::XCVMNetwork;

#[derive(Debug)]
pub struct SpawnJob<M: Middleware> {
    pub id: u16,
    pub event: SpawnEventRow,
    pub variant: SpawnVariant<M>,
}

#[derive(Debug)]
pub enum SpawnVariant<M: Middleware> {
    Eth(Bridge<M>),
    Dali,
}

#[derive(Debug)]
pub struct AssetTransferJob(TransferRow);

impl From<TransferRow> for AssetTransferJob {
    fn from(x: TransferRow) -> Self {
        AssetTransferJob(x)
    }
}

#[async_trait]
pub trait Handle {
    async fn handle(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<()>;
}

#[async_trait]
impl Handle for SpawnJob<SignerMiddleware<Provider<Http>, LocalWallet>> {
    async fn handle(&self, _tx: &mut Transaction<Postgres>) -> anyhow::Result<()> {
        // Some missing implementations in SQLX means we need to remap the array to create a PgArray of PgArrays.
        let event_id_refs: Vec<_> = self
            .event
            .transfers
            .iter()
            .map(|id| Vec::from(id as &[u8]))
            .collect();

        let transfers = sqlx::query!(
            r#"SELECT id, "from", "to", asset_id, network_id, remote_asset_id, amount, created_at FROM transfer WHERE id = ANY($1)"#,
            event_id_refs.as_slice()
        ).fetch_all(_tx.acquire().await?).await?;

        let account: Vec<u8> = hex::decode(&self.event.caller[2..]).unwrap();
        let program = std::str::from_utf8(self.event.data.as_ref())
            .expect("programs are utf8 for now")
            .to_string();
        match &self.variant {
            SpawnVariant::Eth(bridge) => {
                bridge
                    .bridge(
                        Origin {
                            network_id: self.event.source.into(),
                            account: account.into(),
                        },
                        self.event.salt.clone().into(),
                        transfers
                            .iter()
                            .map(|transfer| AssetsAmount {
                                id: u32::from_be_bytes(
                                    (&transfer.remote_asset_id[16..20])
                                        .try_into()
                                        .expect("impossible"),
                                ),
                                amount: transfer.amount as u128, // TODO i64 -> u128
                            })
                            .collect(),
                        program,
                    )
                    .send()
                    .await?
                    .await?;
                Ok(())
            }
            SpawnVariant::Dali => {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let api =
                    ClientBuilder::new()
                        .set_url("ws://127.0.0.1:9988")
                        .build()
                        .await
                        .map_err(|_| anyhow::anyhow!("couldn't build dali client."))?
                        .to_runtime_api::<dali_runtime::RuntimeApi<
                            DefaultConfig,
                            PolkadotExtrinsicParams<DefaultConfig>,
                        >>();

                let call = crate::dali::dali_runtime::runtime_types::dali_runtime::Call::XCVM(
                    crate::dali::dali_runtime::runtime_types::pallet_xcvm::pallet::Call::execute_json_privileged {
                        real_origin: TryInto::<[u8; 32]>::try_into(account)
                            .expect("impossible")
                            .into(),
                        salt: self.event.salt.to_vec(),
                        funds: XCVMTransfer(
                            transfers
                                .into_iter()
                                .map(|transfer| {
                                    (
                                        XCVMAsset(u32::from_be_bytes(
                                            (&transfer.remote_asset_id[16..20])
                                                .try_into()
                                                .expect("impossible"),
                                        )),
                                        Displayed(transfer.amount as u128),
                                    )
                                })
                                .collect(),
                            PhantomData,
                        ),
                        program: self.event.data.to_vec(),
                    }
                );
                let _ = api
                    .tx()
                    .sudo()
                    .sudo(call)?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;
                Ok(())
            }
        }
    }
}

#[async_trait]
impl Handle for AssetTransferJob {
    async fn handle(&self, _tx: &mut Transaction<Postgres>) -> anyhow::Result<()> {
        let signer = PairSigner::new(AccountKeyring::Alice.pair());
        let api =
            ClientBuilder::new()
            .set_url("ws://127.0.0.1:9988")
            .build()
            .await
            .map_err(|_| anyhow::anyhow!("couldn't build dali client."))?
            .to_runtime_api::<dali_runtime::RuntimeApi<
                    DefaultConfig,
                PolkadotExtrinsicParams<DefaultConfig>,
                >>();
        match self.0.network_id {
            XCVMNetwork::ETHEREUM => {
                let _ = api.tx()
                    .mosaic()
                    .accept_transfer(
                        self.0.from.into(),
                        self.0.network_id.into(),
                        dali_runtime::runtime_types::common::types::MosaicRemoteAssetId::EthereumTokenAddress(self.0.remote_asset_id),
                        self.0.amount
                    )?
                    .sign_and_submit_then_watch_default(&signer)
                    .await?
                    .wait_for_finalized_success()
                    .await?;
                Ok(())
            }
            XCVMNetwork::PICASSO => {
                // Nothing to do as we do privileged mint on relayer when executing xcvm
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}

#[async_trait]
impl<M: Send + Sync + Middleware> MarkHandled for SpawnJob<M> {
    async fn mark_handled(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<()> {
        let r = sqlx::query!(
            "
            UPDATE queue
            SET locked = false, handled = true
            WHERE id = $1 AND locked = true AND handled = false
            ",
            self.id as i32
        )
        .execute(tx)
        .await?;
        if r.rows_affected() != 1 {
            anyhow::bail!("job not found")
        }
        Ok(())
    }
}

#[async_trait]
impl MarkHandled for AssetTransferJob {
    async fn mark_handled(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<()> {
        let r = sqlx::query!("INSERT INTO transfer_done (id) values ($1)", &self.0.id[..])
            .execute(tx.acquire().await?)
            .await?;
        if r.rows_affected() != 1 {
            anyhow::bail!("couldn't mark transfer done")
        }
        let r = sqlx::query!("DELETE FROM transfer_locked WHERE id = $1", &self.0.id[..])
            .execute(tx)
            .await?;
        if r.rows_affected() != 1 {
            anyhow::bail!("couldn't mark transfer done")
        }
        Ok(())
    }
}
