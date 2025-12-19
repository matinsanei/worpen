// Abstract Syntax Tree for expressions
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    
    // Variable reference: {{variable_name}}
    Variable(String),
    
    // Binary operations: left op right
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    
    // Unary operations: op expr
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    
    // Function call: function_name(arg1, arg2, ...)
    Call {
        name: String,
        args: Vec<Expr>,
    },
    
    // Pipe operation: expr | filter_name | filter_name
    Pipe {
        expr: Box<Expr>,
        filters: Vec<PipeFilter>,
    },
    
    // Ternary: condition ? true_expr : false_expr
    Ternary {
        condition: Box<Expr>,
        true_expr: Box<Expr>,
        false_expr: Box<Expr>,
    },
    
    // Array: [expr1, expr2, ...]
    Array(Vec<Expr>),
    
    // Object: {key1: value1, key2: value2}
    Object(Vec<(String, Expr)>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Arithmetic
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Pow,      // **
    
    // Comparison
    Eq,       // ==
    Ne,       // !=
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=
    
    // Logical
    And,      // &&
    Or,       // ||
    
    // String
    Concat,   // + (for strings)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,      // !
    Neg,      // -
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipeFilter {
    pub name: String,
    pub args: Vec<Expr>,
}

impl Expr {
    /// Check if expression is a constant (no variables)
    pub fn is_constant(&self) -> bool {
        match self {
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Null => true,
            Expr::Variable(_) => false,
            Expr::Binary { left, right, .. } => left.is_constant() && right.is_constant(),
            Expr::Unary { expr, .. } => expr.is_constant(),
            Expr::Call { args, .. } => args.iter().all(|a| a.is_constant()),
            Expr::Pipe { expr, filters } => {
                expr.is_constant() && filters.iter().all(|f| f.args.iter().all(|a| a.is_constant()))
            }
            Expr::Ternary { condition, true_expr, false_expr } => {
                condition.is_constant() && true_expr.is_constant() && false_expr.is_constant()
            }
            Expr::Array(items) => items.iter().all(|i| i.is_constant()),
            Expr::Object(pairs) => pairs.iter().all(|(_, v)| v.is_constant()),
        }
    }
    
    /// Get all variable names used in this expression
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_variables(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }
    
    fn collect_variables(&self, vars: &mut Vec<String>) {
        match self {
            Expr::Variable(name) => vars.push(name.clone()),
            Expr::Binary { left, right, .. } => {
                left.collect_variables(vars);
                right.collect_variables(vars);
            }
            Expr::Unary { expr, .. } => expr.collect_variables(vars),
            Expr::Call { args, .. } => {
                for arg in args {
                    arg.collect_variables(vars);
                }
            }
            Expr::Pipe { expr, filters } => {
                expr.collect_variables(vars);
                for filter in filters {
                    for arg in &filter.args {
                        arg.collect_variables(vars);
                    }
                }
            }
            Expr::Ternary { condition, true_expr, false_expr } => {
                condition.collect_variables(vars);
                true_expr.collect_variables(vars);
                false_expr.collect_variables(vars);
            }
            Expr::Array(items) => {
                for item in items {
                    item.collect_variables(vars);
                }
            }
            Expr::Object(pairs) => {
                for (_, value) in pairs {
                    value.collect_variables(vars);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_constant() {
        assert!(Expr::Number(42.0).is_constant());
        assert!(Expr::String("hello".to_string()).is_constant());
        assert!(!Expr::Variable("x".to_string()).is_constant());
        
        let binary = Expr::Binary {
            left: Box::new(Expr::Number(1.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };
        assert!(binary.is_constant());
        
        let binary_with_var = Expr::Binary {
            left: Box::new(Expr::Variable("x".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };
        assert!(!binary_with_var.is_constant());
    }
    
    #[test]
    fn test_variables() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Variable("x".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable("y".to_string())),
        };
        
        let vars = expr.variables();
        assert_eq!(vars, vec!["x", "y"]);
    }
    
    #[test]
    fn test_variables_deduplicated() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Variable("x".to_string())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable("x".to_string())),
        };
        
        let vars = expr.variables();
        assert_eq!(vars, vec!["x"]);
    }
}
