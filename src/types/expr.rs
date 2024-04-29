use crate::token_type::{Token,Literal};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Expr {
    Assign{name: Token, value: Box<Expr>},
    Binary{left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping{expression: Box<Expr>},
    Literal{value: Literal},
    Unary{operator: Token, right: Box<Expr>},
    Variable{name: Token},
}
