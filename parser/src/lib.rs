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
            Some(TokenType::Const)
            | Some(TokenType::Let)
            | Some(TokenType::Mut)
            | Some(TokenType::Pub) => Ok(Node::Stmt(Stmt::Decl(self.parse_decl()?))),
            Some(TokenType::Ident(_)) => {
                if self
                    .lexer
                    .clone()
                    .nth(1)
                    .map_or(false, |t| t.kind == TokenType::PlusEql)
                {
                    Ok(Node::Stmt(Stmt::Decl(self.parse_impl_decl()?)))
                } else {
                    // Assume it's an expression statement
                    let expr = self.parse_expr(0)?;
                    // Expect semicolon after expression statement
                    self.expect_and_consume(
                        TokenType::SemiColon,
                        "Expected semicolon after expression",
                    )?;
                    Ok(Node::Stmt(Stmt::ExprStmt(expr)))
                }
            }
            Some(TokenType::OBrack) | Some(TokenType::CBrack) => {
                self.lexer.next(); // Consume the brace
                self.next() // Recurse to get the next actual node
            }
            Some(_) => {
                // Assume it's an expression statement
                let expr = self.parse_expr(0)?;
                // Expect semicolon after expression statement
                self.expect_and_consume(
                    TokenType::SemiColon,
                    "Expected semicolon after expression",
                )?;
                Ok(Node::Stmt(Stmt::ExprStmt(expr)))
            }
            _ => Ok(Node::EOF), // Handle EOF when peek() returns None
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
            TokenType::Dot => {
                info!("Current token in parse_atom: Dot");
                info!("After Dot, peeked: {:?}", self.lexer.peek());
                if self
                    .lexer
                    .peek()
                    .map_or(false, |t| t.kind == TokenType::OBrack)
                {
                    self.parse_struct_literal()
                } else {
                    bail!("Unhandled token in parse_atom: Dot (not a struct literal)")
                }
            }
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
        self.expect_and_consume(TokenType::CParen, "Expected closing parenthesis")?;
        Ok(expr)
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
        self.expect_and_consume(TokenType::CBrack, "Expected closing brace")?;
        Ok(Expr::Block(Block(nodes)))
    }

    fn parse_type(&mut self) -> anyhow::Result<Type> {
        let type_token = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected type, found none"))?;
        match type_token.kind {
            TokenType::Ident(_) => Ok(Type::Identifier(type_token)),
            TokenType::OParen => {
                let mut elements = Vec::new();
                while self
                    .lexer
                    .peek()
                    .map_or(false, |t| t.kind != TokenType::CParen)
                {
                    elements.push(Box::new(self.parse_type()?));

                    match self.lexer.peek().map(|t| &t.kind) {
                        Some(TokenType::Comma) => {
                            self.lexer.next(); // Consume comma
                        }
                        Some(TokenType::CParen) => {
                            // Closing parenthesis, break loop
                            break;
                        }
                        t => bail!("Expected ',' or ')' after parameter, found {:?}", t),
                    }
                }

                self.expect_and_consume(TokenType::CParen, "Expected ')'")?;

                Ok(Type::Touple(elements))
            }
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
        self.expect_and_consume(TokenType::Colon, "Expected ':' after parameter name")?;

        let param_type = self.parse_type()?;

        Ok(Parameter { name, param_type })
    }

    fn parse_struct_definition(&mut self, name: Token) -> anyhow::Result<Struct> {
        // consume `struct`
        self.expect_and_consume(TokenType::Struct, "Expected 'struct'")?;

        // consume `=`
        self.expect_and_consume(TokenType::Eql, "Expected '='")?;

        // consume '{'
        self.expect_and_consume(TokenType::OBrack, "Expected '}")?;

        let mut members = Vec::new();
        while self
            .lexer
            .peek()
            .map_or(false, |t| t.kind != TokenType::CBrack)
        {
            members.push(self.parse_parameter()?);

            match self.lexer.peek().map(|t| &t.kind) {
                Some(TokenType::Comma) => {
                    self.lexer.next();
                }
                Some(TokenType::CBrack) => {
                    break;
                }
                t => bail!("Expected ',' or ')' after parameter, found {:?}", t),
            }
        }

        self.expect_and_consume(TokenType::CBrack, "Expected '}'")?;
        // self.expect_and_consume(TokenType::SemiColon, "Expected ';'")?;

        Ok(Struct {
            name: name,
            members: members,
        })
    }

    fn parse_function_definition(
        &mut self,
        visibility: Option<Token>,
        prot: Token,
        name: Token,
    ) -> anyhow::Result<Function> {
        // Consume `fn`
        self.expect_and_consume(TokenType::Fn, "Expected 'fn'")?;

        // Consume `(`
        self.expect_and_consume(TokenType::OParen, "Expected '('")?;

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
        self.expect_and_consume(TokenType::CParen, "Expected ')'")?;

        // Consume `->`
        self.expect_and_consume(TokenType::RightArrow, "Expected '->'")?;

        // Parse return type
        let return_type = self.parse_type()?;

        // Consume `=`
        self.expect_and_consume(TokenType::Eql, "Expected '='")?;

        // Parse function body (block expression)
        let body = self.parse_expr(0)?;

        Ok(Function {
            visibility,
            prot,
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_function_call(&mut self, callee: Token) -> anyhow::Result<Expr> {
        // Consume `(`
        self.expect_and_consume(TokenType::OParen, "Expected '('")?;

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
        self.expect_and_consume(TokenType::CParen, "Expected ')'")?;

        Ok(Expr::Call(Call { callee, args }))
    }

    fn parse_struct_literal(&mut self) -> anyhow::Result<Expr> {
        info!("Entering parse_struct_literal");
        // Consume `.`
        let dot_token = self.expect_and_consume(TokenType::Dot, "Expected '.'")?;
        info!("Consumed: {:?}", dot_token.kind);
        // Consume `{`
        let obrack_token = self.expect_and_consume(TokenType::OBrack, "Expected '{'")?;
        info!("Consumed: {:?}", obrack_token.kind);

        let mut fields = Vec::new();
        loop {
            info!(
                "Looping in parse_struct_literal. Peeked: {:?}",
                self.lexer.peek()
            );
            // If the next token is '}', we're done
            if self
                .lexer
                .peek()
                .map_or(false, |t| t.kind == TokenType::CBrack)
            {
                info!("Breaking loop: CBrack found");
                break;
            }

            // Consume `.` for field name
            let field_dot_token =
                self.expect_and_consume(TokenType::Dot, "Expected '.' for struct field")?;
            info!("Consumed: {:?}", field_dot_token.kind);
            // Consume field name
            let field_name = self
                .lexer
                .next()
                .ok_or(anyhow::anyhow!("Expected field name, found none"))?;
            info!("Consumed: {:?}", field_name.kind);
            match field_name.kind {
                TokenType::Ident(_) => {} // Correctly matches Ident
                _ => bail!(
                    "Expected identifier for field name, found {:?}",
                    field_name.kind
                ),
            }

            // Consume `=
            let eql_token =
                self.expect_and_consume(TokenType::Eql, "Expected '=' after field name")?;
            info!("Consumed: {:?}\n", eql_token.kind);

            // Parse field value
            let field_value = self.parse_expr(0)?;
            info!("Parsed field value: {:?}\n", field_value);

            fields.push((field_name, field_value));

            // Check for comma
            info!("Checking for comma. Peeked: {:?}\n", self.lexer.peek());
            if self
                .lexer
                .peek()
                .map_or(false, |t| t.kind == TokenType::Comma)
            {
                let comma_token = self.lexer.next().unwrap(); // Consume comma
                info!("Consumed: {:?}\n", comma_token.kind);
            } else if self
                .lexer
                .peek()
                .map_or(false, |t| t.kind == TokenType::CBrack)
            {
                // If no comma, and it's a closing brace, break
                info!("Breaking loop: CBrack found after field\n");
                break;
            } else {
                bail!("Expected ',' or '}}' after struct field");
            }
        }

        // Consume `}`
        let cbrack_token = self.expect_and_consume(TokenType::CBrack, "Expected '}'")?;
        info!("Consumed: {:?}\n", cbrack_token.kind);

        Ok(Expr::StructLiteral(StructLiteral { fields }))
    }

    fn parse_decl(&mut self) -> anyhow::Result<Decl> {
        let visibility = if self
            .lexer
            .peek()
            .map_or(false, |t| t.kind == TokenType::Pub)
        {
            Some(self.lexer.next().unwrap())
        } else {
            None
        };

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
        let next_token_kind = self
            .lexer
            .peek()
            .ok_or(anyhow::anyhow!("Expected token, found none"))?
            .kind
            .clone();

        let decl = match next_token_kind {
            TokenType::Colon => {
                self.lexer.next(); // Consume ':'
                let peeked_kind = self
                    .lexer
                    .peek()
                    .ok_or(anyhow::anyhow!("Expected token after ':', found none"))?
                    .kind
                    .clone();
                match peeked_kind {
                    TokenType::Fn => {
                        Decl::Function(self.parse_function_definition(visibility, prot, name)?)
                    }
                    TokenType::Struct => Decl::Struct(self.parse_struct_definition(name)?),
                    _ => {
                        // It's a variable declaration with a type hint
                        let type_hint = Some(self.parse_type()?);
                        // Consume `:=` or `=`
                        self.expect_and_consume(TokenType::Assign, "Expected Assign")?;
                        let expr = self.parse_expr(0)?;
                        Decl::Variable(Variable {
                            visibility,
                            prot,
                            name,
                            type_hint,
                            expr,
                        })
                    }
                }
            }
            TokenType::Assign | TokenType::Eql => {
                // It's a variable declaration without a type hint
                self.lexer.next(); // Consume `:=` or `=`
                let expr = self.parse_expr(0)?;
                Decl::Variable(Variable {
                    visibility,
                    prot,
                    name,
                    type_hint: None,
                    expr,
                })
            }
            _ => bail!("Expected ':' or '=', found {:?}", next_token_kind),
        };

        // expect `;`
        self.expect_and_consume(TokenType::SemiColon, "Expected Semicolon")?;

        Ok(decl)
    }

    fn parse_impl_decl(&mut self) -> anyhow::Result<Decl> {
        let name = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("Expected identifier for impl, found none"))?;
        match name.kind {
            TokenType::Ident(_) => {}
            t => bail!("Expected Ident for impl name, found {:?}", t),
        }

        self.expect_and_consume(TokenType::PlusEql, "Expected '+='")?;
        self.expect_and_consume(TokenType::Impl, "Expected 'impl'")?;
        self.expect_and_consume(TokenType::OBrack, "Expected '{'")?;

        let mut members = Vec::new();
        while self
            .lexer
            .peek()
            .map_or(false, |t| t.kind != TokenType::CBrack)
        {
            members.push(self.next()?);
        }

        self.expect_and_consume(TokenType::CBrack, "Expected '}'")?;
        self.expect_and_consume(TokenType::SemiColon, "Expected ';'")?;

        Ok(Decl::Impl(Impl {
            name,
            members: Block(members),
        }))
    }

    // Helper methods for token expectation and consumption
    fn expect(&mut self, expected_kind: TokenType, msg: &str) -> anyhow::Result<()> {
        let peeked_kind = self
            .lexer
            .peek()
            .ok_or(anyhow::anyhow!("{}, found none", msg))?;
        if peeked_kind.kind == expected_kind {
            Ok(())
        } else {
            bail!("{}, found {:?}", msg, peeked_kind.kind)
        }
    }

    fn expect_and_consume(&mut self, expected_kind: TokenType, msg: &str) -> anyhow::Result<Token> {
        info!("expect_and_consume: Expected {:?}", expected_kind);
        let token = self
            .lexer
            .next()
            .ok_or(anyhow::anyhow!("{}, found none", msg))?;
        info!("expect_and_consume: Consumed {:?}", token.kind);
        if token.kind == expected_kind {
            Ok(token)
        } else {
            bail!("{}, found {:?}", msg, token.kind)
        }
    }
}

fn get_precedence(token: &TokenType) -> u8 {
    match token {
        TokenType::Plus | TokenType::Minus => 1,
        TokenType::Star | TokenType::Div => 2,
        _ => 0,
    }
}
