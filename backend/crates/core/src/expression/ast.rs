// Abstract Syntax Tree for expressions
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr<'a> {
    // Literals
    Number(f64),
    #[serde(borrow)]
    String(Cow<'a, str>),
    Boolean(bool),
    Null,
    
    // Variable reference: {{variable_name}}
    #[serde(borrow)]
    Variable(Cow<'a, str>),
    
    // Binary operations: left op right
    Binary {
        left: Box<Expr<'a>>,
        op: BinaryOp,
        right: Box<Expr<'a>>,
    },
    
    // Unary operations: op expr
    Unary {
        op: UnaryOp,
        expr: Box<Expr<'a>>,
    },
    
    // Function call: function_name(arg1, arg2, ...)
    Call {
        #[serde(borrow)]
        name: Cow<'a, str>,
        args: Vec<Expr<'a>>,
    },
    
    // Pipe operation: expr | filter_name | filter_name
    Pipe {
        expr: Box<Expr<'a>>,
        filters: Vec<PipeFilter<'a>>,
    },
    
    // Ternary: condition ? true_expr : false_expr
    Ternary {
        condition: Box<Expr<'a>>,
        true_expr: Box<Expr<'a>>,
        false_expr: Box<Expr<'a>>,
    },
    
    // Array: [expr1, expr2, ...]
    Array(Vec<Expr<'a>>),
    
    // Object: {key1: value1, key2: value2}
    Object(Vec<(Cow<'a, str>, Expr<'a>)>),
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
pub struct PipeFilter<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub args: Vec<Expr<'a>>,
}

impl<'a> Expr<'a> {
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
    pub fn variables(&self) -> Vec<Cow<'a, str>> {
        let mut vars = Vec::new();
        self.collect_variables(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }
    
    fn collect_variables(&self, vars: &mut Vec<Cow<'a, str>>) {
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

    /// Convert to an owned expression with 'static lifetime
    pub fn into_owned(self) -> Expr<'static> {
        match self {
            Expr::Number(n) => Expr::Number(n),
            Expr::String(s) => Expr::String(Cow::Owned(s.into_owned())),
            Expr::Boolean(b) => Expr::Boolean(b),
            Expr::Null => Expr::Null,
            Expr::Variable(name) => Expr::Variable(Cow::Owned(name.into_owned())),
            Expr::Binary { left, op, right } => Expr::Binary {
                left: Box::new(left.into_owned()),
                op,
                right: Box::new(right.into_owned()),
            },
            Expr::Unary { op, expr } => Expr::Unary {
                op,
                expr: Box::new(expr.into_owned()),
            },
            Expr::Call { name, args } => Expr::Call {
                name: Cow::Owned(name.into_owned()),
                args: args.into_iter().map(|a| a.into_owned()).collect(),
            },
            Expr::Pipe { expr, filters } => Expr::Pipe {
                expr: Box::new(expr.into_owned()),
                filters: filters.into_iter().map(|f| f.into_owned()).collect(),
            },
            Expr::Ternary { condition, true_expr, false_expr } => Expr::Ternary {
                condition: Box::new(condition.into_owned()),
                true_expr: Box::new(true_expr.into_owned()),
                false_expr: Box::new(false_expr.into_owned()),
            },
            Expr::Array(items) => Expr::Array(items.into_iter().map(|i| i.into_owned()).collect()),
            Expr::Object(pairs) => Expr::Object(
                pairs.into_iter()
                    .map(|(k, v)| (Cow::Owned(k.into_owned()), v.into_owned()))
                    .collect()
            ),
        }
    }
}

impl<'a> PipeFilter<'a> {
    pub fn into_owned(self) -> PipeFilter<'static> {
        PipeFilter {
            name: Cow::Owned(self.name.into_owned()),
            args: self.args.into_iter().map(|a| a.into_owned()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_constant() {
        assert!(Expr::Number(42.0).is_constant());
        assert!(Expr::String(Cow::Borrowed("hello")).is_constant());
        assert!(!Expr::Variable(Cow::Borrowed("x")).is_constant());
        
        let binary = Expr::Binary {
            left: Box::new(Expr::Number(1.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };
        assert!(binary.is_constant());
        
        let binary_with_var = Expr::Binary {
            left: Box::new(Expr::Variable(Cow::Borrowed("x"))),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };
        assert!(!binary_with_var.is_constant());
    }
    
    #[test]
    fn test_variables() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Variable(Cow::Borrowed("x"))),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable(Cow::Borrowed("y"))),
        };
        
        let vars = expr.variables();
        assert_eq!(vars, vec![Cow::Borrowed("x"), Cow::Borrowed("y")]);
    }
    
    #[test]
    fn test_variables_deduplicated() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Variable(Cow::Borrowed("x"))),
            op: BinaryOp::Add,
            right: Box::new(Expr::Variable(Cow::Borrowed("x"))),
        };
        
        let vars = expr.variables();
        assert_eq!(vars, vec![Cow::Borrowed("x")]);
    }
}
