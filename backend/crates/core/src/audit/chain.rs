use chrono::Utc;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

type HmacSha256 = Hmac<Sha256>;

/// Resolve the HMAC audit secret key from environment or fallback to machine anchor
pub fn get_audit_secret_key() -> Vec<u8> {
    if let Ok(key) = std::env::var("WORPEN_AUDIT_KEY") {
        let trimmed = key.trim();
        if !trimmed.is_empty() {
            return trimmed.as_bytes().to_vec();
        }
    }
    b"worpen-system-audit-anchor-hmac-v1".to_vec()
}

/// Compute HMAC-SHA256 signature over entry hash using secret audit key
pub fn compute_hmac_signature(key: &[u8], entry_hash: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any length");
    mac.update(entry_hash.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Cryptographically chained tamper-evident audit record with HMAC-SHA256 anchor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditRecord {
    pub index: u64,
    pub timestamp: String,
    pub prev_hash: String,
    pub action: String,
    pub actor: String,
    pub route_or_workflow: String,
    pub details: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_decision: Option<String>,
    pub entry_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub valid: bool,
    pub total_entries: u64,
    pub genesis_hash: String,
    pub latest_hash: String,
    pub verified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainVerificationError {
    pub broken_index: u64,
    pub reason: String,
    pub expected_hash: Option<String>,
    pub actual_hash: Option<String>,
}

impl std::fmt::Display for ChainVerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Audit chain verification failed at index {}: {}",
            self.broken_index, self.reason
        )
    }
}

impl std::error::Error for ChainVerificationError {}

/// Compute SHA-256 entry hash over canonical record fields
#[allow(clippy::too_many_arguments)]
pub fn compute_entry_hash(
    index: u64,
    timestamp: &str,
    prev_hash: &str,
    action: &str,
    actor: &str,
    route_or_workflow: &str,
    details: &Value,
    policy_decision: Option<&str>,
) -> String {
    let mut hasher = Sha256::new();
    let details_canonical = serde_json::to_string(details).unwrap_or_default();
    let payload = format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        index,
        timestamp,
        prev_hash,
        action,
        actor,
        route_or_workflow,
        details_canonical,
        policy_decision.unwrap_or("")
    );
    hasher.update(payload.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verify cryptographic integrity of an audit chain JSONL string or reader using active audit key
pub fn verify_audit_chain(content: &str) -> Result<VerificationReport, ChainVerificationError> {
    let key = get_audit_secret_key();
    verify_audit_chain_with_key(content, &key)
}

/// Verify cryptographic integrity of an audit chain with an explicit HMAC anchor key
pub fn verify_audit_chain_with_key(
    content: &str,
    key: &[u8],
) -> Result<VerificationReport, ChainVerificationError> {
    if content.trim().is_empty() {
        return Ok(VerificationReport {
            valid: true,
            total_entries: 0,
            genesis_hash: GENESIS_HASH.to_string(),
            latest_hash: GENESIS_HASH.to_string(),
            verified_at: Utc::now().to_rfc3339(),
        });
    }

    let mut expected_index: u64 = 0;
    let mut expected_prev_hash = GENESIS_HASH.to_string();
    let mut genesis_hash = String::new();
    let mut latest_hash = GENESIS_HASH.to_string();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let record: AuditRecord =
            serde_json::from_str(trimmed).map_err(|e| ChainVerificationError {
                broken_index: expected_index,
                reason: format!("JSON parse error on line {}: {}", line_num + 1, e),
                expected_hash: None,
                actual_hash: None,
            })?;

        // 1. Verify index sequence
        if record.index != expected_index {
            return Err(ChainVerificationError {
                broken_index: record.index,
                reason: format!(
                    "Index discontinuity: expected {} but got {}",
                    expected_index, record.index
                ),
                expected_hash: None,
                actual_hash: None,
            });
        }

        // 2. Verify prev_hash matches prior entry_hash
        if record.prev_hash != expected_prev_hash {
            return Err(ChainVerificationError {
                broken_index: record.index,
                reason: format!(
                    "Broken hash chain: expected prev_hash '{}' but got '{}'",
                    expected_prev_hash, record.prev_hash
                ),
                expected_hash: Some(expected_prev_hash),
                actual_hash: Some(record.prev_hash),
            });
        }

        // 3. Recompute and verify entry_hash
        let computed = compute_entry_hash(
            record.index,
            &record.timestamp,
            &record.prev_hash,
            &record.action,
            &record.actor,
            &record.route_or_workflow,
            &record.details,
            record.policy_decision.as_deref(),
        );

        if computed != record.entry_hash {
            return Err(ChainVerificationError {
                broken_index: record.index,
                reason: "Tampered entry hash mismatch".to_string(),
                expected_hash: Some(computed),
                actual_hash: Some(record.entry_hash),
            });
        }

        // 4. Verify HMAC-SHA256 signature if present
        if let Some(ref sig) = record.hmac_signature {
            let expected_sig = compute_hmac_signature(key, &record.entry_hash);
            if sig != &expected_sig {
                return Err(ChainVerificationError {
                    broken_index: record.index,
                    reason: "HMAC signature mismatch: audit record was modified or recalculated without secret audit key".to_string(),
                    expected_hash: Some(expected_sig),
                    actual_hash: Some(sig.clone()),
                });
            }
        }

        if expected_index == 0 {
            genesis_hash = record.entry_hash.clone();
        }
        latest_hash = record.entry_hash.clone();
        expected_prev_hash = record.entry_hash;
        expected_index += 1;
    }

    Ok(VerificationReport {
        valid: true,
        total_entries: expected_index,
        genesis_hash: if genesis_hash.is_empty() {
            GENESIS_HASH.to_string()
        } else {
            genesis_hash
        },
        latest_hash,
        verified_at: Utc::now().to_rfc3339(),
    })
}

/// Thread-safe manager for append-only audit chaining
pub struct AuditChainManager {
    file_path: PathBuf,
    state: RwLock<(u64, String)>, // (next_index, last_hash)
}

impl AuditChainManager {
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Self {
        let file_path = data_dir.as_ref().join("audit_chain.jsonl");
        let (next_index, last_hash) = Self::init_state(&file_path);

        Self {
            file_path,
            state: RwLock::new((next_index, last_hash)),
        }
    }

    fn init_state(path: &Path) -> (u64, String) {
        if !path.exists() {
            return (0, GENESIS_HASH.to_string());
        }

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return (0, GENESIS_HASH.to_string()),
        };

        let reader = BufReader::new(file);
        let mut last_rec: Option<AuditRecord> = None;

        for line in reader.lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Ok(rec) = serde_json::from_str::<AuditRecord>(trimmed) {
                    last_rec = Some(rec);
                }
            }
        }

        match last_rec {
            Some(rec) => (rec.index + 1, rec.entry_hash),
            None => (0, GENESIS_HASH.to_string()),
        }
    }

    /// Append a new audited event to the chain
    pub fn append(
        &self,
        action: &str,
        actor: &str,
        route_or_workflow: &str,
        details: Value,
        policy_decision: Option<&str>,
    ) -> Result<AuditRecord, String> {
        let mut state = self.state.write().unwrap();
        let (index, ref last_hash) = *state;
        let timestamp = Utc::now().to_rfc3339();

        let entry_hash = compute_entry_hash(
            index,
            &timestamp,
            last_hash,
            action,
            actor,
            route_or_workflow,
            &details,
            policy_decision,
        );

        let key = get_audit_secret_key();
        let hmac_signature = compute_hmac_signature(&key, &entry_hash);

        let record = AuditRecord {
            index,
            timestamp,
            prev_hash: last_hash.clone(),
            action: action.to_string(),
            actor: actor.to_string(),
            route_or_workflow: route_or_workflow.to_string(),
            details,
            policy_decision: policy_decision.map(|s| s.to_string()),
            entry_hash: entry_hash.clone(),
            hmac_signature: Some(hmac_signature),
        };

        let line =
            serde_json::to_string(&record).map_err(|e| format!("Serialization error: {}", e))?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| format!("Failed to open audit chain file: {}", e))?;

        writeln!(file, "{}", line).map_err(|e| format!("Failed to write audit line: {}", e))?;

        *state = (index + 1, entry_hash);
        Ok(record)
    }

    /// Read and verify the entire on-disk audit chain
    pub fn verify(&self) -> Result<VerificationReport, ChainVerificationError> {
        if !self.file_path.exists() {
            return Ok(VerificationReport {
                valid: true,
                total_entries: 0,
                genesis_hash: GENESIS_HASH.to_string(),
                latest_hash: GENESIS_HASH.to_string(),
                verified_at: Utc::now().to_rfc3339(),
            });
        }

        let content =
            std::fs::read_to_string(&self.file_path).map_err(|e| ChainVerificationError {
                broken_index: 0,
                reason: format!("Failed to read audit log file: {}", e),
                expected_hash: None,
                actual_hash: None,
            })?;

        verify_audit_chain(&content)
    }

    /// Read all audit records
    pub fn read_all(&self) -> Result<Vec<AuditRecord>, String> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }

        let file =
            File::open(&self.file_path).map_err(|e| format!("Failed to open audit file: {}", e))?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();

        for line in reader.lines() {
            let l = line.map_err(|e| format!("Read line failed: {}", e))?;
            let trimmed = l.trim();
            if !trimmed.is_empty() {
                let rec = serde_json::from_str::<AuditRecord>(trimmed)
                    .map_err(|e| format!("Parse audit record failed: {}", e))?;
                records.push(rec);
            }
        }

        Ok(records)
    }
}
