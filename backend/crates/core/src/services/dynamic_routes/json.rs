use serde_json::Value;
use proto::models::DynamicRouteExecutionContext;
use super::utils::get_json_path;

pub fn handle_json_op(operation: &str, input: &str, args: &[Value], context: &DynamicRouteExecutionContext) -> Value {
    let input_val = if let Some(v) = context.variables.get(input) {
        v.clone()
    } else {
        serde_json::from_str(input).unwrap_or(Value::Null)
    };
    
    match operation {
        "stringify" => Value::String(serde_json::to_string(&input_val).unwrap_or_default()),
        "parse" => {
            if let Value::String(s) = input_val {
                serde_json::from_str(&s).unwrap_or(Value::Null)
            } else {
                Value::Null
            }
        },
        "get_path" => {
            let path = args.first().and_then(|v| v.as_str()).unwrap_or("");
            get_json_path(&input_val, path)
        },
        _ => Value::Null,
    }
}
