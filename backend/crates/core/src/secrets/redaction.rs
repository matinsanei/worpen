use serde_json::Value;
use std::collections::HashSet;
use std::sync::RwLock;

/// Automatic multi-representation redaction engine that masks sensitive secret values
/// across plain text, Base64, Hex, and URL-encoded variants.
#[derive(Default)]
pub struct RedactionEngine {
    known_secrets: RwLock<HashSet<String>>,
}

fn percent_encode_upper(s: &str) -> String {
    let mut res = String::new();
    for b in s.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                res.push(b as char);
            }
            _ => {
                res.push_str(&format!("%{:02X}", b));
            }
        }
    }
    res
}

impl RedactionEngine {
    pub fn new() -> Self {
        Self {
            known_secrets: RwLock::new(HashSet::new()),
        }
    }

    /// Register a known secret value across multiple representations (raw, Base64, Hex, URL-encoded)
    pub fn register(&self, secret: &str) {
        let trimmed = secret.trim();
        if trimmed.len() < 2 {
            return;
        }

        let mut variants = Vec::new();

        // 1. Raw exact secret
        variants.push(trimmed.to_string());

        // 2. Base64 encoded (padded & unpadded)
        let b64 = crate::helpers::base64_encode(trimmed);
        if b64.len() >= 3 && b64 != trimmed {
            let unpadded = b64.trim_end_matches('=').to_string();
            if unpadded.len() >= 3 {
                variants.push(unpadded);
            }
            variants.push(b64);
        }

        // 3. Hexadecimal encoded (lowercase and uppercase)
        let hex_lower = hex::encode(trimmed.as_bytes());
        if hex_lower.len() >= 4 && hex_lower != trimmed {
            let hex_upper = hex_lower.to_uppercase();
            variants.push(hex_upper);
            variants.push(hex_lower);
        }

        // 4. URL percent-encoded (uppercase and lowercase)
        let url_upper = percent_encode_upper(trimmed);
        if url_upper != trimmed {
            let url_lower = url_upper.to_lowercase();
            variants.push(url_lower);
            variants.push(url_upper);
        }

        let mut set = self.known_secrets.write().unwrap();
        for v in variants {
            set.insert(v);
        }
    }

    /// Redact all registered secret representations from a string (longest first)
    pub fn redact_str(&self, text: &str) -> String {
        let set = self.known_secrets.read().unwrap();
        if set.is_empty() {
            return text.to_string();
        }

        // Sort descending by length so longer encoded forms get replaced before sub-strings
        let mut sorted_secrets: Vec<&String> = set.iter().collect();
        sorted_secrets.sort_by_key(|a| std::cmp::Reverse(a.len()));

        let mut redacted = text.to_string();
        for secret in sorted_secrets {
            if redacted.contains(secret.as_str()) {
                redacted = redacted.replace(secret.as_str(), "[REDACTED]");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_encoding_redaction() {
        let engine = RedactionEngine::new();
        let api_key = "sk_live_super$secret@token:123";
        engine.register(api_key);

        // 1. Raw reflection
        assert_eq!(
            engine.redact_str("Error connecting with sk_live_super$secret@token:123 to server"),
            "Error connecting with [REDACTED] to server"
        );

        // 2. Base64 reflection
        let b64 = crate::helpers::base64_encode(api_key);
        assert_eq!(
            engine.redact_str(&format!("Authorization payload: {}", b64)),
            "Authorization payload: [REDACTED]"
        );

        // 3. Hex reflection
        let hex_val = hex::encode(api_key.as_bytes());
        assert_eq!(
            engine.redact_str(&format!("Memory dump buffer: 0x{}", hex_val)),
            "Memory dump buffer: 0x[REDACTED]"
        );

        // 4. URL encoded reflection
        let url_encoded = percent_encode_upper(api_key);
        assert_eq!(
            engine.redact_str(&format!("Redirect query param: token={}", url_encoded)),
            "Redirect query param: token=[REDACTED]"
        );
    }
}
