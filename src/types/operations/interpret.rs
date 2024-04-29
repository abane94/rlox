use crate::token_type::Token;
use crate::types::expr::Expr;
use crate::types::Literal;
use crate::types::stmt::Stmt;

pub fn assign(name: &Token, value: &Box<Expr>) -> Literal {
    todo!()
}

pub fn binary(left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Literal {
    todo!()
}

pub fn grouping(expression: &Box<Expr>) -> Literal {
    todo!()
}

pub fn literal(value: &Literal) -> Literal {
    todo!()
}

pub fn unary(operator: &Token, right: &Box<Expr>) -> Literal {
    todo!()
}

pub fn variable(name: &Token) -> Literal {
    todo!()
}

pub fn block(statements: &Vec<Stmt>) -> Literal {
    todo!()
}

pub fn expression(expression: &Box<Expr>) -> Literal {
    todo!()
}

pub fn print(expression: &Box<Expr>) -> Literal {
    todo!()
}

pub fn var(name: &Token, initializer: &Box<Expr>) -> Literal {
    todo!()
}
