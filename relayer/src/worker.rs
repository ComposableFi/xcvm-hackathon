use std::fmt::Debug;

use crate::{
    egress::Handle,
    storage::{MarkHandled, Store},
};
use futures::Stream;
use futures_util::TryStreamExt;
use sqlx::PgPool;

pub async fn listen_and_store<T: Store + Debug>(
    pool: PgPool,
    ingress: impl Stream<Item = anyhow::Result<T>>,
) -> anyhow::Result<()> {
    ingress
        .try_for_each(move |event| {
            let pool = pool.clone();
            async move {
                let mut tx = pool.begin().await?;
                tracing::info!(event = debug(&event), "store event");
                event.store(&mut tx).await?;
                tx.commit().await?;
                Ok(())
            }
        })
        .await
}

pub async fn dequeue_and_handle<T: Handle + MarkHandled + Debug>(
    pool: PgPool,
    channel: impl Stream<Item = anyhow::Result<T>>,
) -> anyhow::Result<()> {
    channel
        .try_for_each(move |job| {
            let pool = pool.clone();
            async move {
                let mut tx = pool.begin().await?;
                tracing::info!(job = debug(&job), "dequeued and handle a job");
                job.handle(&mut tx).await?;
                job.mark_handled(&mut tx).await?;
                tx.commit().await?;
                Ok(())
            }
        })
        .await
}
