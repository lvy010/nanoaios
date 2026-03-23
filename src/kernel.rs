use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tokio::sync::RwLock;

use crate::config::{AppConfig, config_dir};
use crate::memory::{MemoryStore, SessionMemory, default_memory_dir};
use crate::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct KernelStats {
    pub boot_unix_ms: u128,
    pub turns: u64,
}

pub struct Kernel {
    pub config: AppConfig,
    runtime: Runtime,
    stats: Arc<RwLock<KernelStats>>,
    memory_store: Option<MemoryStore>,
}

impl Kernel {
    pub fn new(config: AppConfig) -> Result<Self> {
        let boot_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let memory_store = if config.memory.enabled {
            Some(MemoryStore::new(default_memory_dir(&config_dir()?)))
        } else {
            None
        };

        Ok(Self {
            runtime: Runtime::new(config.provider.clone()),
            config,
            stats: Arc::new(RwLock::new(KernelStats {
                boot_unix_ms,
                turns: 0,
            })),
            memory_store,
        })
    }

    pub async fn infer_with_session(
        &self,
        prompt: &str,
        session_id: Option<&str>,
    ) -> Result<String> {
        let mut stats = self.stats.write().await;
        stats.turns += 1;
        drop(stats);
        let answer = self.runtime.complete(prompt).await?;
        if let (Some(store), Some(id)) = (&self.memory_store, session_id) {
            store.append_turn(
                id,
                prompt,
                &answer,
                self.config.memory.max_messages_per_session,
            )?;
        }
        Ok(answer)
    }

    pub async fn stats(&self) -> KernelStats {
        self.stats.read().await.clone()
    }

    pub fn session_memory(&self, session_id: &str) -> Result<Option<SessionMemory>> {
        match &self.memory_store {
            Some(store) => store.load_session(session_id),
            None => Ok(None),
        }
    }
}
