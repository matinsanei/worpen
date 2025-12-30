use serde_json::Value;
use chrono::Utc;

pub fn handle_date_op(operation: &str) -> Value {
    match operation {
        "now" => Value::String(Utc::now().to_rfc3339()),
        "timestamp" => Value::Number(Utc::now().timestamp().into()),
        "format" => {
            // TODO: Implement date formatting
            Value::String(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())
        },
        _ => Value::Null,
    }
}
