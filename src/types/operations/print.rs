use crate::token_type::Token;
use crate::types::expr::{Expr};
use crate::types::Literal;
use crate::types::stmt::Stmt;

use super::Print;

pub fn assign(name: &Token, value: &Box<Expr>) -> String {
    todo!()
}

pub fn binary(left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> String {
    parenthesize(&operator.lexeme, vec![left, right])
}

pub fn grouping(expression: &Box<Expr>) -> String {
    parenthesize("grouping", vec![expression])
}

pub fn literal(value: &Literal) -> String {
    match value {
        Literal::String(s) => s.to_string(),
        Literal::Number(n) => n.to_string(),
        Literal::Boolean(true) => "true".to_string(),
        Literal::Boolean(false) => "false".to_string(),
        Literal::None() => "nil".to_string(),
    }
}

pub fn unary(operator: &Token, right: &Box<Expr>) -> String {
    parenthesize(&operator.lexeme, vec![&right])
}

pub fn variable(name: &Token) -> String {
    todo!()
}

pub fn block(statements: &Vec<Stmt>) -> String {
    todo!()
}

pub fn expression(expression: &Box<Expr>) -> String {
    todo!()
}

pub fn print(expression: &Box<Expr>) -> String {
    todo!()
}

pub fn var(name: &Token, initializer: &Box<Expr>) -> String {
    todo!()
}



fn parenthesize(name: &str, exprs: Vec<&Expr>) -> String {
    let ls: String = exprs.iter().map(|expr| expr.print()).collect::<Vec<String>>().join(" ");
    format!("( {} {} )", name, ls)
}
