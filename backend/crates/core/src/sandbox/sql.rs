use proto::models::SandboxPolicy;
use regex::Regex;

/// Validate SQL query against policy guards: read-only restrictions and table allowlists.
pub fn validate_sql_policy(query: &str, policy: &SandboxPolicy) -> Result<(), String> {
    if !policy.allow_sql {
        return Err("sandbox: SQL execution denied (allow_sql=false)".to_string());
    }

    let normalized = query.trim();
    let upper = normalized.to_uppercase();

    // 1. Read-only SQL enforcement
    if policy.read_only_sql {
        let write_keywords = [
            "INSERT", "UPDATE", "DELETE", "DROP", "ALTER", "CREATE", "TRUNCATE", "REPLACE",
            "ATTACH", "PRAGMA", "GRANT", "REVOKE",
        ];

        for kw in write_keywords {
            // Check start of query or statement boundary after semicolon
            if upper.starts_with(kw)
                || upper.contains(&format!("; {}", kw))
                || upper.contains(&format!(";\n{}", kw))
                || upper.contains(&format!(";{}", kw))
            {
                return Err(format!(
                    "sandbox: SQL write operation '{}' denied (read_only_sql=true)",
                    kw
                ));
            }
        }
    }

    // 2. Table allowlist enforcement
    if !policy.allowed_tables.is_empty() {
        let tables = extract_table_names(normalized);
        let allowed_lower: Vec<String> = policy
            .allowed_tables
            .iter()
            .map(|t| t.to_lowercase())
            .collect();

        for table in tables {
            let table_clean = table.trim_matches(|c| c == '"' || c == '`' || c == '\'');
            if !allowed_lower.contains(&table_clean.to_lowercase()) {
                return Err(format!(
                    "sandbox: SQL access to table '{}' denied by allowed_tables",
                    table_clean
                ));
            }
        }
    }

    Ok(())
}

/// Extract table names from SQL clauses (FROM, JOIN, INTO, UPDATE)
fn extract_table_names(query: &str) -> Vec<String> {
    let mut tables = Vec::new();

    // Match keywords followed by an identifier
    let re = Regex::new(r"(?i)\b(?:FROM|JOIN|INTO|UPDATE)\s+([a-zA-Z0-9_`\x22]+)").unwrap();

    for cap in re.captures_iter(query) {
        if let Some(matched) = cap.get(1) {
            let tbl = matched.as_str().trim();
            if !tbl.is_empty() && !tables.contains(&tbl.to_string()) {
                tables.push(tbl.to_string());
            }
        }
    }

    tables
}
