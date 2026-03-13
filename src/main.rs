mod api;
mod cli;
mod config;
mod kernel;
mod runtime;

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use config::{init_config, load_config};
use kernel::Kernel;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { force } => {
            let path = init_config(force)?;
            println!("配置文件已就绪: {}", path.display());
        }
        Commands::Start { config } => {
            let conf = load_config(config.as_deref())?;
            let kernel = Arc::new(Kernel::new(conf));
            api::serve(kernel).await?;
        }
        Commands::Chat { prompt, config } => {
            let conf = load_config(config.as_deref())?;
            let kernel = Kernel::new(conf);
            let answer = kernel.infer(&prompt).await?;
            println!("{answer}");
        }
        Commands::Config { config } => {
            let conf = load_config(config.as_deref())?;
            let rendered = toml::to_string_pretty(&conf)?;
            print!("{rendered}");
        }
    }

    Ok(())
}
