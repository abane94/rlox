use crate::token_type::{Token};
use crate::types::expr::Expr;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Stmt {
    Block{statements: Vec<Stmt>},
    Expression{expression: Box<Expr>},
    Print{expression: Box<Expr>},
    Var{name: Token, initializer: Box<Expr>},
}
