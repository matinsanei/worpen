use serde_json::Value;
use proto::models::DynamicRouteExecutionContext;
use super::utils::resolve_string;

pub fn handle_string_op(operation: &str, input: &str, args: &[Value], context: &DynamicRouteExecutionContext) -> Value {
    let input_str = resolve_string(input, context);
    
    match operation {
        "split" => {
            let delimiter = args.first().and_then(|v| v.as_str()).unwrap_or(",");
            let parts: Vec<Value> = input_str.split(delimiter).map(|s| Value::String(s.to_string())).collect();
            Value::Array(parts)
        },
        "join" => {
            let delimiter = args.first().and_then(|v| v.as_str()).unwrap_or("");
            if let Some(Value::Array(arr)) = context.variables.get(input) {
                let joined = arr.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join(delimiter);
                Value::String(joined)
            } else {
                Value::String(input_str)
            }
        },
        "upper" => Value::String(input_str.to_uppercase()),
        "lower" => Value::String(input_str.to_lowercase()),
        "trim" => Value::String(input_str.trim().to_string()),
        "replace" => {
            let from = args.first().and_then(|v| v.as_str()).unwrap_or("");
            let to = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
            Value::String(input_str.replace(from, to))
        },
        _ => Value::Null,
    }
}
