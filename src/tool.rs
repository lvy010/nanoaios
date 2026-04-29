use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolKind {
    Http,
    Mcp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolManifest {
    pub name: String,
    pub description: String,
    pub kind: ToolKind,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Debug, Clone)]
pub struct ToolRegistry {
    tools_dir: PathBuf,
    tools: HashMap<String, ToolManifest>,
    client: Client,
}

impl ToolRegistry {
    pub fn new(tools_dir: PathBuf) -> Self {
        Self {
            tools_dir,
            tools: HashMap::new(),
            client: Client::new(),
        }
    }

    pub fn load_all(&mut self) -> Result<()> {
        self.tools.clear();
        if !self.tools_dir.exists() {
            return Ok(());
        }
        let entries = fs::read_dir(&self.tools_dir).with_context(|| {
            format!("failed to read tools dir: {}", self.tools_dir.display())
        })?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "toml") {
                match self.load_manifest(&path) {
                    Ok(m) => { self.tools.insert(m.name.clone(), m); }
                    Err(e) => eprintln!("warn: skip {}: {e}", path.display()),
                }
            }
        }
        Ok(())
    }

    fn load_manifest(&self, path: &Path) -> Result<ToolManifest> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read: {}", path.display()))?;
        let manifest: ToolManifest = toml::from_str(&raw)
            .with_context(|| format!("failed to parse: {}", path.display()))?;
        Ok(manifest)
    }

    pub fn add(&mut self, manifest_path: &Path) -> Result<ToolManifest> {
        let manifest = self.load_manifest(manifest_path)?;
        fs::create_dir_all(&self.tools_dir).with_context(|| {
            format!("failed to create tools dir: {}", self.tools_dir.display())
        })?;
        let dest = self.tools_dir.join(format!("{}.toml", manifest.name));
        fs::copy(manifest_path, &dest).with_context(|| {
            format!("failed to copy manifest to {}", dest.display())
        })?;
        self.tools.insert(manifest.name.clone(), manifest.clone());
        Ok(manifest)
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        if self.tools.remove(name).is_none() {
            bail!("tool not found: {name}");
        }
        let path = self.tools_dir.join(format!("{name}.toml"));
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("failed to remove: {}", path.display()))?;
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<&ToolManifest> {
        let mut tools: Vec<_> = self.tools.values().collect();
        tools.sort_by_key(|t| &t.name);
        tools
    }

    pub fn get(&self, name: &str) -> Option<&ToolManifest> {
        self.tools.get(name)
    }

    pub async fn invoke(
        &self,
        name: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let tool = self.tools.get(name)
            .ok_or_else(|| anyhow::anyhow!("tool not found: {name}"))?;
        match tool.kind {
            ToolKind::Http => self.invoke_http(tool, params).await,
            ToolKind::Mcp => self.invoke_mcp(tool, params).await,
        }
    }

    async fn invoke_http(
        &self,
        tool: &ToolManifest,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let base = tool.base_url.as_deref()
            .ok_or_else(|| anyhow::anyhow!("http tool '{}' missing base_url", tool.name))?;
        let path = tool.path.as_deref().unwrap_or("");
        let url = format!("{}{}", base.trim_end_matches('/'), path);
        let method = tool.method.as_deref().unwrap_or("POST");
        let timeout = std::time::Duration::from_secs(tool.timeout_secs);

        let resp = match method.to_uppercase().as_str() {
            "GET" => {
                let query: Vec<(String, String)> = params
                    .as_object()
                    .map(|m| {
                        m.iter()
                            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                self.client.get(&url).query(&query).timeout(timeout)
                    .send().await.context("http tool request failed")?
            }
            _ => {
                self.client.post(&url).json(params).timeout(timeout)
                    .send().await.context("http tool request failed")?
            }
        };

        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("tool '{}' returned {code}: {body}", tool.name);
        }

        let value: serde_json::Value = resp.json().await
            .context("failed to parse tool response as JSON")?;
        Ok(value)
    }

    async fn invoke_mcp(
        &self,
        tool: &ToolManifest,
        _params: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        bail!(
            "MCP tool '{}' invocation not yet implemented (planned for v0.3)",
            tool.name
        )
    }
}

pub fn default_tools_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("tools")
}
