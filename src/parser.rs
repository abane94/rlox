use std::{error::Error, net::AddrParseError};

use crate::{token_type::{Token, TokenType}, types::expr::Expr};

use std::fmt;

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while let Some(_) = self._match(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while let Some(_) = self._match(&[TokenType::GREATER]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right)};
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while let Some(_) = self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right)};
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while let Some(_) = self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary { left:  Box::new(expr), operator, right: Box::new(right)};
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if let Some(_) = self._match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary { operator, right: Box::new(right) }
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        let mtch =  self._match(&[TokenType::FALSE, TokenType::TRUE, TokenType::NIL]);
        if let Some(typ) = mtch  {
            match typ {
                TokenType::FALSE => todo!(),
                TokenType::NIL => todo!(),
                TokenType::TRUE => todo!(),
                _ => panic!("Random match")
            }
        } else if let Some(_) = self._match(&[TokenType::NUMBER, TokenType::STRING]) {
            Expr::Literal { value: self.previous().literal }
        } else if let Some(_) = self._match(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression");
            Expr::Grouping { expression: Box::new(expr) }
        } else {
            panic!("Un matched primary")
        }
    }



    // helpers

    fn _match(&mut self, types: &[TokenType]) -> Option<TokenType> {
        // types.iter().any(|typ: TokenType| {
        //     if (this.check(typ)) {

        //     }
        // })

        for typ in types {
            if self.check(typ) {
                self.advance();
                return Some(typ.to_owned());
            }
        }
        return None;
    }

    fn check(&mut self, typ: &TokenType) -> bool {
        if self.isAtEnd() {return false;}
        return &self.peek().token_type == typ;
    }

    fn advance(&mut self) -> Token {
        if !self.isAtEnd() {
            self.current += 1;
        }
        return self.previous();
    }

    fn isAtEnd(&mut self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> Token {
        self.tokens.get(self.current - 1).unwrap().to_owned()
    }

    fn consume(&mut self, typ: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(&typ) {
            return Result::Ok(self.advance());
        }
        let token = self.peek().to_owned();
        Result::Err(self.error(token, message.to_owned()))

    }

    fn error(&mut self, token: Token, message: String) -> ParseError {
        // Lox.error(token, message)
        ParseError(message)
    }

//     comparison
// previous
// comparison


}
