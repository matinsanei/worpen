use serde_json::Value;
use std::collections::HashSet;
use std::sync::RwLock;

/// Automatic redaction engine that masks sensitive secret values across strings and JSON objects
#[derive(Default)]
pub struct RedactionEngine {
    known_secrets: RwLock<HashSet<String>>,
}

impl RedactionEngine {
    pub fn new() -> Self {
        Self {
            known_secrets: RwLock::new(HashSet::new()),
        }
    }

    /// Register a known secret value to be automatically redacted
    pub fn register(&self, secret: &str) {
        let trimmed = secret.trim();
        // Ignore trivial/short secrets to avoid redacting whitespace or common small chars
        if trimmed.len() >= 3 {
            let mut set = self.known_secrets.write().unwrap();
            set.insert(trimmed.to_string());
        }
    }

    /// Redact all known secrets from a string
    pub fn redact_str(&self, text: &str) -> String {
        let set = self.known_secrets.read().unwrap();
        if set.is_empty() {
            return text.to_string();
        }

        let mut redacted = text.to_string();
        for secret in set.iter() {
            if redacted.contains(secret) {
                redacted = redacted.replace(secret, "[REDACTED]");
            }
        }
        redacted
    }

    /// Recursively walk a JSON Value and redact all known secrets
    pub fn redact_value(&self, val: &Value) -> Value {
        match val {
            Value::String(s) => Value::String(self.redact_str(s)),
            Value::Array(arr) => {
                Value::Array(arr.iter().map(|item| self.redact_value(item)).collect())
            }
            Value::Object(map) => {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k.clone(), self.redact_value(v));
                }
                Value::Object(new_map)
            }
            other => other.clone(),
        }
    }
}
