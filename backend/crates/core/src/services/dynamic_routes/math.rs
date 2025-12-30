use serde_json::Value;
use proto::models::DynamicRouteExecutionContext;
use super::utils::resolve_variables;

pub fn handle_math_op(operation: &str, args: &[Value], context: &DynamicRouteExecutionContext) -> Value {
    // Resolve variables in args first
    let resolved_args: Vec<Value> = args.iter()
        .map(|v| resolve_variables(v, context))
        .collect();
    
    let numbers: Vec<f64> = resolved_args.iter()
        .filter_map(|v| match v {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.parse().ok(),
            _ => None,
        })
        .collect();
    
    match operation {
        "sum" => Value::Number(serde_json::Number::from_f64(numbers.iter().sum()).unwrap()),
        "avg" => {
            let avg = numbers.iter().sum::<f64>() / numbers.len() as f64;
            Value::Number(serde_json::Number::from_f64(avg).unwrap())
        },
        "min" => {
            let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
            Value::Number(serde_json::Number::from_f64(min).unwrap())
        },
        "max" => {
            let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            Value::Number(serde_json::Number::from_f64(max).unwrap())
        },
        "round" => {
            let val = numbers.first().unwrap_or(&0.0).round();
            Value::Number(serde_json::Number::from_f64(val).unwrap())
        },
        "floor" => {
            let val = numbers.first().unwrap_or(&0.0).floor();
            Value::Number(serde_json::Number::from_f64(val).unwrap())
        },
        "ceil" => {
            let val = numbers.first().unwrap_or(&0.0).ceil();
            Value::Number(serde_json::Number::from_f64(val).unwrap())
        },
        "abs" => {
            let val = numbers.first().unwrap_or(&0.0).abs();
            Value::Number(serde_json::Number::from_f64(val).unwrap())
        },
        "pow" => {
            let base = numbers.first().unwrap_or(&0.0);
            let exp = numbers.get(1).unwrap_or(&1.0);
            Value::Number(serde_json::Number::from_f64(base.powf(*exp)).unwrap())
        },
        "sqrt" => {
            let val = numbers.first().unwrap_or(&0.0).sqrt();
            Value::Number(serde_json::Number::from_f64(val).unwrap())
        },
        "subtract" => {
            if numbers.len() >= 2 {
                Value::Number(serde_json::Number::from_f64(numbers[0] - numbers[1]).unwrap())
            } else {
                Value::Null
            }
        },
        "multiply" => {
            let product = numbers.iter().product();
            Value::Number(serde_json::Number::from_f64(product).unwrap())
        },
        "divide" => {
            if numbers.len() >= 2 && numbers[1] != 0.0 {
                Value::Number(serde_json::Number::from_f64(numbers[0] / numbers[1]).unwrap())
            } else {
                Value::Null
            }
        },
        "mod" => {
            if numbers.len() >= 2 && numbers[1] != 0.0 {
                Value::Number(serde_json::Number::from_f64(numbers[0] % numbers[1]).unwrap())
            } else {
                Value::Null
            }
        },
        _ => Value::Null,
    }
}
