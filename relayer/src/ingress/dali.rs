use crate::{
    dali::dali_runtime,
    ingress::{SpawnEventRow, StorableEvent, TransferRow},
};
use async_stream::stream;
use futures::Stream;
use futures_util::StreamExt;
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
use xcvm_core::XCVMNetwork;

#[derive(Debug)]
pub struct XCVMEvent(dali_runtime::xcvm::events::Spawn);

pub struct Listener;
impl Listener {
    pub fn stream() -> impl Stream<Item = anyhow::Result<StorableEvent>> {
        stream! {
          tracing::info!("Connecting to local dali instance...");
          let api = ClientBuilder::new()
            .set_url("ws://127.0.0.1:9988")
            .build()
            .await.map_err(|_| anyhow::anyhow!("couldn't build dali client."))?
            .to_runtime_api::<dali_runtime::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();
          tracing::info!("Capturing events...");
          let mut envelopes = api
            .events()
            .subscribe()
            .await
            .map_err(|_| anyhow::anyhow!("couldn't subscribe to dali events."))?
            .filter_events::<(
              dali_runtime::xcvm::events::Spawn,
              dali_runtime::mosaic::events::TransferOut,
            )>();
          while let Some(Ok(envelope)) = envelopes.next().await {
            let block_hash = envelope.block_hash;
            let event = envelope.event;
            tracing::info!("{:#?}, {:?}", event, block_hash);
            match event {
              (Some(spawn), _) => {
                yield Ok(StorableEvent::Spawn(SpawnEventRow {
                  caller: format!("0x{}", hex::encode(spawn.who)),
                  data: spawn.program.into(),
                  salt: spawn.salt.into(),
                  destination: XCVMNetwork::ETHEREUM,
                  source: XCVMNetwork::PICASSO,
                  transfers: spawn.network_txs.into_iter().map(Into::into).collect()
                }))
              },
              (_, Some(transfer_out)) => {
                yield Ok(StorableEvent::Transfer(TransferRow {
                  id: transfer_out.id.into(),
                  from: transfer_out.from.into(),
                  to_address: transfer_out.to.0,
                  asset_id: transfer_out.asset_id.0,
                  network_id: XCVMNetwork::from(transfer_out.network_id),
                  remote_asset_id: match transfer_out.remote_asset_id {
                    dali_runtime::runtime_types::common::types::MosaicRemoteAssetId::EthereumTokenAddress(x) => x
                  },
                  amount: transfer_out.amount
                }))
              },
              _ => {}
            }
          }
        }
    }
}

pub mod mock {
    use crate::ingress::{SpawnEventRow, StorableEvent};
    use anyhow::Ok;
    use futures::{stream, Stream, StreamExt};
    use rand::prelude::SmallRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    use xcvm_core::XCVMNetwork;

    pub struct Listener {
        addresses: Vec<String>,
        rng: SmallRng,
    }

    impl Default for Listener {
        fn default() -> Self {
            Self {
                addresses: vec!["Address3".to_owned(), "Address4".to_owned()],
                rng: SmallRng::from_entropy(),
            }
        }
    }

    impl Iterator for Listener {
        type Item = StorableEvent;

        fn next(&mut self) -> Option<Self::Item> {
            Some(StorableEvent::Spawn(SpawnEventRow {
                caller: self
                    .addresses
                    .choose(&mut self.rng)
                    .expect("mock listener should have 1 or more addresses configured")
                    .to_owned(),
                data: Default::default(),
                salt: Default::default(),
                destination: XCVMNetwork::ETHEREUM,
                source: XCVMNetwork::PICASSO,
                transfers: Default::default(),
            }))
        }
    }

    impl Listener {
        #[allow(unused)]
        pub fn stream() -> impl Stream<Item = StorableEvent> {
            stream::iter(Listener::default())
        }

        #[allow(unused)]
        pub fn ingress() -> impl Stream<Item = anyhow::Result<StorableEvent>> {
            Self::stream().map(Ok)
        }
    }
}
