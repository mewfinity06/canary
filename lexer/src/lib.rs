#![allow(dead_code)]
use std::iter::Peekable;
use std::str::Chars;

pub mod token;

use crate::token::{Location, Token, TokenType};
use utils::error;

#[derive(Clone)]
pub struct Lexer<'a> {
    content: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a String) -> Self {
        Self {
            content: content.chars().peekable(),
            line: 1,
            col: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let next = self.content.next();
        if let Some(c) = next {
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.content.peek()
    }

    pub fn next_token(&mut self) -> anyhow::Result<Token> {
        self.skip_whitespace_and_comments();

        let loc = Location {
            line: self.line,
            col: self.col,
        };
        let c = match self.peek() {
            Some(c) => *c,
            None => {
                return Ok(Token {
                    kind: TokenType::EOF,
                    loc,
                });
            }
        };

        let kind = match c {
            '.' => {
                self.next_char();
                if self.peek() == Some(&'.') {
                    self.next_char();
                    if self.peek() == Some(&'.') {
                        self.next_char();
                        TokenType::DotDotDot
                    } else {
                        TokenType::Dot
                    }
                } else {
                    TokenType::Dot
                }
            }
            ':' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::Assign
                } else {
                    TokenType::Colon
                }
            }
            '+' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::PlusEql
                } else {
                    TokenType::Plus
                }
            }
            '-' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::MinusEql
                } else if self.peek() == Some(&'>') {
                    self.next_char();
                    TokenType::RightArrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::StarEql
                } else {
                    TokenType::Star
                }
            }
            '/' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::DivEql
                } else {
                    TokenType::Div
                }
            }
            '<' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::LessEql
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                self.next_char();
                if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::GreaterEql
                } else {
                    TokenType::Greater
                }
            }
            '=' => {
                self.next_char();
                if self.peek() == Some(&'>') {
                    self.next_char();
                    TokenType::FatRightArrow
                } else if self.peek() == Some(&'=') {
                    self.next_char();
                    TokenType::DoubleEql
                } else {
                    TokenType::Eql
                }
            }
            '|' => {
                self.next_char();
                if self.peek() == Some(&'>') {
                    self.next_char();
                    TokenType::Pipe
                } else {
                    TokenType::VertBar
                }
            }

            ';' => {
                self.next_char();
                TokenType::SemiColon
            }
            ',' => {
                self.next_char();
                TokenType::Comma
            }
            '?' => {
                self.next_char();
                TokenType::Question
            }
            '!' => {
                self.next_char();
                TokenType::Bang
            }
            '#' => {
                self.next_char();
                TokenType::Pound
            }
            '(' => {
                self.next_char();
                TokenType::OParen
            }
            ')' => {
                self.next_char();
                TokenType::CParen
            }
            '{' => {
                self.next_char();
                TokenType::OBrack
            }
            '}' => {
                self.next_char();
                TokenType::CBrack
            }
            '[' => {
                self.next_char();
                TokenType::OSquare
            }
            ']' => {
                self.next_char();
                TokenType::CSquare
            }

            _ if c.is_alphabetic() || c == '_' => {
                let ident_str = self.read_ident()?;
                TokenType::from(ident_str.as_str())
            }
            _ if c.is_numeric() => {
                let number_str = self.read_number()?;
                TokenType::Number(number_str)
            }
            '"' => {
                let string_str = self.read_string()?;
                TokenType::String(string_str)
            }
            _ => {
                self.next_char();
                TokenType::Invalid(c)
            }
        };

        Ok(Token { kind, loc })
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.next_char();
        }
    }

    fn read_ident(&mut self) -> anyhow::Result<String> {
        let mut res = String::with_capacity(10);
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                res.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        Ok(res)
    }

    fn read_number(&mut self) -> anyhow::Result<String> {
        let mut res = String::with_capacity(10);
        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '_' {
                res.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
        Ok(res)
    }

    fn read_string(&mut self) -> anyhow::Result<String> {
        self.next_char();
        let mut res = String::with_capacity(10);
        while let Some(&c) = self.peek() {
            if c != '"' {
                res.push(self.next_char().unwrap());
            } else {
                self.next_char();
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
        if self.peek() == Some(&'/') {
            let mut content_clone = self.content.clone();
            content_clone.next();
            match content_clone.peek() {
                // Single-line comment
                Some(&'/') => {
                    self.next_char();
                    self.next_char();
                    while let Some(c) = self.next_char() {
                        if c == '\n' {
                            break;
                        }
                    }
                    skipped = true;
                }
                // Multi-line comment /* */
                Some(&'*') => {
                    self.next_char();
                    self.next_char();
                    let mut found_end = false;
                    while let Some(c) = self.next_char() {
                        if c == '*' {
                            if self.peek() == Some(&'/') {
                                self.next_char();
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
            Ok(t) if t.kind == TokenType::EOF => None, // EOF
            Ok(t) => Some(t),
            Err(e) => {
                error!("{e}");
                None
            }
        }
    }
}
