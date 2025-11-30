use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::{Result, anyhow};
use serde_json::Value;

// 线程安全的上下文
#[derive(Debug, Default, Clone)]
pub struct Context {
    data: Arc<Mutex<HashMap<String, Value>>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T: serde::Serialize>(&self, key: &str, value: T) -> Result<()> {
        let mut map = self.data.lock().map_err(|_| anyhow!("Context lock poisoned"))?;
        let json_val = serde_json::to_value(value)?;
        map.insert(key.to_string(), json_val);
        Ok(())
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T> {
        let map = self.data.lock().map_err(|_| anyhow!("Context lock poisoned"))?;
        if let Some(val) = map.get(key) {
            let t: T = serde_json::from_value(val.clone())?;
            Ok(t)
        } else {
            Err(anyhow!("Key '{}' not found in context", key))
        }
    }
}