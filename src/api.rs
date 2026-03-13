use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::kernel::Kernel;

pub async fn serve(kernel: Arc<Kernel>) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", kernel.config.api_host, kernel.config.api_port)
        .parse()
        .context("api_host/api_port 解析失败")?;

    let app = Router::new()
        .route("/", get(index))
        .route("/healthz", get(health))
        .route("/v1/kernel/state", get(state))
        .with_state(kernel);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("监听地址失败: {addr}"))?;
    println!("nanoaios api running on http://{addr}");
    axum::serve(listener, app)
        .await
        .context("API 服务异常退出")?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "ok": true,
        "service": "nanoaios",
    }))
}

async fn index() -> Json<serde_json::Value> {
    Json(json!({
        "service": "nanoaios",
        "status": "running",
        "routes": {
            "health": "/healthz",
            "kernel_state": "/v1/kernel/state"
        }
    }))
}

async fn state(State(kernel): State<Arc<Kernel>>) -> Json<serde_json::Value> {
    let stats = kernel.stats().await;
    Json(json!({
        "node_name": kernel.config.node_name,
        "provider_model": kernel.config.provider.model,
        "boot_unix_ms": stats.boot_unix_ms,
        "turns": stats.turns
    }))
}
