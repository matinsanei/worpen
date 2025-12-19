// Expression evaluator - evaluates AST to produce values
// Phase 2 Day 7-8: Implementation pending

use crate::expression::ast::Expr;
use serde_json::Value;
use std::collections::HashMap;

pub struct Evaluator {
    variables: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    pub fn with_variables(variables: HashMap<String, Value>) -> Self {
        Self { variables }
    }
    
    pub fn evaluate(&self, expr: &Expr) -> Result<Value, String> {
        // TODO: Phase 2 Day 7-8 - Full implementation
        Err("Evaluator not yet implemented".to_string())
    }
    
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evaluator_creation() {
        let evaluator = Evaluator::new();
        assert_eq!(evaluator.variables.len(), 0);
    }
    
    #[test]
    fn test_evaluator_with_variables() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Value::Number(42.into()));
        
        let evaluator = Evaluator::with_variables(vars);
        assert_eq!(evaluator.variables.len(), 1);
    }
}
