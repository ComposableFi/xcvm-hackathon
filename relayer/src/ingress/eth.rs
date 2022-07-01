use super::StorableEvent;
use super::TransferRow;
use crate::abi::{Bridge, BridgeEvents, SpawnFilter as Spawn};
use async_stream::stream;
use futures::{Stream, StreamExt, TryStreamExt};
use rand::Rng;
use rand::SeedableRng;
use xcvm_core::XCVMNetwork;

// DEAD CODE
pub struct Listener {}
impl Listener {
    pub fn stream<M: ethers::providers::Middleware>(
        contract: Bridge<M>,
    ) -> impl Stream<Item = anyhow::Result<StorableEvent>> {
        let mut rng = rand::rngs::StdRng::from_entropy();
        stream! {
            tracing::info!("listening for events for type: {}", std::any::type_name::<Spawn>());
            let events = contract.events();
            let stream = events.stream().await.map_err(|err| anyhow::anyhow!("unable to obtain a stream for contract events: {:?}", err))?;
            let mut event_stream = stream.map_err(|err| anyhow::anyhow!("error getting event from stream: {:?}", err));
            while let Some(item) =  event_stream.next().await {
                match item {
                    Ok(BridgeEvents::TransferFilter(item)) => {
                        tracing::info!("{:?}", item);
                    }
                    Ok(BridgeEvents::CallFilter(item)) => {
                        tracing::info!("{:?}", item);
                    }
                    Ok(BridgeEvents::SucceededFilter(item)) => {
                        tracing::info!("{:?}", item);
                    }
                    Ok(BridgeEvents::SpawnFilter(item)) => {
                        tracing::info!("{:?}", item);
                        let mut transfers = Vec::new();
                        for (asset_id, asset_amount) in item.assets {
                            let mut id = [0u8; 32];
                            rng.fill(&mut id);
                            transfers.push(id);
                            yield Ok(StorableEvent::Transfer(TransferRow {
                                id,
                                from: [0u8; 32],
                                to_address: [0u8; 20],
                                asset_id: 0,
                                network_id: XCVMNetwork::PICASSO,
                                remote_asset_id: [&[0u8; 16][..], &asset_id.to_be_bytes()[..]].concat().try_into().expect("impossible"),
                                amount: asset_amount,
                            }))
                        }
                        yield Ok(StorableEvent::Spawn(crate::SpawnEventRow {
                            caller: item.origin.1.to_string(),
                            data: item.program.as_bytes().to_vec().into(),
                            salt: item.salt.0.into(),
                            destination: XCVMNetwork::PICASSO,
                            source: XCVMNetwork::ETHEREUM,
                            transfers
                        }))
                    },
                    Ok(_) => {}
                    Err(err) => tracing::warn!("error getting event from stream: {:?}", err),
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
                addresses: vec!["Address1".to_owned(), "Address2".to_owned()],
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
                destination: XCVMNetwork::PICASSO,
                source: XCVMNetwork::ETHEREUM,
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
