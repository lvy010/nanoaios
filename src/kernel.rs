use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use tokio::sync::RwLock;

use crate::config::AppConfig;
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
}

impl Kernel {
    pub fn new(config: AppConfig) -> Self {
        let boot_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        Self {
            runtime: Runtime::new(config.provider.clone()),
            config,
            stats: Arc::new(RwLock::new(KernelStats {
                boot_unix_ms,
                turns: 0,
            })),
        }
    }

    pub async fn infer(&self, prompt: &str) -> Result<String> {
        let mut stats = self.stats.write().await;
        stats.turns += 1;
        drop(stats);
        self.runtime.complete(prompt).await
    }

    pub async fn stats(&self) -> KernelStats {
        self.stats.read().await.clone()
    }
}
