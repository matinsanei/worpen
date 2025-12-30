use std::sync::Arc;
use proto::models::LogicOperation;

/// A cached execution plan for a dynamic route.
/// This structure holds a pre-processed, cheap-to-clone version of the route's logic.
/// By storing `Arc<Vec<LogicOperation>>`, we avoid deep cloning the entire logic tree
/// on every execution, which significantly improves performance for hot routes.
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    /// The route logic, wrapped in Arc for cheap cloning.
    pub logic: Arc<Vec<LogicOperation>>,
    /// Whether the route is enabled.
    pub enabled: bool,
    /// Route version for invalidation checks (optional, good practice).
    pub version: String,
}

impl ExecutionPlan {
    /// Create a new execution plan from a list of logic operations.
    pub fn new(logic: Vec<LogicOperation>, enabled: bool, version: String) -> Self {
        Self {
            logic: Arc::new(logic),
            enabled,
            version,
        }
    }
}
