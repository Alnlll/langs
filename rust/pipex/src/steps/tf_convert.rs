use super::traits::Step;
use crate::core::context::Context;
use anyhow::Result;
use async_trait::async_trait;
use tracing::info;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TfConvertConfig {
    pub model_path: String,
    pub output_format: String,
}

pub struct TfConvertStep {
    config: TfConvertConfig,
}

impl TfConvertStep {
    pub fn new(config: TfConvertConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Step for TfConvertStep {
    fn name(&self) -> String {
        "tf_convert".to_string()
    }

    async fn execute(&self, ctx: &Context) -> Result<()> {
        info!("Starting TF Model Conversion: {:?}", self.config);
        
        // 模拟耗时操作
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        
        // 假设转换产生了一个新文件，写入 Context 供下一步使用
        let output_file = format!("{}.{}", self.config.model_path, self.config.output_format);
        ctx.set("converted_model_path", output_file.clone())?;
        
        info!("TF Model Conversion completed. Output: {}", output_file);
        Ok(())
    }
}
