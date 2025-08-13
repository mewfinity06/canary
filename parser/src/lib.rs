#![allow(dead_code, unused_imports)]
use std::iter::Peekable;

use anyhow::bail;

use lexer::Lexer;
use lexer::token::{Token, TokenType};
use utils::*;

pub mod node;

use node::*;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn program(&mut self) -> anyhow::Result<Program> {
        let mut program = Program::new();

        while let x = self.next()? {
            if x == Node::EOF { break }
            program.0.push(x);
        }

        Ok(program)
    }

    fn next(&mut self) -> anyhow::Result<Node> {
        match self.lexer.peek() {
            Some(t) if t.kind == TokenType::EOF => Ok(Node::EOF),
            Some(t)
                if t.kind == TokenType::Const
                    || t.kind == TokenType::Let
                    || t.kind == TokenType::Mut =>
            {
                Ok(Node::Stmt(Stmt::Decl(self.parse_decl()?)))
            }
            Some(t) => bail!("Unhandled token: {:?}", t),
            None => Ok(Node::EOF),
        }
    }

    fn parse_expr(&mut self, precedence: u8) -> anyhow::Result<Expr> {
        let mut lhs = self.parse_atom()?;

        while let Some(op) = self.lexer.peek() {
            let op_precedence = get_precedence(&op.kind);
            if op_precedence == 0 || op_precedence <= precedence {
                break;
            }

            let op = self.lexer.next().unwrap();
            let rhs = self.parse_expr(op_precedence)?;

            lhs = Expr::BinOp(Box::new(BinOp {
                op,
                lhs,
                rhs,
            }));
        }

        Ok(lhs)
    }

    fn parse_atom(&mut self) -> anyhow::Result<Expr> {
        match &self.lexer.peek().ok_or(anyhow::anyhow!("Expected token, found none"))?.kind {
            TokenType::Number(_) => {
                let token = self.lexer.next().unwrap();
                Ok(Expr::Atom(token))
            }
            TokenType::OParen => self.parse_paren(),
            t => bail!("Unhandled token in parse_atom: {:?}", t),
        }
    }

    fn parse_paren(&mut self) -> anyhow::Result<Expr> {
        // Consume `(`.
        self.lexer.next();
        let expr = self.parse_expr(0)?;
        // Consume `)`.
        match self.lexer.peek().ok_or(anyhow::anyhow!("Expected token, found none"))?.kind {
            TokenType::CParen => {
                self.lexer.next();
                Ok(expr)
            }
            _ => bail!("Expected closing parenthesis"),
        }
    }

    fn parse_decl(&mut self) -> anyhow::Result<Decl> {
        // Will be Const|Let|Mut
        let prot = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?;

        let name = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?;

        match name.kind {
            TokenType::Ident(_) => {}
            t => bail!("Expected Ident, found {:?}", t),
        }

        let type_hint: Option<Type> = match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?
            .kind
        {
            // `:=`
            TokenType::Assign => None,
            // `:` <TYPE> `=`
            TokenType::Colon => bail!("Known type is unhandled at this point"),
            t => bail!("Expected Assign or Colon, found {:?}", t),
        };

        let expr = self.parse_expr(0)?;

        // expect `;`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?
            .kind
        {
            TokenType::SemiColon => {}
            t => bail!("Expected Semicolon, found {:?}", t),
        }

        Ok(Decl::Variable(Variable {
            prot,
            name,
            type_hint,
            expr,
        }))
    }
}

fn get_precedence(token: &TokenType) -> u8 {
    match token {
        TokenType::Plus | TokenType::Minus => 1,
        TokenType::Star | TokenType::Div => 2,
        _ => 0,
    }
}