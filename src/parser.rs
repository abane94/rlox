use std::{error::Error, net::AddrParseError};

use crate::{
    token_type::{Literal, Token, TokenType},
    types::expr::Expr,
};

use std::fmt;

#[derive(Debug)]
pub struct ParseError(pub String);

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

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        return self.expression();
        // TODO: add error handling once the production functions have it
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();

        while let Some(_) = self._match(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            });
        }

        return expr;
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();

        while let Some(_) = self._match(&[TokenType::GREATER]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();
        while let Some(_) = self._match(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();
        while let Some(_) = self._match(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(_) = self._match(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let mtch = self._match(&[TokenType::FALSE, TokenType::TRUE, TokenType::NIL]);
        if let Some(typ) = mtch {
            match typ {
                TokenType::FALSE => Ok(Expr::Literal {
                    value: Literal::Boolean(false),
                }),
                TokenType::NIL => Ok(Expr::Literal {
                    value: Literal::None(),
                }),
                TokenType::TRUE => Ok(Expr::Literal {
                    value: Literal::Boolean(true),
                }),
                _ => Err(ParseError("Random match".to_owned())),
            }
        } else if let Some(_) = self._match(&[TokenType::NUMBER, TokenType::STRING]) {
            Ok(Expr::Literal {
                value: self.previous().literal,
            })
        } else if let Some(_) = self._match(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression")?;
            Ok(Expr::Grouping {
                expression: Box::new(expr),
            })
        } else {
            Err(ParseError("Un matched primary".to_owned()))
        }
        // TODO: add error handling, this method will return a Result, withc means, all above will
        // be the same
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
        if self.isAtEnd() {
            return false;
        }
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.isAtEnd() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {}
            }
            self.advance();
        }
    }

    //     comparison
    // previous
    // comparison
}
