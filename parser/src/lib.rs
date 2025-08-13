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
            if x == Node::EOF {
                break;
            }
            program.0.push(x);
        }

        Ok(program)
    }

    fn next(&mut self) -> anyhow::Result<Node> {
        let peeked_kind = self.lexer.peek().map(|t| &t.kind);

        match peeked_kind {
            Some(TokenType::EOF) => Ok(Node::EOF),
            Some(TokenType::Const) | Some(TokenType::Let) | Some(TokenType::Mut) => {
                Ok(Node::Stmt(Stmt::Decl(self.parse_decl()?)))
            }
            Some(TokenType::OBrack) | Some(TokenType::CBrack) => {
                self.lexer.next(); // Consume the brace
                self.next() // Recurse to get the next actual node
            }
            Some(_) => {
                // Assume it's an expression statement
                let expr = self.parse_expr(0)?;
                // Expect semicolon after expression statement
                match self
                    .lexer
                    .next()
                    .ok_or(anyhow::anyhow!(
                        "Expected semicolon after expression, found none"
                    ))?
                    .kind
                {
                    TokenType::SemiColon => {}
                    t => bail!("Expected semicolon after expression, found {:?}", t),
                }
                Ok(Node::Stmt(Stmt::ExprStmt(expr)))
            }
            None => Ok(Node::EOF), // Handle EOF when peek() returns None
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

            lhs = Expr::BinOp(Box::new(BinOp { op, lhs, rhs }));
        }

        Ok(lhs)
    }

    fn parse_atom(&mut self) -> anyhow::Result<Expr> {
        let token = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?;

        match token.kind {
            TokenType::Number(_) => Ok(Expr::Atom(token)),
            TokenType::String(_) => Ok(Expr::Atom(token)), // Added for string literals
            TokenType::OParen => self.parse_paren(),
            TokenType::OBrack => self.parse_block_expr(),
            TokenType::Ident(_) => {
                // If the next token is '(', it's a function call
                if self
                    .lexer
                    .peek()
                    .map_or(false, |t| t.kind == TokenType::OParen)
                {
                    self.parse_function_call(token) // Pass the identifier as the callee
                } else {
                    // Otherwise, it's just an identifier atom
                    Ok(Expr::Atom(token))
                }
            }
            t => bail!("Unhandled token in parse_atom: {:?}", t),
        }
    }

    fn parse_paren(&mut self) -> anyhow::Result<Expr> {
        // Consume `(`.
        self.lexer.next();
        let expr = self.parse_expr(0)?;
        // Consume `)`.
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?
            .kind
        {
            TokenType::CParen => {
                self.lexer.next();
                Ok(expr)
            }
            _ => bail!("Expected closing parenthesis"),
        }
    }

    fn parse_block_expr(&mut self) -> anyhow::Result<Expr> {
        // Consume `{`.
        self.lexer.next();
        let mut nodes = Vec::new();
        while let Some(token) = self.lexer.peek() {
            if token.kind == TokenType::CBrack {
                break;
            }
            nodes.push(self.next()?);
        }
        // Consume `}`.
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?
            .kind
        {
            TokenType::CBrack => Ok(Expr::Block(Block(nodes))),
            _ => bail!("Expected closing brace"),
        }
    }

    fn parse_type(&mut self) -> anyhow::Result<Type> {
        let type_token = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected type, found none"))?;
        match type_token.kind {
            TokenType::Ident(_) => Ok(Type::Identifier(type_token)),
            _ => bail!("Expected identifier for type, found {:?}", type_token.kind),
        }
    }

    fn parse_parameter(&mut self) -> anyhow::Result<Parameter> {
        let name = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected parameter name, found none"))?;
        match name.kind {
            TokenType::Ident(_) => {}
            _ => bail!(
                "Expected identifier for parameter name, found {:?}",
                name.kind
            ),
        }

        // Consume ':'
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!(
                "Expected ':' after parameter name, found none"
            ))?
            .kind
        {
            TokenType::Colon => {}
            _ => bail!("Expected ':' after parameter name"),
        }

        let param_type = self.parse_type()?;

        Ok(Parameter { name, param_type })
    }

    fn parse_function_definition(&mut self, prot: Token, name: Token) -> anyhow::Result<Function> {
        // Consume `fn`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected 'fn', found none"))?
            .kind
        {
            TokenType::Fn => {}
            t => bail!("Expected 'fn', found {:?}", t),
        }

        // Consume `(`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected '(', found none"))?
            .kind
        {
            TokenType::OParen => {}
            t => bail!("Expected '(', found {:?}", t),
        }

        let mut params = Vec::new();
        // Parse parameters until `)`
        while self
            .lexer
            .peek()
            .map_or(false, |t| t.kind != TokenType::CParen)
        {
            params.push(self.parse_parameter()?);

            // Check for comma or closing parenthesis
            match self.lexer.peek().map(|t| &t.kind) {
                Some(TokenType::Comma) => {
                    self.lexer.next(); // Consume comma
                }
                Some(TokenType::CParen) => {
                    // Closing parenthesis, break loop
                    break;
                }
                _ => bail!("Expected ',' or ')' after parameter"),
            }
        }

        // Consume `)`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected ')', found none"))?
            .kind
        {
            TokenType::CParen => {}
            t => bail!("Expected ')', found {:?}", t),
        }

        // Consume `->`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected '->', found none"))?
            .kind
        {
            TokenType::RightArrow => {}
            t => bail!("Expected '->', found {:?}", t),
        }

        // Parse return type
        let return_type = self.parse_type()?;

        // Consume `=`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected '=', found none"))?
            .kind
        {
            TokenType::Eql => {}
            t => bail!("Expected '=', found {:?}", t),
        }

        // Parse function body (block expression)
        let body = match self.parse_block_expr()? {
            Expr::Block(block) => block,
            _ => bail!("Expected a block for function body"),
        };

        Ok(Function {
            prot,
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_function_call(&mut self, callee: Token) -> anyhow::Result<Expr> {
        // Consume `(`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected '(', found none"))?
            .kind
        {
            TokenType::OParen => {}
            t => bail!("Expected '(', found {:?}", t),
        }

        let mut args = Vec::new();
        // Parse arguments until `)`
        while self
            .lexer
            .peek()
            .map_or(false, |t| t.kind != TokenType::CParen)
        {
            args.push(self.parse_expr(0)?);

            // Check for comma or closing parenthesis
            match self.lexer.peek().map(|t| &t.kind) {
                Some(TokenType::Comma) => {
                    self.lexer.next(); // Consume comma
                }
                Some(TokenType::CParen) => {
                    // Closing parenthesis, break loop
                    break;
                }
                _ => bail!("Expected ',' or ')' after argument"),
            }
        }

        // Consume `)`
        match self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected ')', found none"))?
            .kind
        {
            TokenType::CParen => {}
            t => bail!("Expected ')', found {:?}", t),
        }

        Ok(Expr::Call(Call { callee, args }))
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

        // Check for type hint or function definition
        let next_token_kind = &self
            .lexer
            .peek()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?
            .kind;

        let decl = match next_token_kind {
            TokenType::Colon => {
                self.lexer.next(); // Consume ':'
                let peeked_kind = &self
                    .lexer
                    .peek()
                    .ok_or(anyhow::anyhow!("Expected token after ':', found none"))?
                    .kind;
                match peeked_kind {
                    TokenType::Fn => Decl::Function(self.parse_function_definition(prot, name)?),
                    _ => {
                        // It's a variable declaration with a type hint
                        let type_hint = Some(self.parse_type()?);
                        // Consume `:=` or `=`
                        match self
                            .lexer
                            .next()
                            .ok_or(anyhow::anyhow!("Expected Assign, found none"))?
                            .kind
                        {
                            TokenType::Assign => {}
                            t => bail!("Expected Assign, found {:?}", t),
                        };
                        let expr = self.parse_expr(0)?;
                        Decl::Variable(Variable {
                            prot,
                            name,
                            type_hint,
                            expr,
                        })
                    }
                }
            }
            TokenType::Assign => {
                // It's a variable declaration without a type hint
                self.lexer.next(); // Consume `:=`
                let expr = self.parse_expr(0)?;
                Decl::Variable(Variable {
                    prot,
                    name,
                    type_hint: None,
                    expr,
                })
            }
            t => bail!("Expected ':' or '=', found {:?}", t),
        };

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

        Ok(decl)
    }
}

fn get_precedence(token: &TokenType) -> u8 {
    match token {
        TokenType::Plus | TokenType::Minus => 1,
        TokenType::Star | TokenType::Div => 2,
        _ => 0,
    }
}
