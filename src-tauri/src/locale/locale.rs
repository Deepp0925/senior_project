use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub trait LocaleInfo {
    fn code(&self) -> &str;
    fn params(&self) -> &Value;
    fn to_json(&self) -> Value {
        json!({
            "code": self.code(),
            "params": self.params(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locale {
    code: String,
    params: Value,
}

impl Locale {
    pub fn new(code: String, params: Value) -> Self {
        Self { code, params }
    }
}

impl LocaleInfo for Locale {
    fn code(&self) -> &str {
        &self.code
    }

    fn params(&self) -> &Value {
        &self.params
    }
}
