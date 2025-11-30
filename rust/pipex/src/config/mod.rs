use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub pipeline: PipelineConfig,
}

#[derive(Deserialize, Debug)]
pub struct PipelineConfig {
    pub name: String,
    // 定义步骤列表，顺序即为默认执行顺序
    pub steps: Vec<StepConfig>,
}

#[derive(Deserialize, Debug)]
pub struct StepConfig {
    pub id: String,          // 实例ID，例如 "convert_step_1"
    pub step_type: String,   // 类型，例如 "tf_convert"
    // 具体的参数，使用 HashMap 接收任意结构，在工厂方法中解析
    pub params: Option<HashMap<String, serde_yaml::Value>>, 
}
