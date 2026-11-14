use proto::models::SandboxPolicy;
use regex::Regex;

const FORBIDDEN_WRITE_KEYWORDS: &[&str] = &[
    "INSERT", "UPDATE", "DELETE", "DROP", "ALTER", "CREATE", "TRUNCATE", "REPLACE", "ATTACH",
    "DETACH", "PRAGMA", "GRANT", "REVOKE", "VACUUM", "REINDEX",
];

/// Strip string literals, block comments, and line comments from SQL to expose raw tokens.
pub fn sanitize_sql(query: &str) -> String {
    // 1. Strip string literals ('...' and \' escape sequences)
    let no_strings = Regex::new(r"'(?:[^'\\]|\\.)*'")
        .unwrap()
        .replace_all(query, " ");
    // 2. Strip block comments /* ... */
    let no_block_comments = Regex::new(r"/\*[\s\S]*?\*/")
        .unwrap()
        .replace_all(&no_strings, " ");
    // 3. Strip line comments -- ...
    let no_line_comments = Regex::new(r"--[^\r\n]*")
        .unwrap()
        .replace_all(&no_block_comments, " ");
    no_line_comments.to_string()
}

/// Validate SQL query against policy guards: read-only restrictions and table allowlists.
pub fn validate_sql_policy(query: &str, policy: &SandboxPolicy) -> Result<(), String> {
    if !policy.allow_sql {
        return Err("sandbox: SQL execution denied (allow_sql=false)".to_string());
    }

    let sanitized = sanitize_sql(query);

    // 1. Read-only SQL enforcement across the ENTIRE token stream (prevents CTEs, subqueries, comments)
    if policy.read_only_sql {
        let token_re = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
        for mat in token_re.find_iter(&sanitized) {
            let token_upper = mat.as_str().to_uppercase();
            if FORBIDDEN_WRITE_KEYWORDS.contains(&token_upper.as_str()) {
                return Err(format!(
                    "sandbox: SQL write operation '{}' denied (read_only_sql=true)",
                    token_upper
                ));
            }
        }
    }

    // 2. Table allowlist enforcement (supports standard FROM, comma joins, JOIN, INTO, UPDATE)
    if !policy.allowed_tables.is_empty() {
        let tables = extract_table_names(&sanitized);
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

/// Extract table names from SQL clauses, handling comma joins, JOINs, INTO, and UPDATE.
pub fn extract_table_names(sanitized_query: &str) -> Vec<String> {
    let mut tables = Vec::new();

    // 1. Extract comma-separated tables after FROM: FROM t1, t2, t3 ...
    let from_re = Regex::new(r"(?i)\bFROM\s+([^;]+)").unwrap();
    let clause_delim_re = Regex::new(
        r"(?i)\b(?:WHERE|GROUP|HAVING|ORDER|LIMIT|JOIN|LEFT|RIGHT|INNER|OUTER|CROSS|NATURAL|UNION|WINDOW)\b",
    )
    .unwrap();

    for cap in from_re.captures_iter(sanitized_query) {
        if let Some(matched) = cap.get(1) {
            let clause = matched.as_str();
            let clause_clean = clause_delim_re.split(clause).next().unwrap_or(clause);

            for part in clause_clean.split(',') {
                let trimmed = part.trim();
                if let Some(first_word) = trimmed.split_whitespace().next() {
                    let clean =
                        first_word.trim_matches(|c| c == '"' || c == '`' || c == '\'' || c == ';');
                    if !clean.is_empty()
                        && !tables.contains(&clean.to_string())
                        && !clean.starts_with('(')
                    {
                        tables.push(clean.to_string());
                    }
                }
            }
        }
    }

    // 2. Extract tables after JOIN, INTO, UPDATE
    let other_re = Regex::new(r"(?i)\b(?:JOIN|INTO|UPDATE)\s+([a-zA-Z0-9_`\x22]+)").unwrap();
    for cap in other_re.captures_iter(sanitized_query) {
        if let Some(matched) = cap.get(1) {
            let tbl = matched
                .as_str()
                .trim_matches(|c| c == '"' || c == '`' || c == '\'');
            if !tbl.is_empty() && !tables.contains(&tbl.to_string()) {
                tables.push(tbl.to_string());
            }
        }
    }

    tables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cte_and_comment_write_bypass_blocked() {
        let policy = SandboxPolicy {
            allow_sql: true,
            read_only_sql: true,
            ..Default::default()
        };

        // CTE write trick
        let cte_query = "WITH x AS (SELECT 1) DELETE FROM users";
        assert!(validate_sql_policy(cte_query, &policy).is_err());

        // Comment prefix trick
        let comment_query = "/* bypass */ DROP TABLE users";
        assert!(validate_sql_policy(comment_query, &policy).is_err());

        // Line comment trick
        let line_comment = "-- comment\nTRUNCATE orders";
        assert!(validate_sql_policy(line_comment, &policy).is_err());

        // Valid read-only query
        let valid_select = "SELECT id, name FROM users WHERE age > 18";
        assert!(validate_sql_policy(valid_select, &policy).is_ok());

        // String literal containing write keyword should NOT be falsely blocked
        let status_query = "SELECT id FROM tasks WHERE status = 'DELETE'";
        assert!(validate_sql_policy(status_query, &policy).is_ok());
    }

    #[test]
    fn test_comma_join_table_allowlist_bypass_blocked() {
        let policy = SandboxPolicy {
            allow_sql: true,
            read_only_sql: true,
            allowed_tables: vec!["allowed".to_string()],
            ..Default::default()
        };

        // Comma join trying to read secret_table
        let comma_join = "SELECT * FROM allowed, secret_table WHERE allowed.id = secret_table.id";
        let res = validate_sql_policy(comma_join, &policy);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("secret_table"));

        // Single allowed table passes
        let ok_query = "SELECT * FROM allowed WHERE id = 1";
        assert!(validate_sql_policy(ok_query, &policy).is_ok());
    }
}
