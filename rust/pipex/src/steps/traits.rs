use crate::core::context::Context;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Step: Send + Sync {
    // Step 的唯一标识符
    fn name(&self) -> String;

    // 初始化/验证配置
    fn validate(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }

    // 执行具体逻辑
    async fn execute(&self, ctx: &Context) -> Result<()>;
}