// use crate::token_type::Token;

// mod test_mod;

// // operations.rs
// trait Interpret {
//     fn interpret(&self) -> i32;
// }

// trait Print {
//     fn print(&self) -> &str;
// }

// trait ASTNode: Interpret + Print {}


// // types.rs
// // struct Node {}

// enum Expr {
//     Assign{name: Token, value: Box<Expr>},
//     Binary{left: Box<Expr>, operator: Token, right: Box<Expr>}
// }

// impl ASTNode for Expr {}



// // print.rs
// impl Print for Expr {
//     fn print(&self) -> &str {
//         match self {
//             Expr::Assign { name, value } => todo!(),
//             Expr::Binary { left, operator, right } => todo!(),
//         }
//     }
// }


// // interpret.rs
// impl Interpret for Expr {
//     fn interpret(&self) -> i32 {
//         5
//     }
// }

