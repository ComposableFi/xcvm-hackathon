use crate::{
    abi::{CallFilter, SpawnFilter, SucceededFilter, TransferFilter},
    storage::Store,
};
use async_trait::async_trait;
use either::Either;
use sqlx::{Acquire, Postgres, Transaction};
use xcvm_core::XCVMNetwork;

pub mod dali;
pub mod eth;

#[derive(Debug)]
pub enum StorableEvent {
    Spawn(SpawnEventRow),
    Transfer(TransferRow),
    CallFilter(CallFilter),           // table
    SpawnFilter(SpawnFilter),         // table
    SucceededFilter(SucceededFilter), // toggle
    TransferFilter(TransferFilter),   // table
}

#[derive(Debug)]
pub struct TransferRow {
    pub id: [u8; 32],
    pub from: [u8; 32],
    pub to_address: [u8; 20],
    /// Picasso asset ID, not XCVM asset ID.
    pub asset_id: u128,
    pub network_id: XCVMNetwork,
    pub remote_asset_id: [u8; 20],
    pub amount: u128,
}

#[derive(Debug)]
pub struct SpawnEventRow {
    pub caller: String,
    pub data: bytes::Bytes,
    pub salt: bytes::Bytes,
    pub destination: XCVMNetwork,
    pub source: XCVMNetwork,
    // mosaic hashes of transfers required to happen before spawning this program
    // indexes the transfers table to obtain the assets.
    pub transfers: Vec<[u8; 32]>,
}

#[async_trait]
impl Store for StorableEvent {
    async fn store(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<()> {
        match self {
            Self::SpawnFilter(SpawnFilter {
                origin,
                salt,
                network,
                assets,
                child_salt,
                program,
            }) => {
                let assets = serde_json::to_value(&assets).unwrap();
                let _ = sqlx::query!(
                    r#"
                    INSERT INTO "SpawnFilter" (
                        origin_network_id,
                        origin_address,
                        salt,
                        program,
                        child_salt,
                        assets,
                        network
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                    origin.0 as i32,
                    AsRef::<[u8]>::as_ref(&origin.1),
                    AsRef::<[u8]>::as_ref(salt),
                    program,
                    AsRef::<[u8]>::as_ref(child_salt),
                    assets,
                    network.as_u32() as i64
                )
                .execute(tx.acquire().await?)
                .await?;
                tracing::info!("inserted CallFilter");
                Ok(())
            }
            Self::Spawn(SpawnEventRow {
                source,
                destination,
                data,
                salt,
                caller,
                transfers,
            }) => {
                let tx = tx.acquire().await?;
                // how bulk in sqlx?
                let row = sqlx::query!(
                  "INSERT INTO queue (source, destination, payload, salt, caller) VALUES ($1, $2, $3, $4, $5) RETURNING id",
                  source.0 as i32,
                  destination.0 as i32,
                  &data[..],
                  &salt[..],
                  caller
                ).fetch_one(tx.acquire().await?).await?;
                tracing::info!(
                    id = row.id,
                    chain_id = source.0,
                    destination = destination.0,
                    "inserted event"
                );
                for transfer in transfers {
                    let _ = sqlx::query!(
                        "INSERT INTO transfer_queue (transfer_id, queue_id) VALUES ($1, $2)",
                        transfer.to_vec(),
                        row.id,
                    )
                    .execute(tx.acquire().await?)
                    .await?;
                    tracing::info!("inserted transfer_queue");
                }
                Ok(())
            }
            Self::Transfer(TransferRow {
                id,
                from,
                to_address,
                asset_id,
                network_id,
                remote_asset_id,
                amount,
            }) => {
                let _ = sqlx::query!(
                  r#"INSERT INTO transfer (id, "from", "to", asset_id, network_id, remote_asset_id, amount, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, now())"#,
                  &id[..],
                  &from[..],
                  &to_address[..],
                  *asset_id as i64,
                  network_id.0 as i32,
                  &remote_asset_id[..],
                  *amount as i64
                ).execute(tx.acquire().await?).await?;
                tracing::info!("inserted transfer");
                Ok(())
            }
            StorableEvent::CallFilter(CallFilter {
                payload,
                origin,
                salt,
                to,
            }) => {
                let payload = serde_json::to_value(&payload).unwrap();
                let _ = sqlx::query!(
                    r#"
                    INSERT INTO "CallFilter" (payload, origin_network_id, origin_address, salt, "to")
                    VALUES ($1, $2, $3, $4, $5)
                    "#,
                    payload,
                    origin.0 as i32,
                    AsRef::<[u8]>::as_ref(&origin.1),
                    AsRef::<[u8]>::as_ref(salt),
                    AsRef::<[u8]>::as_ref(to),
                )
                .execute(tx.acquire().await?)
                .await?;
                tracing::info!("inserted CallFilter");
                Ok(())
            }
            StorableEvent::SucceededFilter(SucceededFilter { origin, salt }) => {
                let _ = sqlx::query!(
                    r#"
                    INSERT INTO "SucceededFilter" (origin_network_id, origin_address, salt)
                    VALUES ($1, $2, $3)
                    "#,
                    origin.0 as i32,
                    AsRef::<[u8]>::as_ref(&origin.1),
                    AsRef::<[u8]>::as_ref(salt),
                )
                .execute(tx.acquire().await?)
                .await?;
                tracing::info!("inserted CallFilter");
                Ok(())
            }
            StorableEvent::TransferFilter(TransferFilter {
                from,
                asset,
                origin,
                salt,
                to,
            }) => {
                let _ = sqlx::query!(
                    r#"
                    INSERT INTO "TransferFilter" (
                        "from",
                        asset_id,
                        asset_amount,
                        origin_network_id,
                        origin_address,
                        salt,
                        "to"
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                    AsRef::<[u8]>::as_ref(from),
                    asset.0 as i32,
                    asset.1 as i64,
                    origin.0 as i32,
                    AsRef::<[u8]>::as_ref(&origin.1),
                    AsRef::<[u8]>::as_ref(salt),
                    AsRef::<[u8]>::as_ref(to),
                )
                .execute(tx.acquire().await?)
                .await?;
                tracing::info!("inserted CallFilter");
                Ok(())
            }
        }
    }
}
