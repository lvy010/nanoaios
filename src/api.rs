use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::extract::Path;
use axum::extract::State;
use axum::routing::get;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::kernel::Kernel;

pub async fn serve(kernel: Arc<Kernel>) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", kernel.config.api_host, kernel.config.api_port)
        .parse()
        .context("failed to parse api_host/api_port")?;

    let app = Router::new()
        .route("/", get(index))
        .route("/healthz", get(health))
        .route("/v1/kernel/state", get(state))
        .route("/v1/kernel/memory/{session_id}", get(session_memory))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/tools", get(list_tools))
        .route("/v1/tools/{tool_name}/invoke", post(invoke_tool))
        .with_state(kernel);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind address: {addr}"))?;
    println!("nanoaios api running on http://{addr}");
    axum::serve(listener, app)
        .await
        .context("API server exited unexpectedly")?;
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
            "kernel_state": "/v1/kernel/state",
            "kernel_memory": "/v1/kernel/memory/{session_id}",
            "chat_completions": "/v1/chat/completions",
            "tools": "/v1/tools",
            "tool_invoke": "/v1/tools/{tool_name}/invoke"
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

async fn session_memory(
    Path(session_id): Path<String>,
    State(kernel): State<Arc<Kernel>>,
) -> Json<serde_json::Value> {
    match kernel.session_memory(&session_id) {
        Ok(Some(memory)) => Json(json!({"ok": true, "memory": memory})),
        Ok(None) => Json(json!({"ok": false, "error": "session not found or memory disabled"})),
        Err(err) => Json(json!({"ok": false, "error": err.to_string()})),
    }
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    prompt: String,
    session_id: Option<String>,
}

async fn chat_completions(
    State(kernel): State<Arc<Kernel>>,
    Json(payload): Json<ChatRequest>,
) -> Json<serde_json::Value> {
    match kernel
        .infer_with_session(&payload.prompt, payload.session_id.as_deref())
        .await
    {
        Ok(answer) => Json(json!({
            "ok": true,
            "answer": answer,
            "session_id": payload.session_id
        })),
        Err(err) => Json(json!({
            "ok": false,
            "error": err.to_string()
        })),
    }
}

async fn list_tools(State(kernel): State<Arc<Kernel>>) -> Json<serde_json::Value> {
    let tools = kernel.tool_list();
    let items: Vec<_> = tools
        .iter()
        .map(|t| json!({"name": t.name, "kind": t.kind, "description": t.description}))
        .collect();
    Json(json!({"ok": true, "tools": items}))
}

#[derive(Debug, Deserialize)]
struct ToolInvokeRequest {
    params: serde_json::Value,
}

async fn invoke_tool(
    Path(tool_name): Path<String>,
    State(kernel): State<Arc<Kernel>>,
    Json(payload): Json<ToolInvokeRequest>,
) -> Json<serde_json::Value> {
    match kernel.tool_invoke(&tool_name, &payload.params).await {
        Ok(result) => Json(json!({"ok": true, "result": result})),
        Err(err) => Json(json!({"ok": false, "error": err.to_string()})),
    }
}
