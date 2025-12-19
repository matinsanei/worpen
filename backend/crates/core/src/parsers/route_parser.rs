/// Route parser with support for both JSON and YAML formats

use proto::models::RegisterRouteRequest;
use crate::parsers::detector::{detect_format, InputFormat};

/// Parse route definition from JSON or YAML string
/// 
/// Automatically detects format and parses accordingly.
/// 
/// # Arguments
/// 
/// * `content` - Raw JSON or YAML string
/// 
/// # Returns
/// 
/// * `Ok(RegisterRouteRequest)` - Successfully parsed route
/// * `Err(String)` - Parse error with detailed message
/// 
/// # Examples
/// 
/// ```
/// let json = r#"{"name":"test","path":"/test","method":"GET","logic":[]}"#;
/// let route = parse_route(json).unwrap();
/// assert_eq!(route.name, "test");
/// ```
pub fn parse_route(content: &str) -> Result<RegisterRouteRequest, String> {
    let format = detect_format(content);
    
    match format {
        InputFormat::Json => {
            serde_json::from_str::<RegisterRouteRequest>(content)
                .map_err(|e| format!("JSON parse error: {}", e))
        }
        InputFormat::Yaml => {
            serde_yaml::from_str::<RegisterRouteRequest>(content)
                .map_err(|e| format!("YAML parse error: {}", e))
        }
    }
}

/// Parse route definition with explicit format
/// 
/// Use this when you know the format in advance (e.g., from Content-Type header)
pub fn parse_route_with_format(
    content: &str,
    format: InputFormat,
) -> Result<RegisterRouteRequest, String> {
    match format {
        InputFormat::Json => {
            serde_json::from_str::<RegisterRouteRequest>(content)
                .map_err(|e| format!("JSON parse error: {}", e))
        }
        InputFormat::Yaml => {
            serde_yaml::from_str::<RegisterRouteRequest>(content)
                .map_err(|e| format!("YAML parse error: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proto::models::HttpMethod;

    #[test]
    fn test_parse_simple_json() {
        let json = r#"{
            "name": "test_route",
            "path": "/api/test",
            "method": "GET",
            "logic": [],
            "parameters": [],
            "enabled": true,
            "version": "1.0.0"
        }"#;
        
        let route = parse_route(json).unwrap();
        assert_eq!(route.name, "test_route");
        assert_eq!(route.path, "/api/test");
        assert!(matches!(route.method, HttpMethod::GET));
    }

    #[test]
    fn test_parse_simple_yaml() {
        let yaml = r#"
name: test_route
path: /api/test
method: GET
logic: []
parameters: []
enabled: true
version: "1.0.0"
"#;
        
        let route = parse_route(yaml).unwrap();
        assert_eq!(route.name, "test_route");
        assert_eq!(route.path, "/api/test");
        assert!(matches!(route.method, HttpMethod::GET));
    }

    #[test]
    fn test_parse_yaml_with_description() {
        let yaml = r#"
name: echo_api
description: Returns your input
path: /api/echo
method: POST
logic: []
parameters: []
enabled: true
version: "1.0.0"
"#;
        
        let route = parse_route(yaml).unwrap();
        assert_eq!(route.name, "echo_api");
        assert_eq!(route.description, "Returns your input");
        assert!(matches!(route.method, HttpMethod::POST));
        assert_eq!(route.logic.len(), 0);
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid = r#"{"name": "test", invalid json}"#;
        let result = parse_route(invalid);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("JSON parse error"));
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let invalid = "name: test\n  invalid: : yaml";
        let result = parse_route(invalid);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("YAML parse error"));
    }

    #[test]
    fn test_parse_with_explicit_format() {
        let json = r#"{"name":"test","path":"/test","method":"GET","logic":[],"parameters":[],"enabled":true,"version":"1.0.0"}"#;
        
        let route = parse_route_with_format(json, InputFormat::Json).unwrap();
        assert_eq!(route.name, "test");
    }

    #[test]
    fn test_parse_yaml_multiline() {
        let yaml = r#"
name: multi_line_test
description: |
  This is a multi-line
  description for testing
  YAML features
path: /api/multiline
method: PUT
logic: []
parameters: []
enabled: true
version: "1.0.0"
"#;
        
        let route = parse_route(yaml).unwrap();
        assert_eq!(route.name, "multi_line_test");
        assert!(route.description.contains("multi-line"));
    }

    #[test]
    fn test_parse_json_with_comments_fails() {
        // JSON doesn't support comments, should fail
        let json_with_comment = r#"{
            // This is a comment
            "name": "test"
        }"#;
        
        let result = parse_route(json_with_comment);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_yaml_with_comments_succeeds() {
        let yaml = r#"
# This is a YAML comment
name: test_with_comment
# Another comment
path: /api/test
method: GET
logic: []
parameters: []
enabled: true
version: "1.0.0"
"#;
        
        let route = parse_route(yaml).unwrap();
        assert_eq!(route.name, "test_with_comment");
    }
}
