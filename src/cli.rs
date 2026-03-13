use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "nanoaios")]
#[command(about = "AIOS 原生最小内核（Linux 风格）", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 初始化 ~/.nanoaios/config.toml
    Init {
        /// 已存在配置时强制覆盖
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    /// 启动 API 内核
    Start {
        /// 指定配置路径（默认 ~/.nanoaios/config.toml）
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// 单轮对话测试（走 Runtime 抽象）
    Chat {
        /// 用户输入
        prompt: String,
        /// 指定配置路径（默认 ~/.nanoaios/config.toml）
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// 打印当前配置
    Config {
        /// 指定配置路径（默认 ~/.nanoaios/config.toml）
        #[arg(long)]
        config: Option<PathBuf>,
    },
}
