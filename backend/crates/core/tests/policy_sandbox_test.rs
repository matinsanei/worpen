use proto::models::SandboxPolicy;
use worpen_core::sandbox::{is_blocked_ip, resolve_and_validate_host, validate_sql_policy};

#[test]
fn test_ssrf_ip_blocklist_comprehensive() {
    // 1. Loopback addresses
    assert!(is_blocked_ip("127.0.0.1".parse().unwrap()));
    assert!(is_blocked_ip("127.1.2.3".parse().unwrap()));
    assert!(is_blocked_ip("::1".parse().unwrap()));

    // 2. Private IPv4 (RFC 1918)
    assert!(is_blocked_ip("10.0.0.1".parse().unwrap()));
    assert!(is_blocked_ip("10.254.254.254".parse().unwrap()));
    assert!(is_blocked_ip("172.16.0.1".parse().unwrap()));
    assert!(is_blocked_ip("172.31.255.255".parse().unwrap()));
    assert!(is_blocked_ip("192.168.1.1".parse().unwrap()));

    // 3. Cloud metadata & Link-Local (AWS/GCP/Azure: 169.254.169.254)
    assert!(is_blocked_ip("169.254.169.254".parse().unwrap()));
    assert!(is_blocked_ip("fe80::1".parse().unwrap()));

    // 4. Carrier-Grade NAT (RFC 6598: 100.64.0.0/10)
    assert!(is_blocked_ip("100.64.0.1".parse().unwrap()));
    assert!(is_blocked_ip("100.127.255.254".parse().unwrap()));

    // 5. Broadcast, Unspecified, Multicast
    assert!(is_blocked_ip("0.0.0.0".parse().unwrap()));
    assert!(is_blocked_ip("255.255.255.255".parse().unwrap()));
    assert!(is_blocked_ip("224.0.0.1".parse().unwrap()));
    assert!(is_blocked_ip("ff02::1".parse().unwrap()));

    // 6. IPv4-mapped IPv6
    assert!(is_blocked_ip("::ffff:127.0.0.1".parse().unwrap()));
    assert!(is_blocked_ip("::ffff:10.0.0.1".parse().unwrap()));
    assert!(is_blocked_ip("::ffff:169.254.169.254".parse().unwrap()));

    // 7. Legitimate Public IPs (must NOT be blocked)
    assert!(!is_blocked_ip("8.8.8.8".parse().unwrap()));
    assert!(!is_blocked_ip("1.1.1.1".parse().unwrap()));
    assert!(!is_blocked_ip("93.184.216.34".parse().unwrap())); // example.com
    assert!(!is_blocked_ip("2606:4700:4700::1111".parse().unwrap())); // Cloudflare DNS IPv6
}

#[tokio::test]
async fn test_ssrf_host_resolution_guard() {
    let policy = SandboxPolicy {
        allow_net: true,
        allowed_hosts: vec!["127.0.0.1".to_string(), "api.stripe.com".to_string()],
        block_private_ips: true,
        ..Default::default()
    };

    // Explicit private IP must be rejected when block_private_ips = true
    let loopback_res = resolve_and_validate_host("http://127.0.0.1:8080/v1", &policy).await;
    assert!(loopback_res.is_err());
    let err = loopback_res.unwrap_err();
    assert!(
        err.contains("SSRF blocked"),
        "Expected SSRF block error, got: {}",
        err
    );

    // Unallowed host must be rejected
    let unallowed_res = resolve_and_validate_host("https://evil.com/steal", &policy).await;
    assert!(unallowed_res.is_err());
    assert!(unallowed_res.unwrap_err().contains("denied by allowlist"));

    // Substring spoofing (e.g. api.stripe.com.evil.com) must be rejected
    let spoof_res = resolve_and_validate_host("https://api.stripe.com.evil.com/pay", &policy).await;
    assert!(spoof_res.is_err());
    assert!(spoof_res.unwrap_err().contains("denied by allowlist"));
}

#[test]
fn test_sql_sandbox_read_only_and_table_allowlist() {
    let policy = SandboxPolicy {
        allow_sql: true,
        read_only_sql: true,
        allowed_tables: vec!["customers".to_string(), "invoices".to_string()],
        ..Default::default()
    };

    // 1. Read-only checks
    let valid_select = validate_sql_policy("SELECT id, name FROM customers WHERE id = 1", &policy);
    assert!(valid_select.is_ok());

    let denied_insert =
        validate_sql_policy("INSERT INTO customers (name) VALUES ('Mallory')", &policy);
    assert!(denied_insert.is_err());
    assert!(denied_insert.unwrap_err().contains("read_only_sql=true"));

    let denied_delete = validate_sql_policy("DELETE FROM customers WHERE id = 1", &policy);
    assert!(denied_delete.is_err());
    assert!(denied_delete.unwrap_err().contains("read_only_sql=true"));

    let denied_drop = validate_sql_policy("DROP TABLE invoices", &policy);
    assert!(denied_drop.is_err());
    assert!(denied_drop.unwrap_err().contains("read_only_sql=true"));

    // Semicolon injection of write query
    let denied_compound =
        validate_sql_policy("SELECT 1 FROM customers; DELETE FROM invoices", &policy);
    assert!(denied_compound.is_err());
    assert!(denied_compound.unwrap_err().contains("read_only_sql=true"));

    // 2. Table allowlist check on unallowed table
    let write_policy = SandboxPolicy {
        allow_sql: true,
        read_only_sql: false,
        allowed_tables: vec!["invoices".to_string()],
        ..Default::default()
    };

    let allowed_table = validate_sql_policy(
        "UPDATE invoices SET status = 'paid' WHERE id = 10",
        &write_policy,
    );
    assert!(allowed_table.is_ok());

    let unallowed_table = validate_sql_policy("SELECT * FROM credentials", &write_policy);
    assert!(unallowed_table.is_err());
    assert!(unallowed_table
        .unwrap_err()
        .contains("denied by allowed_tables"));
}

#[test]
fn test_ready_made_policy_packs() {
    let analyst = SandboxPolicy::read_only_analyst();
    assert!(!analyst.allow_net);
    assert!(analyst.allow_sql);
    assert!(analyst.read_only_sql);
    assert_eq!(analyst.max_sql_rows, 500);

    let billing = SandboxPolicy::billing_operator();
    assert!(billing.allow_net);
    assert!(billing
        .allowed_hosts
        .contains(&"api.stripe.com".to_string()));
    assert!(billing.block_private_ips);
    assert!(!billing.read_only_sql);
    assert!(billing.allowed_tables.contains(&"invoices".to_string()));
    assert!(billing.allowed_tables.contains(&"payments".to_string()));

    let support = SandboxPolicy::support_agent();
    assert!(support.allow_net);
    assert!(support
        .allowed_hosts
        .contains(&"api.zendesk.com".to_string()));
    assert!(support.read_only_sql);
    assert!(support.allowed_tables.contains(&"tickets".to_string()));
}
