use crate::abi::Bridge;
use crate::egress::{SpawnJob, SpawnVariant};
use crate::{
    egress::{AssetTransferJob, Handle},
    ingress::{SpawnEventRow, TransferRow},
};
use async_stream::stream;
use async_trait::async_trait;
use bytes::Bytes;
use ethers::prelude::Middleware;
use futures::Future;
use futures_core::Stream;
use sqlx::{PgPool, Postgres, Transaction};
use std::time::Duration;
use xcvm_core::XCVMNetwork;

#[async_trait]
pub trait Store {
    async fn store(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<()>;
}

pub trait Queue<T: MarkHandled + Handle>: Stream<Item = T> {}

#[async_trait]
pub trait MarkHandled {
    async fn mark_handled(&self, tx: &mut Transaction<Postgres>) -> anyhow::Result<()>;
}

pub struct PollingQueue<F> {
    db: PgPool,
    poll_fn: F,
    was_success: bool,
    backoff: Duration,
}

impl<F, FUT, T> PollingQueue<F>
where
    F: Fn(sqlx::pool::PoolConnection<sqlx::Postgres>) -> FUT,
    FUT: Future<Output = anyhow::Result<Option<T>>>,
{
    pub fn new(db: PgPool, poll_fn: F) -> Self {
        PollingQueue {
            db,
            poll_fn,
            was_success: true,
            backoff: Duration::from_secs(3),
        }
    }

    pub fn into_stream(mut self) -> impl Stream<Item = anyhow::Result<T>> {
        stream! {
            loop {

                if !self.was_success {
                    tokio::time::sleep(self.backoff).await;
                    self.was_success = true;
                }

                let conn = self.db.acquire().await?;
                let item = (self.poll_fn)(conn).await?;

                if item.is_none() {
                    self.was_success = false;
                    continue;
                }

                yield Ok(item.unwrap())
            }
        }
    }
}

pub async fn poll_for_event<M: Middleware + Send + Sync>(
    bridge: Bridge<M>,
    mut conn: sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> anyhow::Result<Option<SpawnJob<M>>> {
    let r = sqlx::query!(
        "
        WITH queue_to_process AS (
            UPDATE queue
            SET locked = true
            WHERE id = (
                SELECT id
                FROM queue q
                WHERE locked = false AND handled = false AND NOT EXISTS (
                  SELECT * FROM transfer_queue tq
                  WHERE tq.queue_id = q.id
                  AND NOT EXISTS (
                    SELECT * FROM transfer_done td
                    WHERE td.id = tq.transfer_id
                  )
                )
                ORDER BY id ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING *
        )
        SELECT *, (
            SELECT ARRAY_AGG(tq.transfer_id) AS transfers
            FROM transfer_queue tq
            WHERE tq.queue_id = id
        )
        FROM queue_to_process
        "
    )
    .fetch_optional(&mut conn)
    .await?;
    match r {
        Some(record) => {
            let id = record.id as u16;
            let event = SpawnEventRow {
                caller: record.caller,
                data: Bytes::from(record.payload),
                salt: Bytes::from(record.salt),
                destination: XCVMNetwork::from(record.destination as u32),
                source: XCVMNetwork::from(record.source as u32),
                transfers: record
                    .transfers
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .map(|t| {
                        t.try_into()
                            .map_err(|_| anyhow::anyhow!("impossible; qed;"))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            };
            match event.source {
                XCVMNetwork::PICASSO => Ok(Some(SpawnJob {
                    event,
                    id,
                    variant: SpawnVariant::Eth(bridge),
                })),
                XCVMNetwork::ETHEREUM => Ok(Some(SpawnJob {
                    event,
                    id,
                    variant: SpawnVariant::Dali,
                })),
                _ => unreachable!(),
            }
        }
        None => Ok(None),
    }
}

pub async fn poll_for_transfer(
    mut conn: sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> anyhow::Result<Option<AssetTransferJob>> {
    let r = sqlx::query!(
        "
        WITH transfer_to_process AS (
            SELECT *
            FROM transfer t
            WHERE NOT EXISTS (
                SELECT id
                FROM transfer_locked tl
                WHERE t.id = tl.id
            )
            AND NOT EXISTS (
                SELECT id
                FROM transfer_done td
                WHERE t.id = td.id
            )
            ORDER BY t.created_at ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
        ), applied_lock AS (
            INSERT INTO transfer_locked (id)
            SELECT id
            FROM transfer_to_process
            RETURNING id
        )
        SELECT *
        FROM transfer_to_process
    "
    )
    .fetch_optional(&mut conn)
    .await?;
    match r {
        Some(record) => Ok(Some(
            TransferRow {
                id: record
                    .id
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("impossible; qed;"))?,
                from: record
                    .from
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("impossible; qed;"))?,
                to_address: record
                    .to
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("impossible; qed;"))?,
                asset_id: record.asset_id as u128,
                network_id: XCVMNetwork::from(record.network_id as u32),
                remote_asset_id: record
                    .remote_asset_id
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("impossible; qed;"))?,
                amount: record.amount as u128,
            }
            .into(),
        )),
        None => Ok(None),
    }
}
