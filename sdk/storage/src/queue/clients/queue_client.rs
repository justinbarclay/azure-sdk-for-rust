use crate::clients::StorageClient;
use crate::queue::prelude::*;
use crate::queue::requests::*;
use std::sync::Arc;

pub trait AsQueueClient<QN: Into<String>> {
    fn as_queue_client(&self, queue_name: QN) -> Arc<QueueClient>;
}

impl<QN: Into<String>> AsQueueClient<QN> for Arc<StorageClient> {
    fn as_queue_client(&self, queue_name: QN) -> Arc<QueueClient> {
        QueueClient::new(self.clone(), queue_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct QueueClient {
    storage_client: Arc<StorageClient>,
    queue_name: String,
}

impl QueueClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>, queue_name: String) -> Arc<Self> {
        Arc::new(Self {
            storage_client,
            queue_name,
        })
    }

    pub(crate) fn storage_client(&self) -> &StorageClient {
        self.storage_client.as_ref()
    }

    pub(crate) fn queue_url(&self) -> Result<url::Url, url::ParseError> {
        self.storage_client()
            .storage_account_client()
            .queue_storage_url()
            .join(&self.queue_name)
    }

    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub fn put(&self) -> PutMessageBuilder {
        PutMessageBuilder::new(self)
    }

    pub fn peek(&self) -> PeekMessagesBuilder {
        PeekMessagesBuilder::new(self)
    }

    pub fn get(&self) -> GetMessagesBuilder {
        GetMessagesBuilder::new(self)
    }

    pub fn delete<'a>(&'a self, pop_receipt: &'a dyn PopReceipt) -> DeleteMessageBuilder {
        DeleteMessageBuilder::new(self, pop_receipt)
    }

    pub fn clear(&self) -> ClearMessagesBuilder {
        ClearMessagesBuilder::new(self)
    }
}
