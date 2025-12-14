pub mod adapters;
pub mod db;

pub use adapters::*;
pub use db::initialize_db;

pub fn hello() -> String {
    "Hello from infra".to_string()
}

