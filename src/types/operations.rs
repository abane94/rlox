use crate::token_type::{Literal};
use crate::types::expr::Expr;
use crate::types::stmt::Stmt;
mod print;
mod interpret;
pub trait Print {
    fn print(&self) -> String;
}

pub trait Interpret {
    fn interpret(&self) -> Literal;
}


impl Print for Expr {
    fn print(&self) -> String {
        match self {
            Expr::Assign { name, value } => print::assign(name, value),
            Expr::Binary { left, operator, right } => print::binary(left, operator, right),
            Expr::Grouping { expression } => print::grouping(expression),
            Expr::Literal { value } => print::literal(value),
            Expr::Unary { operator, right } => print::unary(operator, right),
            Expr::Variable { name } => print::variable(name),
        }
    }
}

impl Print for Stmt {
    fn print(&self) -> String {
        match self {
            Stmt::Block { statements } => print::block(statements),
            Stmt::Expression { expression } => print::expression(expression),
            Stmt::Print { expression } => print::print(expression),
            Stmt::Var { name, initializer } => print::var(name, initializer),
        }
    }
}

impl Interpret for Expr {
    fn interpret(&self) -> Literal {
        match self {
            Expr::Assign { name, value } => interpret::assign(name, value),
            Expr::Binary { left, operator, right } => interpret::binary(left, operator, right),
            Expr::Grouping { expression } => interpret::grouping(expression),
            Expr::Literal { value } => interpret::literal(value),
            Expr::Unary { operator, right } => interpret::unary(operator, right),
            Expr::Variable { name } => interpret::variable(name),
        }
    }
}

impl Interpret for Stmt {
    fn interpret(&self) -> Literal {
        match self {
            Stmt::Block { statements } => interpret::block(statements),
            Stmt::Expression { expression } => interpret::expression(expression),
            Stmt::Print { expression } => interpret::print(expression),
            Stmt::Var { name, initializer } => interpret::var(name, initializer),
        }
    }
}
