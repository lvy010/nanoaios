use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "nanoaios")]
#[command(about = "Minimal native AIOS kernel (Linux style)", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize ~/.nanoaios/config.toml
    Init {
        /// Overwrite config if it already exists
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    /// Start the kernel API
    Start {
        /// Optional config path (default: ~/.nanoaios/config.toml)
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Single-turn chat via runtime abstraction
    Chat {
        /// User prompt
        prompt: String,
        /// Session ID (writes to local Session/Memory when provided)
        #[arg(long)]
        session: Option<String>,
        /// Optional config path (default: ~/.nanoaios/config.toml)
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Show memory content for a session
    Session {
        /// Session ID
        id: String,
        /// Optional config path (default: ~/.nanoaios/config.toml)
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Print current config
    Config {
        /// Optional config path (default: ~/.nanoaios/config.toml)
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Manage registered tools
    Tool {
        #[command(subcommand)]
        action: ToolAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum ToolAction {
    /// List all registered tools
    List {
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Register a tool from a manifest file
    Add {
        /// Path to tool manifest (.toml)
        manifest: PathBuf,
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Remove a registered tool by name
    Remove {
        /// Tool name
        name: String,
        #[arg(long)]
        config: Option<PathBuf>,
    },
}
