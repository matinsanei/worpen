use serde_json::json;
use std::fs;
use worpen_core::audit::{
    compute_entry_hash, export_cef, export_jsonl, export_otel, verify_audit_chain,
    AuditChainManager, AuditRecord, GENESIS_HASH,
};

#[test]
fn test_audit_chain_verification_clean_chain() {
    let mut records = Vec::new();
    let mut prev_hash = GENESIS_HASH.to_string();

    for i in 0..5 {
        let ts = format!("2026-11-14T12:0{}:00Z", i);
        let action = format!("action.step_{}", i);
        let details = json!({ "step": i, "val": format!("test_{}", i) });
        let entry_hash = compute_entry_hash(
            i,
            &ts,
            &prev_hash,
            &action,
            "tester",
            "test_wf",
            &details,
            Some("allowed"),
        );

        records.push(AuditRecord {
            index: i,
            timestamp: ts,
            prev_hash: prev_hash.clone(),
            action,
            actor: "tester".to_string(),
            route_or_workflow: "test_wf".to_string(),
            details,
            policy_decision: Some("allowed".to_string()),
            entry_hash: entry_hash.clone(),
        });
        prev_hash = entry_hash;
    }

    let jsonl = export_jsonl(&records);
    let report = verify_audit_chain(&jsonl).expect("clean chain should verify successfully");
    assert!(report.valid);
    assert_eq!(report.total_entries, 5);
    assert_eq!(report.latest_hash, prev_hash);
}

#[test]
fn test_audit_chain_detects_tampered_details() {
    let mut records = Vec::new();
    let mut prev_hash = GENESIS_HASH.to_string();

    for i in 0..4 {
        let ts = format!("2026-11-14T12:0{}:00Z", i);
        let action = format!("action.step_{}", i);
        let details = json!({ "amount": 100 * (i + 1) });
        let entry_hash = compute_entry_hash(
            i,
            &ts,
            &prev_hash,
            &action,
            "tester",
            "test_wf",
            &details,
            Some("allowed"),
        );

        records.push(AuditRecord {
            index: i,
            timestamp: ts,
            prev_hash: prev_hash.clone(),
            action,
            actor: "tester".to_string(),
            route_or_workflow: "test_wf".to_string(),
            details,
            policy_decision: Some("allowed".to_string()),
            entry_hash: entry_hash.clone(),
        });
        prev_hash = entry_hash;
    }

    // Tamper with details at index 2 (an attacker modified the amount from 300 to 999999)
    records[2].details = json!({ "amount": 999999 });

    let jsonl = export_jsonl(&records);
    let err = verify_audit_chain(&jsonl).expect_err("tampered chain must fail verification");
    assert_eq!(err.broken_index, 2);
    assert!(err.reason.contains("Tampered entry hash mismatch"));
}

#[test]
fn test_audit_chain_detects_broken_hash_pointer() {
    let mut records = Vec::new();
    let mut prev_hash = GENESIS_HASH.to_string();

    for i in 0..4 {
        let ts = format!("2026-11-14T12:0{}:00Z", i);
        let action = format!("action.step_{}", i);
        let details = json!({ "step": i });
        let entry_hash = compute_entry_hash(
            i, &ts, &prev_hash, &action, "tester", "test_wf", &details, None,
        );

        records.push(AuditRecord {
            index: i,
            timestamp: ts,
            prev_hash: prev_hash.clone(),
            action,
            actor: "tester".to_string(),
            route_or_workflow: "test_wf".to_string(),
            details,
            policy_decision: None,
            entry_hash: entry_hash.clone(),
        });
        prev_hash = entry_hash;
    }

    // Modify the prev_hash pointer of record 3
    records[3].prev_hash =
        "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_string();

    let jsonl = export_jsonl(&records);
    let err = verify_audit_chain(&jsonl).expect_err("broken prev_hash must fail");
    assert_eq!(err.broken_index, 3);
    assert!(err.reason.contains("Broken hash chain"));
}

#[test]
fn test_audit_chain_detects_index_discontinuity() {
    let mut records = Vec::new();
    let mut prev_hash = GENESIS_HASH.to_string();

    for i in 0..3 {
        let ts = format!("2026-11-14T12:0{}:00Z", i);
        let action = format!("action.step_{}", i);
        let details = json!({ "step": i });
        let entry_hash = compute_entry_hash(
            i, &ts, &prev_hash, &action, "tester", "test_wf", &details, None,
        );

        records.push(AuditRecord {
            index: i,
            timestamp: ts,
            prev_hash: prev_hash.clone(),
            action,
            actor: "tester".to_string(),
            route_or_workflow: "test_wf".to_string(),
            details,
            policy_decision: None,
            entry_hash: entry_hash.clone(),
        });
        prev_hash = entry_hash;
    }

    // Change index of record 2 from 2 to 4 (a deleted entry)
    records[2].index = 4;

    let jsonl = export_jsonl(&records);
    let err = verify_audit_chain(&jsonl).expect_err("index jump must fail");
    assert_eq!(err.broken_index, 4);
    assert!(err.reason.contains("Index discontinuity"));
}

#[test]
fn test_audit_manager_append_and_persistence() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_audit_test_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    {
        let manager = AuditChainManager::new(&data_dir);
        let rec1 = manager
            .append(
                "workflow.start",
                "claude_agent",
                "customer_refund",
                json!({ "customer_id": "cust_123", "amount": 49.99 }),
                Some("allowed"),
            )
            .unwrap();
        assert_eq!(rec1.index, 0);
        assert_eq!(rec1.prev_hash, GENESIS_HASH);

        let rec2 = manager
            .append(
                "workflow.approval_requested",
                "system",
                "customer_refund",
                json!({ "refund_id": "ref_999" }),
                Some("approval_pending"),
            )
            .unwrap();
        assert_eq!(rec2.index, 1);
        assert_eq!(rec2.prev_hash, rec1.entry_hash);

        let report = manager.verify().unwrap();
        assert!(report.valid);
        assert_eq!(report.total_entries, 2);
    }

    // Re-open from disk: verify continuation of state
    {
        let manager2 = AuditChainManager::new(&data_dir);
        let rec3 = manager2
            .append(
                "workflow.approved",
                "sarah_manager",
                "customer_refund",
                json!({ "reason": "VIP customer exception" }),
                Some("allowed"),
            )
            .unwrap();
        assert_eq!(rec3.index, 2);

        let report = manager2.verify().unwrap();
        assert!(report.valid);
        assert_eq!(report.total_entries, 3);

        let records = manager2.read_all().unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].action, "workflow.start");
        assert_eq!(records[1].action, "workflow.approval_requested");
        assert_eq!(records[2].action, "workflow.approved");
    }
}

#[test]
fn test_export_formats_cef_and_otel() {
    let records = vec![
        AuditRecord {
            index: 0,
            timestamp: "2026-11-14T12:00:00Z".to_string(),
            prev_hash: GENESIS_HASH.to_string(),
            action: "workflow.start".to_string(),
            actor: "agent_alpha".to_string(),
            route_or_workflow: "refund_wf".to_string(),
            details: json!({ "order": "123" }),
            policy_decision: Some("allowed".to_string()),
            entry_hash: "hash0".to_string(),
        },
        AuditRecord {
            index: 1,
            timestamp: "2026-11-14T12:01:00Z".to_string(),
            prev_hash: "hash0".to_string(),
            action: "workflow.denied".to_string(),
            actor: "agent_alpha".to_string(),
            route_or_workflow: "refund_wf".to_string(),
            details: json!({ "error": "SSRF attempt" }),
            policy_decision: Some("denied_ssrf_blocked".to_string()),
            entry_hash: "hash1".to_string(),
        },
    ];

    // CEF format check
    let cef = export_cef(&records);
    let lines: Vec<&str> = cef.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].starts_with("CEF:0|Worpen|ActionPlane|0.1.0|workflow.start|"));
    assert!(lines[0].contains("|3|")); // normal severity
    assert!(lines[1].contains("|7|")); // denied severity

    // OTel format check
    let otel = export_otel(&records);
    assert!(otel.get("resourceLogs").is_some());
    let log_records = &otel["resourceLogs"][0]["scopeLogs"][0]["logRecords"];
    assert_eq!(log_records.as_array().unwrap().len(), 2);
    assert_eq!(log_records[0]["severityText"], "INFO");
    assert_eq!(log_records[1]["severityText"], "WARN");
}
