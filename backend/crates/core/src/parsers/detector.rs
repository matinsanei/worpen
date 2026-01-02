/// Format detection for Dynamic Routes
/// Detects whether input is JSON or YAML

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Json,
    Yaml,
}

/// Detect format from content string
/// 
/// # Examples
/// 
/// ```
/// use worpen_core::parsers::detector::{detect_format, InputFormat};
/// 
/// let json = r#"{"name": "test"}"#;
/// assert_eq!(detect_format(json), InputFormat::Json);
/// 
/// let yaml = "name: test";
/// assert_eq!(detect_format(yaml), InputFormat::Yaml);
/// ```
pub fn detect_format(content: &str) -> InputFormat {
    let trimmed = content.trim_start();
    
    // JSON objects start with { or arrays with [
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        InputFormat::Json
    } else {
        // Everything else treated as YAML
        InputFormat::Yaml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_json_object() {
        let content = r#"{"name": "test", "value": 123}"#;
        assert_eq!(detect_format(content), InputFormat::Json);
    }

    #[test]
    fn test_detect_json_array() {
        let content = r#"[{"id": 1}, {"id": 2}]"#;
        assert_eq!(detect_format(content), InputFormat::Json);
    }

    #[test]
    fn test_detect_json_with_whitespace() {
        let content = "  \n  {\n    \"name\": \"test\"\n  }";
        assert_eq!(detect_format(content), InputFormat::Json);
    }

    #[test]
    fn test_detect_yaml_simple() {
        let content = "name: test\nvalue: 123";
        assert_eq!(detect_format(content), InputFormat::Yaml);
    }

    #[test]
    fn test_detect_yaml_with_dash() {
        let content = "- name: test\n- value: 123";
        assert_eq!(detect_format(content), InputFormat::Yaml);
    }

    #[test]
    fn test_detect_yaml_key_only() {
        let content = "name:";
        assert_eq!(detect_format(content), InputFormat::Yaml);
    }

    #[test]
    fn test_detect_yaml_with_comment() {
        let content = "# This is a comment\nname: test";
        assert_eq!(detect_format(content), InputFormat::Yaml);
    }
}
