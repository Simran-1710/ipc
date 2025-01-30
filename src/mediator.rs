use std::{sync::mpsc::{self, Sender}, collections::HashMap, time::Duration, sync::Arc};
use tokio::sync::Mutex;
use anyhow::Result;
use crate::response::Response;
use crate::ipc_trait::IPC;
use serde::Serialize;
use async_trait;

#[derive(Clone)]
pub(crate) struct IPCMediator {
    default_timeout: Duration,
    topics: Arc<Mutex<HashMap<String, Sender<Response>>>>, // handling multi-threads, to-do: handle multiple servers
}

impl IPCMediator {
    pub(crate) fn new(default_timeout_secs: u64) -> Self {
        let default_timeout = Duration::from_secs(default_timeout_secs);
        let topics = Arc::new(Mutex::new(HashMap::new()));
        IPCMediator {
            default_timeout,
            topics,
        }
    }

    fn handle_timeout(&self, timeout: Duration, sender: Sender<Response>) {
        tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            let _ = sender.send(Response::Timeout);
        });
    }
}

#[async_trait::async_trait]
impl IPC for IPCMediator {
    async fn publish<T>(&self, identifier: &str, payload: &T) -> Result<()>
where
    T: Serialize + Send + Sync,
{
    let json = serde_json::to_string(payload)?;
    let response = Response::Success(json);

    let topics = self.topics.lock().await;
    println!("[PUBLISH] Checking HashMap state: {:?}", topics.keys().collect::<Vec<_>>());

    if let Some(sender) = topics.get(identifier) {
        sender.send(response)?;
    } else {
        return Err(anyhow::anyhow!("No subscriber found for identifier: {}", identifier));
    }
    Ok(())
}

    async fn subscribe<T>(&self, identifier: &str, timeout: Option<u64>) -> Result<Response<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let timeout = timeout.map_or_else(|| self.default_timeout, |tm| Duration::from_secs(tm));
        println!("Timeout: {:?}",timeout);

        let (sender, receiver) = mpsc::channel(); //can use oneshot also for single producer/consumer
        {
            let mut topics = self.topics.lock().await;
            topics.insert(identifier.to_owned(), sender.to_owned());
            println!("[SUBSCRIBE] HashMap state after insert: {:?}", topics.keys().collect::<Vec<_>>());
        }

        self.handle_timeout(timeout, sender);
        let response = receiver.recv().map_err(|_| anyhow::anyhow!("Receiver error"))?;
        Ok(response.extract())
    }
}

impl std::fmt::Debug for IPCMediator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "IPC".fmt(f)
    }
}

impl Serialize for IPCMediator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str("IPC")
    }
}