use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Trait for retrieving secrets from an underlying storage source
pub trait SecretSource: Send + Sync {
    fn get(&self, name: &str) -> Option<String>;
}

/// Source that resolves secrets from environment variables
pub struct EnvSecretSource;

impl SecretSource for EnvSecretSource {
    fn get(&self, name: &str) -> Option<String> {
        let normalized = name.replace('-', "_");
        let candidates = [
            format!("WORPEN_SECRET_{}", normalized.to_uppercase()),
            normalized.to_uppercase(),
            name.to_string(),
        ];

        for cand in candidates {
            if let Ok(val) = std::env::var(&cand) {
                if !val.trim().is_empty() {
                    return Some(val);
                }
            }
        }
        None
    }
}

/// Source that resolves secrets from files in a directory (e.g. Kubernetes/Docker secrets)
pub struct FileSecretSource {
    dir: PathBuf,
}

impl FileSecretSource {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            dir: dir.as_ref().to_path_buf(),
        }
    }
}

impl SecretSource for FileSecretSource {
    fn get(&self, name: &str) -> Option<String> {
        let candidates = [self.dir.join(name), self.dir.join(format!("{}.txt", name))];

        for path in candidates {
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    let trimmed = content.trim().to_string();
                    if !trimmed.is_empty() {
                        return Some(trimmed);
                    }
                }
            }
        }
        None
    }
}

/// Thread-safe in-memory secret source (ideal for tests and dynamic injection)
#[derive(Default)]
pub struct MemorySecretSource {
    secrets: RwLock<HashMap<String, String>>,
}

impl MemorySecretSource {
    pub fn new() -> Self {
        Self {
            secrets: RwLock::new(HashMap::new()),
        }
    }

    pub fn set(&self, name: &str, value: &str) {
        let mut map = self.secrets.write().unwrap();
        map.insert(name.to_string(), value.to_string());
    }
}

impl SecretSource for MemorySecretSource {
    fn get(&self, name: &str) -> Option<String> {
        let map = self.secrets.read().unwrap();
        map.get(name).cloned()
    }
}

/// Source that resolves secrets from OS keychain or credential store
pub struct KeychainSecretSource {
    service: String,
}

impl KeychainSecretSource {
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
        }
    }
}

impl SecretSource for KeychainSecretSource {
    fn get(&self, name: &str) -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("security")
                .args([
                    "find-generic-password",
                    "-s",
                    &self.service,
                    "-a",
                    name,
                    "-w",
                ])
                .output()
                .ok()?;
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
        #[cfg(target_os = "linux")]
        {
            let output = std::process::Command::new("secret-tool")
                .args(["lookup", "service", &self.service, "account", name])
                .output()
                .ok()?;
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
        None
    }
}
