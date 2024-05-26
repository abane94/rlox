use crate::token_type::Token;
use crate::types::expr::Expr;
use crate::types::stmt::Stmt;
use crate::types::Literal;

use super::Interpret;

#[derive(Debug)]
pub struct RuntimeError(pub String);

pub fn assign(name: &Token, value: &Box<Expr>) -> Literal {
    todo!()
}

pub fn binary(left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Literal {
    todo!()
}

pub fn grouping(expression: &Box<Expr>) -> Literal {
    return expression.interpret();
}

pub fn literal(value: &Literal) -> Literal {
    todo!()
}

pub fn unary(operator: &Token, _right: &Box<Expr>) -> Literal {
    let right = _right.interpret();
    match operator.token_type {
        token_type::TokenType::MINUS => match right {
            Literal::String(_) => RuntimeError("Invalid Cast: Cannot negate String"),
            Literal::Number(num) => -num,
            Literal::Boolean(_) => RuntimeError("Invalid Cast: Cannot negate Boolean, use '!'"),
            Literal::None() => RuntimeError("Invalid Cast: Cannot negate Empty value"),
        },
        _ => !panic("INvalid unary token"),
    }
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
