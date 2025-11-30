mod config;
mod core;
mod steps;

use crate::config::AppConfig;
use crate::core::engine::PipelineEngine;
use anyhow::Result;
use std::fs::File;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 初始化日志
    tracing_subscriber::fmt::init();

    // 2. 加载配置
    let file = File::open("config/config.yaml")?;
    let config: AppConfig = serde_yaml::from_reader(file)?;

    // 3. 构建引擎
    let engine = PipelineEngine::from_config(config)?;

    // 4. 执行
    // 可以通过命令行参数传入 start_from
    let start_from = std::env::args().nth(1); 
    
    engine.run(start_from).await?;

    Ok(())
}
