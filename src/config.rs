use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

const APP_DIR_NAME: &str = ".nanoaios";
const CONFIG_FILE_NAME: &str = "config.toml";
const ENV_PREFIX: &str = "NANOAIOS_";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    Mock,
    OpenaiCompatible,
}

impl ProviderKind {
    fn from_str_lossy(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "mock" => Ok(Self::Mock),
            "openai_compatible" => Ok(Self::OpenaiCompatible),
            other => Err(anyhow!("未知的 provider kind: {other}，可选: mock, openai_compatible")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key_env: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub node_name: String,
    pub api_host: String,
    pub api_port: u16,
    pub provider: ProviderConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            node_name: "nanoaios-local".to_string(),
            api_host: "127.0.0.1".to_string(),
            api_port: 4242,
            provider: ProviderConfig {
                kind: ProviderKind::Mock,
                model: "nanoaios/mock-v1".to_string(),
                base_url: Some("https://api.openai.com/v1".to_string()),
                api_key_env: Some("OPENAI_API_KEY".to_string()),
            },
        }
    }
}

fn env_var(name: &str) -> Option<String> {
    env::var(format!("{ENV_PREFIX}{name}")).ok()
}

pub fn config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("无法解析 HOME 目录"))?;
    Ok(home.join(APP_DIR_NAME))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}

pub fn init_config(force: bool) -> Result<PathBuf> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir).with_context(|| format!("创建配置目录失败: {}", dir.display()))?;

    let path = config_path()?;
    if path.exists() && !force {
        return Ok(path);
    }

    let content = toml::to_string_pretty(&AppConfig::default()).context("序列化默认配置失败")?;
    fs::write(&path, content).with_context(|| format!("写入配置失败: {}", path.display()))?;
    Ok(path)
}

pub fn load_config(path: Option<&Path>) -> Result<AppConfig> {
    let mut config = AppConfig::default();

    let toml_path = match path {
        Some(p) => p.to_path_buf(),
        None => config_path()?,
    };

    if toml_path.exists() {
        let raw = fs::read_to_string(&toml_path)
            .with_context(|| format!("读取配置失败: {}", toml_path.display()))?;
        config = toml::from_str::<AppConfig>(&raw)
            .with_context(|| format!("解析配置失败: {}", toml_path.display()))?;
    }

    if let Some(v) = env_var("NODE_NAME") {
        config.node_name = v;
    }
    if let Some(v) = env_var("API_HOST") {
        config.api_host = v;
    }
    if let Some(v) = env_var("API_PORT") {
        config.api_port = v.parse().with_context(|| "NANOAIOS_API_PORT 解析失败，需要数字")?;
    }
    if let Some(v) = env_var("PROVIDER_KIND") {
        config.provider.kind = ProviderKind::from_str_lossy(&v)?;
    }
    if let Some(v) = env_var("PROVIDER_MODEL") {
        config.provider.model = v;
    }
    if let Some(v) = env_var("PROVIDER_BASE_URL") {
        config.provider.base_url = Some(v);
    }

    Ok(config)
}