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
