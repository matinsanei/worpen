use api::cli::{AuditVerifyArgs, DoctorArgs, InitArgs};
use api::ops::{audit_verify, doctor, init};
use proto::models::workflow::WorkflowSpecV1;
use std::fs;

#[tokio::test]
async fn test_cli_init_generates_valid_scaffold_and_templates() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_cli_init_{}", uuid::Uuid::new_v4()));
    let path_str = temp_dir.to_str().unwrap().to_string();

    let init_args = InitArgs {
        path: path_str.clone(),
    };
    init(&init_args).await.expect("worpen init should succeed");

    assert!(temp_dir.join("workflows").exists());
    assert!(temp_dir.join("data").exists());
    assert!(temp_dir.join(".cursor/mcp.json").exists());
    assert!(temp_dir.join(".env.example").exists());
    assert!(temp_dir.join("worpen.yaml").exists());

    // Verify all starter templates parse into valid WorkflowSpecV1
    for template_name in ["refund.yaml", "db_report.yaml", "issue_triage.yaml"] {
        let file_path = temp_dir.join("workflows").join(template_name);
        assert!(file_path.exists(), "Template {} must exist", template_name);
        let content = fs::read_to_string(&file_path).unwrap();
        let spec: Result<WorkflowSpecV1, _> = serde_yaml::from_str(&content);
        assert!(
            spec.is_ok(),
            "Template {} must parse into valid WorkflowSpecV1: {:?}",
            template_name,
            spec.err()
        );
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[tokio::test]
async fn test_cli_doctor_runs_cleanly() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_cli_doctor_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();

    let doctor_args = DoctorArgs {
        data_dir: temp_dir.to_str().unwrap().to_string(),
    };

    let result = doctor(&doctor_args).await;
    assert!(result.is_ok(), "doctor check must pass");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[tokio::test]
async fn test_cli_audit_verify_on_empty_and_active_chain() {
    let temp_dir = std::env::temp_dir().join(format!("worpen_cli_audit_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir).unwrap();
    let data_dir = temp_dir.to_str().unwrap().to_string();

    // 1. Empty audit log check
    let args = AuditVerifyArgs {
        data_dir: data_dir.clone(),
        server: None,
        token: None,
    };
    let res = audit_verify(&args).await;
    assert!(res.is_ok(), "Empty chain verification must succeed");

    // 2. Active audit log
    let audit_file = temp_dir.join("audit_chain.jsonl");
    let record = serde_json::json!({
        "index": 0,
        "timestamp": "2026-11-15T00:00:00Z",
        "prev_hash": "0000000000000000000000000000000000000000000000000000000000000000",
        "action": "test.init",
        "actor": "admin",
        "route_or_workflow": "system",
        "details": { "status": "ok" },
        "entry_hash": worpen_core::audit::compute_entry_hash(
            0,
            "2026-11-15T00:00:00Z",
            "0000000000000000000000000000000000000000000000000000000000000000",
            "test.init",
            "admin",
            "system",
            &serde_json::json!({ "status": "ok" }),
            None
        )
    });
    fs::write(&audit_file, format!("{}\n", record)).unwrap();

    let res2 = audit_verify(&args).await;
    assert!(res2.is_ok(), "Active chain verification must succeed");

    let _ = fs::remove_dir_all(&temp_dir);
}
