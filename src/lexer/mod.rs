use std::iter::Peekable;
use std::str::Chars;

pub mod token;

use token::Token;

use anyhow::anyhow;

pub struct Lexer<'a> {
    file_name: String,
    content: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    const KEYWORDS: &'a [&'a str] = &[
        "const",
        "val",
        "mut",
        "struct",
        "enum",
        "macro",
        "impl",
        "interface",
        "priv",
        "pub",
        "override",
        "fn",
        "Self",
        "self",
        "defer",
        "if",
        "else",
        "switch",
        "for",
        "break",
        "continue",
        "unreachable",
    ];

    pub fn new(file_name: String, content: &'a String) -> Self {
        Self {
            file_name: file_name,
            content: content.chars().peekable(),
        }
    }

    pub fn next(&mut self) -> anyhow::Result<Token> {
        self.skip_whitespace();

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
                    Token::Invalid(c)
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
                if Self::is_keyword(&ident_str) {
                    Token::Keyword(ident_str)
                } else {
                    Token::Ident(ident_str)
                }
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

    fn is_keyword(needle: &str) -> bool {
        for &keyword in Self::KEYWORDS {
            if needle == keyword {
                return true;
            }
        }
        false
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
            if c.is_numeric() {
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
}
