use std::env;

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde_json::json;

use crate::config::{ProviderConfig, ProviderKind};

pub struct Runtime {
    provider: ProviderConfig,
    client: Client,
}

impl Runtime {
    pub fn new(provider: ProviderConfig) -> Self {
        Self {
            provider,
            client: Client::new(),
        }
    }

    pub async fn complete(&self, prompt: &str) -> Result<String> {
        match self.provider.kind {
            ProviderKind::Mock => Ok(format!("mock-response: {prompt}")),
            ProviderKind::OpenaiCompatible => self.complete_openai_compatible(prompt).await,
        }
    }

    async fn complete_openai_compatible(&self, prompt: &str) -> Result<String> {
        let api_key_env = self
            .provider
            .api_key_env
            .as_deref()
            .unwrap_or("OPENAI_API_KEY");
        let api_key =
            env::var(api_key_env).with_context(|| format!("missing env var: {api_key_env}"))?;
        let base_url = self
            .provider
            .base_url
            .as_deref()
            .unwrap_or("https://api.openai.com/v1");
        let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

        let resp = self
            .client
            .post(url)
            .bearer_auth(api_key)
            .json(&json!({
                "model": self.provider.model,
                "messages": [{"role": "user", "content": prompt}],
            }))
            .send()
            .await
            .context("request to OpenAI-compatible endpoint failed")?;

        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "<empty>".to_string());
            bail!("model request failed: {code} {body}");
        }

        let value: serde_json::Value = resp
            .json()
            .await
            .context("failed to parse model response")?;
        let content = value["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("response missing choices[0].message.content"))?;
        Ok(content.to_string())
    }
}
