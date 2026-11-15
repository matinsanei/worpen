use proto::models::routes::SandboxPolicy;
use proto::models::workflow::WorkflowSpecV1;
use std::net::IpAddr;
use worpen_core::sandbox::{is_blocked_ip, validate_sql_policy};

#[test]
fn test_adversarial_ssrf_obfuscated_and_special_ips() {
    // 1. Loopback variations
    let loopbacks: Vec<&str> = vec![
        "127.0.0.1",
        "127.0.0.2",
        "127.255.255.254",
        "0.0.0.0",
        "::1",
    ];
    for raw in loopbacks {
        let ip: IpAddr = raw.parse().expect("valid IP syntax");
        assert!(
            is_blocked_ip(ip),
            "Loopback/unspecified IP {} must be strictly blocked",
            raw
        );
    }

    // 2. Cloud metadata & link-local
    let link_locals: Vec<&str> = vec![
        "169.254.169.254",
        "169.254.0.1",
        "169.254.255.255",
        "fe80::1",
        "fe80::dead:beef",
    ];
    for raw in link_locals {
        let ip: IpAddr = raw.parse().expect("valid IP syntax");
        assert!(
            is_blocked_ip(ip),
            "Link-local / cloud metadata IP {} must be strictly blocked",
            raw
        );
    }

    // 3. RFC 1918 Private ranges
    let privates: Vec<&str> = vec![
        "10.0.0.1",
        "10.255.255.254",
        "172.16.0.1",
        "172.31.255.254",
        "192.168.0.1",
        "192.168.1.254",
    ];
    for raw in privates {
        let ip: IpAddr = raw.parse().expect("valid IP syntax");
        assert!(
            is_blocked_ip(ip),
            "Private RFC 1918 IP {} must be strictly blocked",
            raw
        );
    }

    // 4. CGNAT (RFC 6598) and Multicast
    let cgnat_and_multicast: Vec<&str> = vec![
        "100.64.0.1",
        "100.127.255.254",
        "224.0.0.1",
        "239.255.255.255",
        "ff02::1",
    ];
    for raw in cgnat_and_multicast {
        let ip: IpAddr = raw.parse().expect("valid IP syntax");
        assert!(
            is_blocked_ip(ip),
            "CGNAT or Multicast IP {} must be strictly blocked",
            raw
        );
    }

    // 5. IPv4-mapped IPv6 bypass attempts
    let ipv4_mapped: Vec<&str> = vec![
        "::ffff:127.0.0.1",
        "::ffff:169.254.169.254",
        "::ffff:10.0.0.1",
        "::ffff:192.168.1.1",
    ];
    for raw in ipv4_mapped {
        let ip: IpAddr = raw.parse().expect("valid IP syntax");
        assert!(
            is_blocked_ip(ip),
            "IPv4-mapped IPv6 address {} must be detected and blocked",
            raw
        );
    }

    // 6. Legitimate public IPs must be permitted
    let public_ips: Vec<&str> = vec![
        "8.8.8.8",
        "1.1.1.1",
        "93.184.216.34",        // example.com
        "2606:4700:4700::1111", // cloudflare dns ipv6
    ];
    for raw in public_ips {
        let ip: IpAddr = raw.parse().expect("valid IP syntax");
        assert!(
            !is_blocked_ip(ip),
            "Legitimate public IP {} must NOT be blocked",
            raw
        );
    }
}

#[test]
fn test_adversarial_domain_spoofing_attempts() {
    let policy = SandboxPolicy {
        allow_net: true,
        allowed_hosts: vec!["api.stripe.com".to_string(), "api.github.com".to_string()],
        ..Default::default()
    };

    // Attacker-controlled suffix and prefix spoofing
    let spoof_domains = vec![
        "api.stripe.com.attacker.com",
        "evil-api.stripe.com",
        "api.stripe.com.evil.org",
        "fakeapi.stripe.com",
        "stripe.com.attacker.io",
    ];

    for domain in spoof_domains {
        assert!(
            !policy.is_host_allowed(domain),
            "Domain spoofing attempt '{}' must be rejected",
            domain
        );
    }

    // Valid exact and subdomain matches
    assert!(policy.is_host_allowed("api.stripe.com"));
    assert!(policy.is_host_allowed("api.github.com"));
}

#[test]
fn test_adversarial_sql_injection_and_destructive_queries() {
    let read_only_policy = SandboxPolicy {
        allow_sql: true,
        read_only_sql: true,
        allowed_tables: vec!["refunds".to_string(), "customers".to_string()],
        ..Default::default()
    };

    let attack_queries = vec![
        // Direct DDL/DML attacks
        "DROP TABLE refunds;",
        "ALTER TABLE refunds ADD COLUMN pwned TEXT;",
        "TRUNCATE TABLE refunds;",
        "DELETE FROM refunds WHERE 1=1;",
        "UPDATE refunds SET amount = 999999 WHERE id = 1;",
        "INSERT INTO refunds (customer, amount) VALUES ('attacker', 1000);",
        "REPLACE INTO refunds (id, amount) VALUES (1, 1000);",
        // Stacked queries / comment tricks
        "SELECT * FROM refunds; DROP TABLE refunds; --",
        "SELECT * FROM refunds WHERE id = 1; DELETE FROM refunds;",
        // Unauthorized table access
        "SELECT * FROM user_passwords;",
        "SELECT * FROM sqlite_master;",
        "SELECT password_hash FROM admin_credentials;",
        // CTE mutation attack
        "WITH cte AS (SELECT 1) DELETE FROM refunds WHERE 1=1;",
        // Comment bypass attack
        "/* comment */ DROP TABLE refunds;",
        "-- leading comment\nTRUNCATE TABLE refunds;",
        // Comma join table allowlist bypass
        "SELECT * FROM refunds, admin_credentials WHERE refunds.id = admin_credentials.id;",
    ];

    for q in attack_queries {
        let res = validate_sql_policy(q, &read_only_policy);
        assert!(
            res.is_err(),
            "Malicious SQL query '{}' must be blocked by sandbox policy",
            q
        );
    }

    // Legitimate read queries against whitelisted tables must succeed
    assert!(validate_sql_policy("SELECT * FROM refunds WHERE id = $1", &read_only_policy).is_ok());
    assert!(validate_sql_policy("SELECT count(*) FROM customers", &read_only_policy).is_ok());
}

#[test]
fn test_adversarial_template_injection_and_path_traversal() {
    // Malicious YAML attempting path traversal in workflow name
    let traversal_yaml = r#"
name: "../../etc/passwd"
version: "1.0.0"
description: Traversal attempt
steps:
  - id: s1
    op: transform
    expr: "1 + 1"
    output_var: res
return:
  status: 200
  value: {}
"#;

    let spec: WorkflowSpecV1 = serde_yaml::from_str(traversal_yaml).expect("valid yaml");
    let err = worpen_core::workflow::WorkflowValidator::validate(&spec);
    assert!(
        err.is_err(),
        "Path traversal in workflow name must be rejected"
    );
    assert!(err.unwrap_err().contains("Invalid workflow name"));
}

#[test]
fn test_adversarial_unbounded_loop_rejection() {
    // 1. Missing max_iterations: rejected at parse time
    let missing_max_iter_yaml = r#"
name: loop_attack
version: "1.0.0"
description: Missing max_iterations
steps:
  - id: loop_step
    op: loop
    items: "items"
    as_var: item
    body: []
return:
  status: 200
  value: {}
"#;
    assert!(serde_yaml::from_str::<WorkflowSpecV1>(missing_max_iter_yaml).is_err());

    // 2. max_iterations: 0 (rejected during validation)
    let zero_iter_yaml = r#"
name: loop_attack_zero
version: "1.0.0"
description: Zero iterations
steps:
  - id: loop_step
    op: loop
    items: "items"
    as_var: item
    max_iterations: 0
    body: []
return:
  status: 200
  value: {}
"#;
    let spec1: WorkflowSpecV1 = serde_yaml::from_str(zero_iter_yaml).unwrap();
    let err1 = worpen_core::workflow::WorkflowValidator::validate(&spec1);
    assert!(err1.is_err());
    assert!(err1.unwrap_err().contains("must be greater than 0"));

    // 3. Exceeds policy cap (e.g. 50,000 iterations when policy is 1,000)
    let excessive_iter_yaml = r#"
name: loop_attack_excessive
version: "1.0.0"
description: Excessive iterations
steps:
  - id: loop_step
    op: loop
    items: "items"
    as_var: item
    max_iterations: 50000
    body: []
return:
  status: 200
  value: {}
"#;
    let spec2: WorkflowSpecV1 = serde_yaml::from_str(excessive_iter_yaml).unwrap();
    let err2 = worpen_core::workflow::WorkflowValidator::validate(&spec2);
    assert!(err2.is_err());
    assert!(err2.unwrap_err().contains("exceeds policy cap"));
}

#[test]
fn test_adversarial_sql_ast_nested_subqueries_and_ctes() {
    let policy = SandboxPolicy {
        allow_sql: true,
        read_only_sql: true,
        allowed_tables: vec!["orders".to_string(), "customers".to_string()],
        ..Default::default()
    };

    // Subquery trying to read from forbidden table `credentials`
    let subquery_in_where =
        "SELECT * FROM orders WHERE customer_id IN (SELECT id FROM credentials)";
    let res1 = validate_sql_policy(subquery_in_where, &policy);
    assert!(res1.is_err());
    assert!(res1.unwrap_err().contains("credentials"));

    // Subquery in FROM derived table
    let subquery_in_from =
        "SELECT o.id FROM (SELECT * FROM credentials) AS c JOIN orders o ON o.customer_id = c.id";
    let res2 = validate_sql_policy(subquery_in_from, &policy);
    assert!(res2.is_err());
    assert!(res2.unwrap_err().contains("credentials"));

    // CTE referencing forbidden table
    let cte_leak = "WITH leaked AS (SELECT pass FROM credentials) SELECT * FROM orders, leaked";
    let res3 = validate_sql_policy(cte_leak, &policy);
    assert!(res3.is_err());
    assert!(res3.unwrap_err().contains("credentials"));

    // Allowed CTE with only allowed tables
    let cte_safe = "WITH safe_cte AS (SELECT id FROM customers) SELECT * FROM orders JOIN safe_cte ON orders.customer_id = safe_cte.id";
    let res4 = validate_sql_policy(cte_safe, &policy);
    assert!(res4.is_ok(), "Safe CTE on allowed tables must pass");
}

#[test]
fn test_hmac_audit_key_generation_and_permissions() {
    use std::fs;
    use std::path::Path;

    // Ensure WORPEN_AUDIT_KEY is not set for this test
    std::env::remove_var("WORPEN_AUDIT_KEY");
    let key_path = Path::new(".worpen_audit.key");

    let key = worpen_core::audit::get_audit_secret_key();
    assert_eq!(key.len(), 32, "Key must be 32 bytes");
    assert!(key_path.exists(), ".worpen_audit.key file must exist");

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let meta = fs::metadata(key_path).unwrap();
        let mode = meta.mode() & 0o777;
        assert_eq!(mode, 0o600, "Key file permissions must be 0600 on unix");
    }

    // Subsequent retrieval reads the existing file
    let key2 = worpen_core::audit::get_audit_secret_key();
    assert_eq!(key, key2, "Subsequent reads must return the same key");
}

#[test]
fn test_ast_sql_policy_subqueries_and_ctes_adversarial() {
    let policy = SandboxPolicy {
        allow_sql: true,
        read_only_sql: true,
        allowed_tables: vec!["allowed_table".to_string()],
        ..Default::default()
    };

    // 1. Subquery in WHERE: SELECT * FROM allowed_table WHERE id IN (SELECT id FROM secret_table)
    let q_where = "SELECT * FROM allowed_table WHERE id IN (SELECT id FROM secret_table)";
    let res_where = validate_sql_policy(q_where, &policy);
    assert!(
        res_where.is_err(),
        "Subquery in WHERE referencing secret_table must fail validate_sql_policy"
    );
    assert!(res_where.unwrap_err().contains("secret_table"));

    // 2. Subquery in FROM: SELECT * FROM (SELECT * FROM secret_table) AS sub
    let q_from = "SELECT * FROM (SELECT * FROM secret_table) AS sub";
    let res_from = validate_sql_policy(q_from, &policy);
    assert!(
        res_from.is_err(),
        "Subquery in FROM referencing secret_table must fail validate_sql_policy"
    );
    assert!(res_from.unwrap_err().contains("secret_table"));

    // 3. CTE subquery: WITH x AS (SELECT * FROM secret_table) SELECT * FROM allowed_table
    let q_cte = "WITH x AS (SELECT * FROM secret_table) SELECT * FROM allowed_table";
    let res_cte = validate_sql_policy(q_cte, &policy);
    assert!(
        res_cte.is_err(),
        "CTE referencing secret_table must fail validate_sql_policy"
    );
    assert!(res_cte.unwrap_err().contains("secret_table"));

    // 4. Valid query with CTE on allowed table: WITH x AS (SELECT * FROM allowed_table) SELECT * FROM x
    let q_safe = "WITH x AS (SELECT * FROM allowed_table) SELECT * FROM x";
    let res_safe = validate_sql_policy(q_safe, &policy);
    assert!(
        res_safe.is_ok(),
        "Valid CTE query on allowed_table must pass: {:?}",
        res_safe.err()
    );

    // 5. EXISTS subquery referencing secret_table
    let q_exists = "SELECT * FROM allowed_table WHERE EXISTS (SELECT 1 FROM secret_table WHERE secret_table.id = allowed_table.id)";
    let res_exists = validate_sql_policy(q_exists, &policy);
    assert!(res_exists.is_err());
    assert!(res_exists.unwrap_err().contains("secret_table"));

    // 6. Projection subquery referencing secret_table
    let q_proj = "SELECT (SELECT secret FROM secret_table LIMIT 1) FROM allowed_table";
    let res_proj = validate_sql_policy(q_proj, &policy);
    assert!(res_proj.is_err());
    assert!(res_proj.unwrap_err().contains("secret_table"));
}

#[tokio::test]
async fn test_ssrf_redirect_attack_blocked_at_hop() {
    use proto::models::ExecutionMode;
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use worpen_core::services::DynamicRouteService;
    use worpen_core::workflow::service::WorkflowService;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let server_handle = tokio::spawn(async move {
        for _ in 0..3 {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = vec![0u8; 1024];
                let n = socket.read(&mut buf).await.unwrap_or(0);
                let req_text = String::from_utf8_lossy(&buf[..n]);

                let response = if req_text.contains("/redirect_to_metadata") {
                    "HTTP/1.1 302 Found\r\nLocation: http://169.254.169.254/latest/meta-data/\r\nContent-Length: 0\r\n\r\n".to_string()
                } else if req_text.contains("/redirect_to_loopback") {
                    "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9099/metadata\r\nContent-Length: 0\r\n\r\n".to_string()
                } else if req_text.contains("/redirect_to_internal_ip") {
                    "HTTP/1.1 302 Found\r\nLocation: http://10.0.0.1/admin\r\nContent-Length: 0\r\n\r\n".to_string()
                } else {
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK".to_string()
                };
                let _ = socket.write_all(response.as_bytes()).await;
            }
        }
    });

    let temp_dir = std::env::temp_dir().join(format!("worpen_ssrf_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool);
    let route_service_arc = Arc::new(route_service);
    let workflow_service = WorkflowService::new(data_dir, route_service_arc);

    // 1. Initial URL redirects (302) to cloud metadata IP 169.254.169.254
    let yaml_meta = format!(
        r#"
name: ssrf_redirect_metadata
version: "1.0"
policy:
  allow_net: true
  allowed_hosts:
    - "127.0.0.1"
  block_private_ips: false
steps:
  - id: http_step
    op: http
    method: GET
    url: "http://127.0.0.1:{}/redirect_to_metadata"
    output_var: res
return:
  status: 200
  value: "{{{{res}}}}"
"#,
        port
    );
    workflow_service
        .register_workflow(&yaml_meta)
        .await
        .unwrap();
    let err_meta = workflow_service
        .execute("ssrf_redirect_metadata", None, ExecutionMode::Live)
        .await
        .unwrap_err();
    assert!(
        err_meta.contains("sandbox: redirect to 'http://169.254.169.254/latest/meta-data/' denied"),
        "Expected redirect to cloud metadata to be denied, got: {}",
        err_meta
    );

    // 2. Initial URL redirects (302) to http://127.0.0.1:9099/metadata where 127.0.0.1 is not in allowed_hosts
    let yaml_loopback = format!(
        r#"
name: ssrf_redirect_loopback
version: "1.0"
policy:
  allow_net: true
  allowed_hosts:
    - "localhost"
  block_private_ips: false
steps:
  - id: http_step
    op: http
    method: GET
    url: "http://localhost:{}/redirect_to_loopback"
    output_var: res
return:
  status: 200
  value: "{{{{res}}}}"
"#,
        port
    );
    workflow_service
        .register_workflow(&yaml_loopback)
        .await
        .unwrap();
    let err_loopback = workflow_service
        .execute("ssrf_redirect_loopback", None, ExecutionMode::Live)
        .await
        .unwrap_err();
    assert!(
        err_loopback.contains("sandbox: redirect to 'http://127.0.0.1:9099/metadata' denied"),
        "Expected redirect to loopback to be denied, got: {}",
        err_loopback
    );

    // 3. Initial URL redirects (302) to private IP 10.0.0.1
    let yaml_internal = format!(
        r#"
name: ssrf_redirect_internal
version: "1.0"
policy:
  allow_net: true
  allowed_hosts:
    - "127.0.0.1"
  block_private_ips: false
steps:
  - id: http_step
    op: http
    method: GET
    url: "http://127.0.0.1:{}/redirect_to_internal_ip"
    output_var: res
return:
  status: 200
  value: "{{{{res}}}}"
"#,
        port
    );
    workflow_service
        .register_workflow(&yaml_internal)
        .await
        .unwrap();
    let err_internal = workflow_service
        .execute("ssrf_redirect_internal", None, ExecutionMode::Live)
        .await
        .unwrap_err();
    assert!(
        err_internal.contains("sandbox: redirect to 'http://10.0.0.1/admin' denied"),
        "Expected redirect to 10.0.0.1 to be denied, got: {}",
        err_internal
    );

    let _ = server_handle.await;
}

#[tokio::test]
async fn test_ssrf_redirect_loop_capped_at_max_hops() {
    use proto::models::ExecutionMode;
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use worpen_core::services::DynamicRouteService;
    use worpen_core::workflow::service::WorkflowService;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let server_handle = tokio::spawn(async move {
        for _ in 0..10 {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = vec![0u8; 1024];
                let _ = socket.read(&mut buf).await;
                let response = format!(
                    "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:{}/infinite_loop\r\nContent-Length: 0\r\n\r\n",
                    port
                );
                let _ = socket.write_all(response.as_bytes()).await;
            }
        }
    });

    let temp_dir = std::env::temp_dir().join(format!("worpen_loop_test_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    let db_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    let mut route_service = DynamicRouteService::with_data_dir(data_dir.clone());
    route_service.set_db_pool(db_pool);
    let route_service_arc = Arc::new(route_service);
    let workflow_service = WorkflowService::new(data_dir, route_service_arc);

    let yaml = format!(
        r#"
name: ssrf_redirect_infinite_loop
version: "1.0"
policy:
  allow_net: true
  allowed_hosts:
    - "127.0.0.1"
  block_private_ips: false
steps:
  - id: http_step
    op: http
    method: GET
    url: "http://127.0.0.1:{}/infinite_loop"
    output_var: res
return:
  status: 200
  value: "{{{{res}}}}"
"#,
        port
    );

    workflow_service.register_workflow(&yaml).await.unwrap();
    let err = workflow_service
        .execute("ssrf_redirect_infinite_loop", None, ExecutionMode::Live)
        .await
        .unwrap_err();

    assert!(
        err.contains("sandbox: redirect limit exceeded (max 5 hops)"),
        "Expected redirect limit error, got: {}",
        err
    );

    server_handle.abort();
}
