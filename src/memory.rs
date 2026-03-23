use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub unix_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMemory {
    pub session_id: String,
    pub created_unix_ms: u128,
    pub updated_unix_ms: u128,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone)]
pub struct MemoryStore {
    root_dir: PathBuf,
}

impl MemoryStore {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub fn load_session(&self, session_id: &str) -> Result<Option<SessionMemory>> {
        let path = self.session_path(session_id)?;
        if !path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read session memory: {}", path.display()))?;
        let session = serde_json::from_str::<SessionMemory>(&raw)
            .with_context(|| format!("failed to parse session memory: {}", path.display()))?;
        Ok(Some(session))
    }

    pub fn append_turn(
        &self,
        session_id: &str,
        prompt: &str,
        answer: &str,
        max_messages: usize,
    ) -> Result<SessionMemory> {
        self.ensure_root_dir()?;
        let now = now_unix_ms();
        let mut session = match self.load_session(session_id)? {
            Some(existing) => existing,
            None => SessionMemory {
                session_id: session_id.to_string(),
                created_unix_ms: now,
                updated_unix_ms: now,
                messages: Vec::new(),
            },
        };

        session.messages.push(ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
            unix_ms: now,
        });
        session.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: answer.to_string(),
            unix_ms: now,
        });
        session.updated_unix_ms = now;

        if max_messages > 0 && session.messages.len() > max_messages {
            let remove_count = session.messages.len() - max_messages;
            session.messages.drain(0..remove_count);
        }

        self.save_session(&session)?;
        Ok(session)
    }

    fn save_session(&self, session: &SessionMemory) -> Result<()> {
        self.ensure_root_dir()?;
        let path = self.session_path(&session.session_id)?;
        let raw =
            serde_json::to_string_pretty(session).context("failed to serialize session memory")?;
        fs::write(&path, raw)
            .with_context(|| format!("failed to write session memory: {}", path.display()))?;
        Ok(())
    }

    fn session_path(&self, session_id: &str) -> Result<PathBuf> {
        let normalized = normalize_session_id(session_id)?;
        Ok(self.root_dir.join(format!("{normalized}.json")))
    }

    fn ensure_root_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.root_dir).with_context(|| {
            format!(
                "failed to create session directory: {}",
                self.root_dir.display()
            )
        })?;
        Ok(())
    }
}

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn normalize_session_id(session_id: &str) -> Result<String> {
    if session_id.is_empty() {
        bail!("session_id cannot be empty");
    }
    if session_id.len() > 128 {
        bail!("session_id length must be <= 128");
    }
    if !session_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(anyhow!(
            "session_id supports only letters, numbers, hyphen, and underscore"
        ));
    }
    Ok(session_id.to_string())
}

pub fn default_memory_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("sessions")
}

#[cfg(test)]
mod tests {
    use super::normalize_session_id;

    #[test]
    fn session_id_validation() {
        assert!(normalize_session_id("abc_123-xy").is_ok());
        assert!(normalize_session_id("").is_err());
        assert!(normalize_session_id("../bad").is_err());
        assert!(normalize_session_id("non_ascii").is_ok());
        assert!(normalize_session_id("\u{4F60}\u{597D}").is_err());
        assert!(normalize_session_id("bad/id").is_err());
    }
}
