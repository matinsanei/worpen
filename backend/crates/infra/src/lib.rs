pub mod adapters;

pub use adapters::*;

pub fn hello() -> String {
    "Hello from infra".to_string()
}

