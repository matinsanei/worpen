use super::provider::{
    EnvSecretSource, FileSecretSource, KeychainSecretSource, MemorySecretSource, SecretSource,
};
use super::redaction::RedactionEngine;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;

pub struct SecretStore {
    sources: Vec<Box<dyn SecretSource>>,
    memory_source: Arc<MemorySecretSource>,
    redaction: Arc<RedactionEngine>,
}

impl Default for SecretStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretStore {
    pub fn new() -> Self {
        let memory = Arc::new(MemorySecretSource::new());
        let redaction = Arc::new(RedactionEngine::new());

        // Pre-register all existing WORPEN_SECRET_* env values in the redaction engine
        for (k, v) in std::env::vars() {
            if k.starts_with("WORPEN_SECRET_") && !v.trim().is_empty() {
                redaction.register(&v);
            }
        }

        let mut sources: Vec<Box<dyn SecretSource>> = Vec::new();

        // 1. In-memory source (highest priority)
        sources.push(Box::new(MemoryWrapper(memory.clone())));

        // 2. Default secrets directory if exists
        let secrets_dir = std::env::var("SECRETS_DIR").unwrap_or_else(|_| "secrets".to_string());
        if Path::new(&secrets_dir).exists() {
            sources.push(Box::new(FileSecretSource::new(secrets_dir)));
        }

        // 3. OS keychain source
        sources.push(Box::new(KeychainSecretSource::new("worpen")));

        // 4. Environment variables source
        sources.push(Box::new(EnvSecretSource));

        Self {
            sources,
            memory_source: memory,
            redaction,
        }
    }

    /// Add a custom secret source
    pub fn add_source(&mut self, source: Box<dyn SecretSource>) {
        self.sources.insert(0, source);
    }

    /// In-memory secret setter (convenient for testing and dynamic credentials)
    pub fn set_secret(&self, name: &str, value: &str) {
        self.memory_source.set(name, value);
        self.redaction.register(value);
    }

    /// Access the shared RedactionEngine
    pub fn redaction(&self) -> Arc<RedactionEngine> {
        self.redaction.clone()
    }

    /// Resolve a single secret by name
    pub fn resolve_secret(&self, name: &str) -> Result<String, String> {
        for source in &self.sources {
            if let Some(val) = source.get(name) {
                self.redaction.register(&val);
                return Ok(val);
            }
        }
        Err(format!(
            "Secret '{}' not found in any configured secret source",
            name
        ))
    }

    /// Interpolate `{{secret:name}}` references in a string with actual secret values
    pub fn inject_secrets(&self, text: &str) -> Result<String, String> {
        let re = regex::Regex::new(r"\{\{\s*secret:([^}]+)\s*\}\}").unwrap();
        let mut result = text.to_string();

        for cap in re.captures_iter(text) {
            if let Some(matched) = cap.get(1) {
                let secret_name = matched.as_str().trim();
                let secret_val = self.resolve_secret(secret_name)?;
                let token = cap.get(0).unwrap().as_str();
                result = result.replace(token, &secret_val);
            }
        }
        Ok(result)
    }

    /// Recursively resolve and inject secrets into a JSON Value
    pub fn inject_secrets_into_value(&self, val: &Value) -> Result<Value, String> {
        match val {
            Value::String(s) => {
                let resolved = self.inject_secrets(s)?;
                Ok(Value::String(resolved))
            }
            Value::Array(arr) => {
                let mut new_arr = Vec::new();
                for item in arr {
                    new_arr.push(self.inject_secrets_into_value(item)?);
                }
                Ok(Value::Array(new_arr))
            }
            Value::Object(map) => {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k.clone(), self.inject_secrets_into_value(v)?);
                }
                Ok(Value::Object(new_map))
            }
            other => Ok(other.clone()),
        }
    }
}

struct MemoryWrapper(Arc<MemorySecretSource>);

impl SecretSource for MemoryWrapper {
    fn get(&self, name: &str) -> Option<String> {
        self.0.get(name)
    }
}
