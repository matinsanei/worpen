use super::chain::AuditRecord;
use serde_json::{json, Value};

/// Export audit records as newline-delimited JSON
pub fn export_jsonl(records: &[AuditRecord]) -> String {
    let mut out = String::new();
    for r in records {
        if let Ok(line) = serde_json::to_string(r) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out
}

/// Export audit records in standard Common Event Format (CEF)
/// Format: CEF:Version|Device Vendor|Device Product|Device Version|Device Event Class ID|Name|Severity|[Extension]
pub fn export_cef(records: &[AuditRecord]) -> String {
    let mut out = String::new();

    for r in records {
        let severity = if r
            .policy_decision
            .as_ref()
            .map(|p| p.starts_with("denied"))
            .unwrap_or(false)
        {
            "7" // High
        } else {
            "3" // Low / Normal
        };

        let details_escaped = serde_json::to_string(&r.details)
            .unwrap_or_default()
            .replace('\\', "\\\\")
            .replace('=', "\\=");

        let cef_line = format!(
            "CEF:0|Worpen|ActionPlane|0.1.0|{}|{}|{}|src={} cs1={} cs1Label=Workflow msg={} cn1={} cn1Label=Index cs2={} cs2Label=EntryHash",
            r.action,
            r.action,
            severity,
            r.actor,
            r.route_or_workflow,
            details_escaped,
            r.index,
            r.entry_hash
        );
        out.push_str(&cef_line);
        out.push('\n');
    }

    out
}

/// Export audit records in OpenTelemetry (OTel) JSON LogRecords format
pub fn export_otel(records: &[AuditRecord]) -> Value {
    let log_records: Vec<Value> = records
        .iter()
        .map(|r| {
            let severity_number = if r.policy_decision.as_ref().map(|p| p.starts_with("denied")).unwrap_or(false) {
                13 // WARN
            } else {
                9 // INFO
            };

            let severity_text = if severity_number == 13 { "WARN" } else { "INFO" };

            json!({
                "timeUnixNano": r.timestamp,
                "severityNumber": severity_number,
                "severityText": severity_text,
                "body": {
                    "stringValue": format!("Action '{}' performed by '{}' on '{}'", r.action, r.actor, r.route_or_workflow)
                },
                "attributes": [
                    { "key": "worpen.audit.index", "value": { "intValue": r.index } },
                    { "key": "worpen.audit.action", "value": { "stringValue": &r.action } },
                    { "key": "worpen.audit.actor", "value": { "stringValue": &r.actor } },
                    { "key": "worpen.audit.workflow", "value": { "stringValue": &r.route_or_workflow } },
                    { "key": "worpen.audit.entry_hash", "value": { "stringValue": &r.entry_hash } },
                    { "key": "worpen.audit.prev_hash", "value": { "stringValue": &r.prev_hash } },
                    { "key": "worpen.audit.policy_decision", "value": { "stringValue": r.policy_decision.as_deref().unwrap_or("none") } },
                    { "key": "worpen.audit.details", "value": { "stringValue": r.details.to_string() } }
                ]
            })
        })
        .collect();

    json!({
        "resourceLogs": [
            {
                "resource": {
                    "attributes": [
                        { "key": "service.name", "value": { "stringValue": "worpen" } },
                        { "key": "service.version", "value": { "stringValue": "0.1.0" } }
                    ]
                },
                "scopeLogs": [
                    {
                        "scope": { "name": "worpen.audit" },
                        "logRecords": log_records
                    }
                ]
            }
        ]
    })
}
