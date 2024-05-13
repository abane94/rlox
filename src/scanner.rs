use core::panic;

use crate::token_type::{Token, Literal, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        // let b = UnicodeSegmentation::graphemes(&source, true).collect::<Vec<&str>>();
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                break;
            }

            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), Literal::None(), self.line));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let n = self.line;
        let chr = self.advance();
        match chr {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),

            '!' => self.add_match_or(&'=', TokenType::BANG_EQUAL, TokenType::BANG),
            '=' => self.add_match_or(&'=', TokenType::EQUAL_EQUAL, TokenType::EQUAL ),
            '<' => self.add_match_or(&'=', TokenType::LESS_EQUAL, TokenType::LESS),
            '>' => self.add_match_or(&'=', TokenType::GREATER_EQUAL, TokenType::GREATER),

            '/' => {
                if self.peek().is_some_and(|c| c == &'/') {
                    let _ = self.advance();
                    while !self.is_at_end() && self.peek().is_some_and(|c|c != &'\n') {  // TODO: is_at_end check might be redundant here
                        let _ = self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }

            },

            ' ' | '\r' | '\t'  => {},
            '\n' => self.line += 1,

            '"' => self.string(),


            x => {
                if x.is_ascii_digit() {
                    self.number()
                } else if x.is_alphabetic() || x == &'_' {
                    self.identifier();
                } else {
                    panic!("{} Unexpected character: {}", n, x)
                }
            }
        }
    }

    fn advance(&mut self) -> &char {
        let char = self.source.get(self.current);
        self.current += 1;
        char.unwrap()
    }

    fn add_token(&mut self, tt: TokenType) {
        self.add_value_token(tt, Literal::None())
    }

    fn add_value_token(&mut self, tt: TokenType, literal: Literal) {
        let chars = self.source.get(self.start..self.current).unwrap();
        let text = String::from_iter(chars);
        self.tokens.push(Token::new(tt, text, literal, self.line))
    }

    fn matches(&mut self, expected: &char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.get(self.current).is_some_and(|x| x == expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_match_or(&mut self, chr: &char, matches: TokenType, no_match: TokenType) {
        if self.peek().is_some_and(|c| c == chr) {
            let _ = self.advance();
            self.add_token(matches);
        }
        else {
            self.add_token(no_match)
        }
    }

    fn peek(&self) -> Option<&char> {
        if self.is_at_end() {
            return None;
        }
        self.source.get(self.current)
    }

    fn string(&mut self) {
        while self.peek().is_some_and(|c| c != &'"') && !self.is_at_end() { // is_at_end check might be redundant
            if self.peek().is_some_and(|c| c == &'\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string literal");
        }

        self.advance(); // consume the ending "

        let str = String::from_iter(self.source.get((self.start + 1)..(self.current - 1)).unwrap());
        self.add_value_token(TokenType::STRING, Literal::String(str));

    }

    fn number(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek().is_some_and(|c| c == &'.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            // comsume the .
            self.advance();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let str = String::from_iter(self.source.get((self.start)..(self.current)).unwrap());
        let num: f32 = str.parse().unwrap();
        self.add_value_token(TokenType::NUMBER, Literal::Number(num));
    }

    fn peek_next(&self) -> Option<&char> {
        if self.current + 1 >= self.source.len() {
            return None;
        }
        return self.source.get(self.current + 1)
    }

    fn identifier(&mut self) {
        while let Some(c) = self.peek() && (c.is_alphabetic() || c == &'_') {
            self.advance();
        }
        let str = String::from_iter(self.source.get((self.start)..(self.current)).unwrap());
        let t_type_opt = Scanner::keyword(&str);
        match t_type_opt {
            Some(t_type) => self.add_token(t_type),
            None => self.add_token(TokenType::IDENTIFIER)
        }
    }

    fn keyword(word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            _ => None
        }
    }
}


