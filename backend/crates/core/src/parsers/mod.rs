/// Parsers module for Dynamic Routes Engine
/// 
/// This module provides parsing functionality for route definitions
/// in both JSON and YAML formats with automatic format detection.

pub mod detector;
pub mod route_parser;

// Re-export commonly used items
pub use detector::{detect_format, InputFormat};
pub use route_parser::{parse_route, parse_route_with_format};
