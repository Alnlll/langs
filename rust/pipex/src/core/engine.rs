use crate::config::AppConfig;
use crate::core::context::Context;
use crate::steps::traits::Step;
use crate::steps::tf_convert::{TfConvertStep, TfConvertConfig};
// 引入其他 Step...
use anyhow::{Result, anyhow};
use tracing::{info, error, warn};
use std::collections::HashMap;

pub struct PipelineEngine {
    steps: Vec<Box<dyn Step>>,
    context: Context,
}

impl PipelineEngine {
    // 工厂方法：根据配置构建 Pipeline
    pub fn from_config(config: AppConfig) -> Result<Self> {
        let mut steps: Vec<Box<dyn Step>> = Vec::new();

        for step_conf in config.pipeline.steps {
            let step: Box<dyn Step> = match step_conf.step_type.as_str() {
                "tf_convert" => {
                    // 将 yaml value 转回 json value 再转为具体 Config 结构
                    // 这里简化处理，实际可能需要更健壮的转换
                    let params = step_conf.params.unwrap_or_default();
                    let params_json = serde_json::to_value(params)?;
                    let cfg: TfConvertConfig = serde_json::from_value(params_json)?;
                    Box::new(TfConvertStep::new(cfg))
                }
                // 在这里添加更多的 Step 类型匹配
                // "device_exec" => ...
                unknown => return Err(anyhow!("Unknown step type: {}", unknown)),
            };
            steps.push(step);
        }

        Ok(Self {
            steps,
            context: Context::new(),
        })
    }

    // 执行逻辑
    pub async fn run(&self, start_from: Option<String>) -> Result<()> {
        let mut should_run = start_from.is_none();
        
        info!("Starting Pipeline Execution...");

        for step in &self.steps {
            let step_name = step.name(); // 这里实际应该用配置里的 id 来区分同类型的不同步骤

            // 检查是否到达指定的起始点
            if !should_run {
                // 这里为了演示简单，假设 step.name() 返回的是配置里的 id 或者 type
                // 实际建议在 Step trait 中增加 id() 方法
                if let Some(ref start_id) = start_from {
                     // 注意：这里需要 Step trait 有个 id() 方法对应配置文件的 id
                     // 暂时用 type name 模拟，实际请修改 Trait
                    if step.name() == *start_id { 
                        should_run = true;
                        info!("Skipping to start point: {}", start_id);
                    } else {
                        info!("Skipping step: {}", step_name);
                        continue;
                    }
                }
            }

            if should_run {
                info!(">>> Executing Step: {}", step_name);
                
                // 1. 验证
                if let Err(e) = step.validate(&self.context) {
                    error!("Validation failed for step {}: {}", step_name, e);
                    return Err(e);
                }

                // 2. 执行
                match step.execute(&self.context).await {
                    Ok(_) => info!("<<< Step {} finished successfully.", step_name),
                    Err(e) => {
                        error!("!!! Step {} failed: {:?}", step_name, e);
                        // 这里可以根据策略决定是重试、忽略还是终止
                        return Err(e);
                    }
                }
            }
        }
        
        info!("Pipeline execution finished.");
        Ok(())
    }
}
