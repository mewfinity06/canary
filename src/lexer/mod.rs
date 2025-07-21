use std::iter::Peekable;
use std::str::Chars;

pub mod token;

use crate::error;
use token::Token;

pub struct Lexer<'a> {
    file_name: String,
    content: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_name: String, content: &'a String) -> Self {
        Self {
            file_name: file_name,
            content: content.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> anyhow::Result<Token> {
        self.skip_whitespace_and_comments();

        let c = match self.content.peek() {
            Some(c) => *c,
            None => return Ok(Token::EOF),
        };

        let token = match c {
            '.' => {
                self.content.next();
                if self.content.peek() == Some(&'.') {
                    self.content.next();
                    if self.content.peek() == Some(&'.') {
                        self.content.next();
                        Token::DotDotDot
                    } else {
                        Token::Dot
                    }
                } else {
                    Token::Dot
                }
            }
            ':' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::Assign
                } else {
                    Token::Colon
                }
            }
            '+' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::PlusEql
                } else {
                    Token::Plus
                }
            }
            '-' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::MinusEql
                } else if self.content.peek() == Some(&'>') {
                    self.content.next();
                    Token::RightArrow
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::StarEql
                } else {
                    Token::Star
                }
            }
            '/' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::DivEql
                } else {
                    Token::Div
                }
            }
            '<' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::LessEql
                } else {
                    Token::Less
                }
            }
            '>' => {
                self.content.next();
                if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::GreaterEql
                } else {
                    Token::Greater
                }
            }
            '=' => {
                self.content.next();
                if self.content.peek() == Some(&'>') {
                    self.content.next();
                    Token::FatRightArrow
                } else if self.content.peek() == Some(&'=') {
                    self.content.next();
                    Token::DoubleEql
                } else {
                    Token::Eql
                }
            }
            '|' => {
                self.content.next();
                if self.content.peek() == Some(&'>') {
                    self.content.next();
                    Token::Pipe
                } else {
                    Token::VertBar
                }
            }

            ';' => {
                self.content.next();
                Token::SemiColon
            }
            ',' => {
                self.content.next();
                Token::Comma
            }
            '?' => {
                self.content.next();
                Token::Question
            }
            '!' => {
                self.content.next();
                Token::Bang
            }
            '#' => {
                self.content.next();
                Token::Pound
            }
            '(' => {
                self.content.next();
                Token::OParen
            }
            ')' => {
                self.content.next();
                Token::CParen
            }
            '{' => {
                self.content.next();
                Token::OBrack
            }
            '}' => {
                self.content.next();
                Token::CBrack
            }
            '[' => {
                self.content.next();
                Token::OSquare
            }
            ']' => {
                self.content.next();
                Token::CSquare
            }

            _ if c.is_alphabetic() || c == '_' => {
                let ident_str = self.read_ident()?;
                Token::from(ident_str.as_str())
            }
            _ if c.is_numeric() => {
                let number_str = self.read_number()?;
                Token::Number(number_str)
            }
            '"' => {
                let string_str = self.read_string()?;
                Token::String(string_str)
            }
            _ => {
                self.content.next();
                Token::Invalid(c)
            }
        };

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while self.content.peek().is_some_and(|c| c.is_whitespace()) {
            self.content.next();
        }
    }

    fn read_ident(&mut self) -> anyhow::Result<String> {
        let mut res = String::with_capacity(10);
        while let Some(&c) = self.content.peek() {
            if c.is_alphanumeric() || c == '_' {
                res.push(self.content.next().unwrap());
            } else {
                break;
            }
        }
        Ok(res)
    }

    fn read_number(&mut self) -> anyhow::Result<String> {
        let mut res = String::with_capacity(10);
        while let Some(&c) = self.content.peek() {
            if c.is_numeric() || c == '_' {
                res.push(self.content.next().unwrap());
            } else {
                break;
            }
        }
        Ok(res)
    }

    fn read_string(&mut self) -> anyhow::Result<String> {
        self.content.next();
        let mut res = String::with_capacity(10);
        while let Some(&c) = self.content.peek() {
            if c != '"' {
                res.push(self.content.next().unwrap());
            } else {
                self.content.next();
                break;
            }
        }
        Ok(res)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let mut skipped_something = false;
            let initial_len = self.content.clone().count();

            self.skip_whitespace();

            if self.content.clone().count() < initial_len {
                skipped_something = true;
            }

            if self.skip_comment() {
                skipped_something = true;
            }

            if !skipped_something {
                break;
            }
        }
    }

    fn skip_comment(&mut self) -> bool {
        let mut skipped = false;
        if self.content.peek() == Some(&'/') {
            self.content.next();
            match self.content.peek() {
                // Single-line comment
                Some(&'/') => {
                    self.content.next();
                    while let Some(c) = self.content.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                    skipped = true;
                }
                // Multi-line comment /* */
                Some(&'*') => {
                    self.content.next();
                    let mut found_end = false;
                    while let Some(c) = self.content.next() {
                        if c == '*' {
                            if self.content.peek() == Some(&'/') {
                                self.content.next();
                                found_end = true;
                                break;
                            }
                        }
                    }
                    if !found_end {
                        error!("Found unclosed multiline comment");
                    }
                    skipped = true;
                }
                _ => {}
            }
        }
        skipped
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(t) => Some(t),
            Err(e) => {
                error!("{e}");
                None
            }
        }
    }
}
