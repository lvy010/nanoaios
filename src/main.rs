mod api;
mod cli;
mod config;
mod kernel;
mod memory;
mod runtime;
mod tool;

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, ToolAction};
use config::{init_config, load_config};
use kernel::Kernel;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            let path = init_config(force)?;
            println!("Config is ready: {}", path.display());
        }
        Commands::Start { config } => {
            let conf = load_config(config.as_deref())?;
            let kernel = Arc::new(Kernel::new(conf)?);
            api::serve(kernel).await?;
        }
        Commands::Chat {
            prompt,
            session,
            config,
        } => {
            let conf = load_config(config.as_deref())?;
            let kernel = Kernel::new(conf)?;
            let answer = kernel
                .infer_with_session(&prompt, session.as_deref())
                .await?;
            println!("{answer}");
        }
        Commands::Session { id, config } => {
            let conf = load_config(config.as_deref())?;
            let kernel = Kernel::new(conf)?;
            let memory = kernel.session_memory(&id)?;
            let rendered = serde_json::to_string_pretty(&memory)?;
            println!("{rendered}");
        }
        Commands::Config { config } => {
            let conf = load_config(config.as_deref())?;
            let rendered = toml::to_string_pretty(&conf)?;
            print!("{rendered}");
        }
        Commands::Tool { action } => {
            match action {
                ToolAction::List { config } => {
                    let conf = load_config(config.as_deref())?;
                    let kernel = Kernel::new(conf)?;
                    let tools = kernel.tool_list();
                    if tools.is_empty() {
                        println!("No tools registered.");
                    } else {
                        for t in &tools {
                            println!("  {} ({:?}) - {}", t.name, t.kind, t.description);
                        }
                    }
                }
                ToolAction::Add { manifest, config } => {
                    let conf = load_config(config.as_deref())?;
                    let kernel = Kernel::new(conf)?;
                    let t = kernel.tool_add(&manifest)?;
                    println!("Registered tool: {} ({:?})", t.name, t.kind);
                }
                ToolAction::Remove { name, config } => {
                    let conf = load_config(config.as_deref())?;
                    let kernel = Kernel::new(conf)?;
                    kernel.tool_remove(&name)?;
                    println!("Removed tool: {name}");
                }
            }
        }
    }

    Ok(())
}
