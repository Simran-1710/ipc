use async_trait::async_trait;
use serde::Serialize;
use anyhow::Result;
use crate::response::Response;

/// The IPC trait defines the interface for communication (publish/subscribe)
#[async_trait]
pub trait IPC: Send + Sync + std::fmt::Debug + Serialize {
    async fn publish<T>(&self, identifier: &str, payload: &T) -> Result<()>
    where
        T: Serialize + Send + Sync;

    async fn subscribe<T>(&self, identifier: &str, timeout: Option<u64>) -> Result<Response<T>>
    where
        T: serde::de::DeserializeOwned;
}
