use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tokio::sync::RwLock;

use crate::config::{AppConfig, config_dir};
use crate::memory::{MemoryStore, SessionMemory, default_memory_dir};
use crate::runtime::Runtime;
use crate::tool::{ToolManifest, ToolRegistry, default_tools_dir};

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
    tool_registry: RwLock<ToolRegistry>,
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

        let mut tool_registry = ToolRegistry::new(default_tools_dir(&config_dir()?));
        if let Err(e) = tool_registry.load_all() {
            eprintln!("warn: failed to load tools: {e}");
        }

        Ok(Self {
            runtime: Runtime::new(config.provider.clone()),
            config,
            stats: Arc::new(RwLock::new(KernelStats {
                boot_unix_ms,
                turns: 0,
            })),
            memory_store,
            tool_registry: RwLock::new(tool_registry),
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

    pub fn tool_list(&self) -> Vec<ToolManifest> {
        let reg = self.tool_registry.blocking_read();
        reg.list().into_iter().cloned().collect()
    }

    pub fn tool_add(&self, manifest_path: &std::path::Path) -> Result<ToolManifest> {
        let mut reg = self.tool_registry.blocking_write();
        reg.add(manifest_path)
    }

    pub fn tool_remove(&self, name: &str) -> Result<()> {
        let mut reg = self.tool_registry.blocking_write();
        reg.remove(name)
    }

    pub async fn tool_invoke(
        &self,
        name: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let reg = self.tool_registry.read().await;
        reg.invoke(name, params).await
    }
}
