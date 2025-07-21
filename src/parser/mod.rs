use anyhow::{Result, bail};
use std::iter::Peekable;

pub mod node;

use crate::error;
use crate::lexer::{
    Lexer,
    token::{Token, TokenType},
};
use crate::parser::node::{
    Atom, Decl, ElseBranch, EnumVariant, Expr, ForLoopKind, FunctionBody, IfBranch, ImplBlock,
    InterfaceMember, MacroArg, MacroArm, MacroDefinition, Node, Stmt, StructField, SwitchCase,
    Type, WhileLoopKind,
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser.
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    /// Peeks at the next token without consuming it.
    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    /// Consumes and returns the next token.
    fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    /// Expects and consumes a specific token type, returning an error if it doesn't match.
    fn expect_and_consume(&mut self, expected: TokenType) -> Result<Token> {
        let token = self
            .next_token()
            .ok_or_else(|| anyhow::anyhow!("Unexpected EOF, expected {:?}", expected))?;
        if token.kind == expected {
            Ok(token)
        } else {
            bail!(
                "Expected {:?}, found {:?} at line {} col {}",
                expected,
                token.kind,
                token.loc.line,
                token.loc.col
            )
        }
    }

    /// Parses the entire program into a vector of nodes.
    pub fn parse_program(&mut self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();
        while self.peek().is_some() {
            nodes.push(self.parse_node()?);
        }
        Ok(nodes)
    }

    /// Parses a single node, which can be a statement or an expression.
    fn parse_node(&mut self) -> Result<Node> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenType::Const) => {
                let mut clone = self.lexer.clone();
                clone.next(); // const
                if let Some(token) = clone.next() {
                    if let TokenType::Ident(_) = token.kind {
                        if let Some(token) = clone.next() {
                            if token.kind == TokenType::Colon {
                                match clone.next().map(|t| t.kind) {
                                    Some(TokenType::Struct) => {
                                        return Ok(Node::Stmt(Stmt::StructDef(
                                            self.parse_struct_def()?,
                                        )));
                                    }
                                    Some(TokenType::Enum) => {
                                        return Ok(Node::Stmt(Stmt::EnumDef(
                                            self.parse_enum_def()?,
                                        )));
                                    }
                                    Some(TokenType::Interface) => {
                                        return Ok(Node::Stmt(Stmt::InterfaceDef(
                                            self.parse_interface_def()?,
                                        )));
                                    }
                                    Some(TokenType::Macro) => {
                                        return Ok(Node::Stmt(Stmt::MacroDef(
                                            self.parse_macro_def()?,
                                        )));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                Ok(Node::Stmt(Stmt::Decl(self.parse_decl()?)))
            }
            Some(TokenType::Val)
            | Some(TokenType::Mut)
            | Some(TokenType::Priv)
            | Some(TokenType::Pub)
            | Some(TokenType::Override) => Ok(Node::Stmt(Stmt::Decl(self.parse_decl()?))),
            Some(TokenType::If) => Ok(Node::Stmt(Stmt::If(self.parse_if_stmt()?))),
            Some(TokenType::Switch) => Ok(Node::Stmt(Stmt::Switch(self.parse_switch_stmt()?))),
            Some(TokenType::For) => Ok(Node::Stmt(Stmt::For(self.parse_for_stmt()?))),
            Some(TokenType::While) => Ok(Node::Stmt(Stmt::While(self.parse_while_stmt()?))),
            Some(TokenType::Return) => {
                self.next_token();
                let expr = if self.peek().map(|t| &t.kind) == Some(&TokenType::SemiColon) {
                    None
                } else {
                    Some(self.parse_expr()?)
                };
                self.expect_and_consume(TokenType::SemiColon)?;
                Ok(Node::Stmt(Stmt::Return(expr)))
            }
            Some(TokenType::Break) => {
                self.next_token();
                self.expect_and_consume(TokenType::SemiColon)?;
                Ok(Node::Stmt(Stmt::Break))
            }
            Some(TokenType::Continue) => {
                self.next_token();
                self.expect_and_consume(TokenType::SemiColon)?;
                Ok(Node::Stmt(Stmt::Continue))
            }
            Some(TokenType::Ident(_)) => {
                let mut clone = self.lexer.clone();
                clone.next(); // ident
                match clone.next().map(|t| t.kind) {
                    Some(TokenType::Bang) => {
                        let expr = self.parse_macro_call()?;
                        self.expect_and_consume(TokenType::SemiColon)?;
                        Ok(Node::Expr(expr))
                    }
                    Some(TokenType::PlusEql) => {
                        Ok(Node::Stmt(Stmt::ImplBlock(self.parse_impl_block()?)))
                    }
                    _ => {
                        let expr = self.parse_expr()?;
                        if self.peek().map(|t| &t.kind) == Some(&TokenType::SemiColon) {
                            self.next_token();
                        }
                        Ok(Node::Stmt(Stmt::Expr(expr)))
                    }
                }
            }
            None => Ok(Node::EOF),
            _ => {
                let expr = self.parse_expr()?;
                if self.peek().map(|t| &t.kind) == Some(&TokenType::SemiColon) {
                    self.next_token();
                }
                Ok(Node::Stmt(Stmt::Expr(expr)))
            }
        }
    }

    /// Parses a declaration statement.
    fn parse_decl(&mut self) -> Result<Decl> {
        let protection = self
            .next_token()
            .ok_or_else(|| anyhow::anyhow!("Expected protection keyword"))?;
        let name = self.expect_ident()?;
        let mut type_ = None;
        let special_type = matches!(
            protection.kind,
            TokenType::Struct | TokenType::Enum | TokenType::Interface
        );

        if self.peek().map(|t| &t.kind) == Some(&TokenType::Colon) {
            self.next_token();
            type_ = Some(self.parse_type()?);
        } else if self.peek().map(|t| &t.kind) == Some(&TokenType::Assign) {
            self.next_token();
        } else {
            bail!("Expected colon (:) or assign (:=)");
        }

        let expr = self.parse_expr()?;
        self.expect_and_consume(TokenType::SemiColon)?;

        Ok(Decl {
            special_type,
            protection,
            name,
            type_,
            expr,
        })
    }

    /// Parses a type annotation.
    fn parse_type(&mut self) -> Result<Type> {
        let mut is_optional = false;
        let mut generics = Vec::new();
        let base_type;

        if self.peek().map(|t| &t.kind) == Some(&TokenType::Fn) {
            self.next_token();
            self.expect_and_consume(TokenType::OParen)?;
            let mut params = Vec::new();
            while self.peek().map(|t| &t.kind) != Some(&TokenType::CParen) {
                let param_name = self.expect_ident()?;
                self.expect_and_consume(TokenType::Colon)?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                    self.next_token();
                }
            }
            self.expect_and_consume(TokenType::CParen)?;
            self.expect_and_consume(TokenType::RightArrow)?;
            let return_type = self.parse_type()?;
            base_type = Expr::FunctionType {
                params,
                return_type: Box::new(return_type),
            };
        } else {
            base_type = self.parse_expr()?;
        }

        if self.peek().map(|t| &t.kind) == Some(&TokenType::Less) {
            self.next_token();
            while self.peek().map(|t| &t.kind) != Some(&TokenType::Greater) {
                generics.push(self.expect_ident()?);
                if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                    self.next_token();
                }
            }
            self.expect_and_consume(TokenType::Greater)?;
        }

        if self.peek().map(|t| &t.kind) == Some(&TokenType::Question) {
            self.next_token();
            is_optional = true;
        }

        Ok(Type {
            base: base_type,
            generics,
            is_optional,
        })
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_binary_op_expr(0)
    }

    /// Parses a binary operation expression, handling operator precedence.
    fn parse_binary_op_expr(&mut self, min_precedence: u8) -> Result<Expr> {
        let mut left = self.parse_atom_or_call()?;

        while let Some(op_token) = self.peek().cloned() {
            let (precedence, associativity) = self.get_operator_precedence(&op_token.kind);

            if precedence < min_precedence {
                break;
            }

            let op = self.next_token().unwrap();

            let next_min_precedence = if associativity == Associativity::Left {
                precedence + 1
            } else {
                precedence
            };

            let right = self.parse_binary_op_expr(next_min_precedence)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Parses an atomic expression or a function call.
    fn parse_atom_or_call(&mut self) -> Result<Expr> {
        let mut expr = self.parse_atom()?;

        loop {
            match self.peek().map(|t| &t.kind) {
                Some(TokenType::OParen) => {
                    self.next_token();
                    let mut args = Vec::new();
                    while self.peek().map(|t| &t.kind) != Some(&TokenType::CParen) {
                        args.push(self.parse_expr()?);
                        if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                            self.next_token();
                        }
                    }
                    self.expect_and_consume(TokenType::CParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                Some(TokenType::Dot) => {
                    self.next_token();
                    let member = self.expect_ident()?;
                    expr = Expr::MemberAccess {
                        object: Box::new(expr),
                        member,
                    };
                }
                Some(TokenType::OSquare) => {
                    self.next_token();
                    let index = self.parse_expr()?;
                    self.expect_and_consume(TokenType::CSquare)?;
                    expr = Expr::IndexAccess {
                        array: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parses an atomic expression.
    fn parse_atom(&mut self) -> Result<Expr> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenType::Ident(_)) => Ok(Expr::Atom(Atom::Ident(self.expect_ident()?))),
            Some(TokenType::Number(_)) => Ok(Expr::Atom(Atom::Number(self.next_token().unwrap()))),
            Some(TokenType::String(_)) => Ok(Expr::Atom(Atom::String(self.next_token().unwrap()))),
            Some(TokenType::OParen) => {
                self.next_token();
                let expr = self.parse_expr()?;
                self.expect_and_consume(TokenType::CParen)?;
                Ok(Expr::Parenthesized(Box::new(expr)))
            }
            Some(TokenType::OBrack) => {
                self.next_token();
                let mut elements = Vec::new();
                while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
                    elements.push(self.parse_expr()?);
                    if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                        self.next_token();
                    }
                }
                self.expect_and_consume(TokenType::CBrack)?;
                Ok(Expr::ArrayLiteral(elements))
            }
            Some(TokenType::Dot) => {
                self.next_token();
                if self.peek().map(|t| &t.kind) == Some(&TokenType::OBrack) {
                    self.next_token();
                    let mut fields = Vec::new();
                    while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
                        self.expect_and_consume(TokenType::Dot)?;
                        let name = self.expect_ident()?;
                        self.expect_and_consume(TokenType::Eql)?;
                        let value = self.parse_expr()?;
                        fields.push((name, value));
                        if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                            self.next_token();
                        }
                    }
                    self.expect_and_consume(TokenType::CBrack)?;
                    Ok(Expr::StructLiteral(fields))
                } else if let Some(TokenType::Ident(_)) = self.peek().map(|t| &t.kind) {
                    let variant_name = self.expect_ident()?;
                    let mut value = None;
                    if self.peek().map(|t| &t.kind) == Some(&TokenType::OBrack) {
                        self.next_token();
                        value = Some(Box::new(self.parse_expr()?));
                        self.expect_and_consume(TokenType::CBrack)?;
                    }
                    Ok(Expr::EnumVariantLiteral {
                        variant_name,
                        value,
                    })
                } else {
                    bail!("Expected identifier or {{ after dot for literal")
                }
            }
            Some(TokenType::Unreachable) => {
                self.next_token();
                Ok(Expr::Unreachable)
            }
            _ => bail!("Expected an atom, found {:?}", self.peek()),
        }
    }

    /// Returns the precedence and associativity of an operator.
    fn get_operator_precedence(&self, token: &TokenType) -> (u8, Associativity) {
        match token {
            TokenType::Eql
            | TokenType::Assign
            | TokenType::PlusEql
            | TokenType::MinusEql
            | TokenType::StarEql
            | TokenType::DivEql => (1, Associativity::Right),
            TokenType::DoubleEql
            | TokenType::Less
            | TokenType::Greater
            | TokenType::LessEql
            | TokenType::GreaterEql => (2, Associativity::Left),
            TokenType::Plus | TokenType::Minus => (3, Associativity::Left),
            TokenType::Star | TokenType::Div => (4, Associativity::Left),
            _ => (0, Associativity::Left),
        }
    }

    /// Parses a macro definition.
    fn parse_macro_def(&mut self) -> Result<MacroDefinition> {
        self.expect_and_consume(TokenType::Const)?;
        let name = self.expect_ident()?;
        self.expect_and_consume(TokenType::Colon)?;
        self.expect_and_consume(TokenType::Macro)?;
        self.expect_and_consume(TokenType::Eql)?;
        self.expect_and_consume(TokenType::OBrack)?;

        let mut arms = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            arms.push(self.parse_macro_arm()?);
        }
        self.expect_and_consume(TokenType::CBrack)?;
        self.expect_and_consume(TokenType::SemiColon)?;

        Ok(MacroDefinition { name, arms })
    }

    /// Parses a macro arm.
    fn parse_macro_arm(&mut self) -> Result<MacroArm> {
        self.expect_and_consume(TokenType::OParen)?;
        let mut params = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CParen) {
            let name = self.expect_ident()?;
            self.expect_and_consume(TokenType::Colon)?;
            let type_ = self.expect_ident()?;
            params.push((name, type_));
            if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                self.next_token();
            }
        }
        self.expect_and_consume(TokenType::CParen)?;

        self.expect_and_consume(TokenType::Assign)?;
        self.expect_and_consume(TokenType::OBrack)?;
        let body = self.parse_function_body()?;
        self.expect_and_consume(TokenType::CBrack)?;
        self.expect_and_consume(TokenType::SemiColon)?;

        Ok(MacroArm { params, body })
    }

    /// Parses a macro call.
    fn parse_macro_call(&mut self) -> Result<Expr> {
        let name = self.expect_ident()?;
        self.expect_and_consume(TokenType::Bang)?;
        self.expect_and_consume(TokenType::OParen)?;
        let mut args = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CParen) {
            args.push(self.parse_macro_arg()?);
            if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                self.next_token();
            }
        }
        self.expect_and_consume(TokenType::CParen)?;
        Ok(Expr::MacroCall { name, args })
    }

    /// Parses a macro argument.
    fn parse_macro_arg(&mut self) -> Result<MacroArg> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenType::Pound) => {
                self.next_token();
                if self.peek().map(|t| &t.kind) == Some(&TokenType::OSquare) {
                    self.next_token();
                    self.expect_and_consume(TokenType::Pound)?;
                    let ident = self.expect_ident()?;
                    self.expect_and_consume(TokenType::Comma)?;
                    self.expect_and_consume(TokenType::CSquare)?;
                    self.expect_and_consume(TokenType::Star)?;
                    Ok(MacroArg::VariadicInterpolation(ident))
                } else {
                    Ok(MacroArg::InterpolatedIdent(self.expect_ident()?))
                }
            }
            Some(TokenType::String(_)) => Ok(MacroArg::String(self.next_token().unwrap())),
            _ => Ok(MacroArg::Expr(self.parse_expr()?)),
        }
    }

    /// Parses a struct definition.
    fn parse_struct_def(&mut self) -> Result<Decl> {
        self.expect_and_consume(TokenType::Const)?;
        let name = self.expect_ident()?;
        self.expect_and_consume(TokenType::Colon)?;
        let protection = self.expect_and_consume(TokenType::Struct)?;
        self.expect_and_consume(TokenType::Eql)?;
        self.expect_and_consume(TokenType::OBrack)?;

        let fields = self.parse_struct_body()?;
        self.expect_and_consume(TokenType::CBrack)?;
        self.expect_and_consume(TokenType::SemiColon)?;

        Ok(Decl {
            special_type: true,
            protection,
            name,
            type_: Some(Type {
                base: Expr::Struct(fields),
                generics: Vec::new(),
                is_optional: false,
            }),
            expr: Expr::Empty,
        })
    }

    /// Parses the body of a struct.
    fn parse_struct_body(&mut self) -> Result<Vec<StructField>> {
        let mut fields = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            let is_private = self.peek().map(|t| &t.kind) == Some(&TokenType::Priv);
            if is_private {
                self.next_token();
            }
            let field_name = self.expect_ident()?;
            let is_optional = self.peek().map(|t| &t.kind) == Some(&TokenType::Question);
            if is_optional {
                self.next_token();
            }

            let field_type = if self.peek().map(|t| &t.kind) == Some(&TokenType::Colon) {
                self.next_token();
                Some(self.parse_type()?)
            } else if self.peek().map(|t| &t.kind) == Some(&TokenType::Enum) {
                self.next_token();
                self.expect_and_consume(TokenType::Eql)?;
                self.expect_and_consume(TokenType::OBrack)?;
                let variants = self.parse_enum_body()?;
                self.expect_and_consume(TokenType::CBrack)?;
                Some(Type {
                    base: Expr::AnonEnum(variants),
                    generics: Vec::new(),
                    is_optional: false,
                })
            } else if self.peek().map(|t| &t.kind) == Some(&TokenType::Struct) {
                self.next_token();
                self.expect_and_consume(TokenType::Eql)?;
                self.expect_and_consume(TokenType::OBrack)?;
                let struct_fields = self.parse_struct_body()?;
                self.expect_and_consume(TokenType::CBrack)?;
                Some(Type {
                    base: Expr::AnonStruct(struct_fields),
                    generics: Vec::new(),
                    is_optional: false,
                })
            } else {
                None
            };
            fields.push(StructField {
                name: field_name,
                type_: field_type,
                is_optional,
                is_private,
            });
            if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                self.next_token();
            }
        }
        Ok(fields)
    }

    /// Parses an enum definition.
    fn parse_enum_def(&mut self) -> Result<Decl> {
        self.expect_and_consume(TokenType::Const)?;
        let name = self.expect_ident()?;
        self.expect_and_consume(TokenType::Colon)?;
        let protection = self.expect_and_consume(TokenType::Enum)?;

        let mut generics = Vec::new();
        if self.peek().map(|t| &t.kind) == Some(&TokenType::Less) {
            self.next_token();
            while self.peek().map(|t| &t.kind) != Some(&TokenType::Greater) {
                generics.push(self.expect_ident()?);
                if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                    self.next_token();
                }
            }
            self.expect_and_consume(TokenType::Greater)?;
        }

        self.expect_and_consume(TokenType::Eql)?;
        self.expect_and_consume(TokenType::OBrack)?;
        let variants = self.parse_enum_body()?;
        self.expect_and_consume(TokenType::CBrack)?;
        self.expect_and_consume(TokenType::SemiColon)?;

        Ok(Decl {
            special_type: true,
            protection,
            name,
            type_: Some(Type {
                base: Expr::Enum { variants, generics },
                generics: Vec::new(),
                is_optional: false,
            }),
            expr: Expr::Empty,
        })
    }

    /// Parses the body of an enum.
    fn parse_enum_body(&mut self) -> Result<Vec<EnumVariant>> {
        let mut variants = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            let variant_name = self.expect_ident()?;
            let variant_type = if self.peek().map(|t| &t.kind) == Some(&TokenType::Colon) {
                self.next_token();
                Some(self.parse_type()?)
            } else {
                None
            };
            variants.push(EnumVariant {
                name: variant_name,
                type_: variant_type,
            });
            if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                self.next_token();
            }
        }
        Ok(variants)
    }

    /// Parses an interface definition.
    fn parse_interface_def(&mut self) -> Result<Decl> {
        self.expect_and_consume(TokenType::Const)?;
        let name = self.expect_ident()?;
        self.expect_and_consume(TokenType::Colon)?;
        let protection = self.expect_and_consume(TokenType::Interface)?;
        self.expect_and_consume(TokenType::Eql)?;
        self.expect_and_consume(TokenType::OBrack)?;

        let mut members = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            let is_override = self.peek().map(|t| &t.kind) == Some(&TokenType::Override);
            if is_override {
                self.next_token();
            }
            self.expect_and_consume(TokenType::Const)?;
            let member_name = self.expect_ident()?;
            self.expect_and_consume(TokenType::Colon)?;
            self.expect_and_consume(TokenType::Fn)?;
            self.expect_and_consume(TokenType::OParen)?;
            let mut params = Vec::new();
            while self.peek().map(|t| &t.kind) != Some(&TokenType::CParen) {
                let param_name = self.expect_ident()?;
                self.expect_and_consume(TokenType::Colon)?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                    self.next_token();
                }
            }
            self.expect_and_consume(TokenType::CParen)?;
            self.expect_and_consume(TokenType::RightArrow)?;
            let return_type = self.parse_type()?;

            let body = if self.peek().map(|t| &t.kind) == Some(&TokenType::Eql) {
                self.next_token();
                self.expect_and_consume(TokenType::OBrack)?;
                let b = self.parse_function_body()?;
                self.expect_and_consume(TokenType::CBrack)?;
                Some(b)
            } else {
                None
            };
            self.expect_and_consume(TokenType::SemiColon)?;
            members.push(InterfaceMember {
                name: member_name,
                params,
                return_type,
                body,
                is_override,
            });
        }
        self.expect_and_consume(TokenType::CBrack)?;
        self.expect_and_consume(TokenType::SemiColon)?;

        Ok(Decl {
            special_type: true,
            protection,
            name,
            type_: Some(Type {
                base: Expr::Interface(members),
                generics: Vec::new(),
                is_optional: false,
            }),
            expr: Expr::Empty,
        })
    }

    /// Parses an implementation block.
    fn parse_impl_block(&mut self) -> Result<ImplBlock> {
        let target_name = self.expect_ident()?;
        self.expect_and_consume(TokenType::PlusEql)?;
        let impl_name = if let Some(token) = self.peek() {
            if let TokenType::Ident(_) = token.kind {
                Some(self.expect_ident()?)
            } else {
                None
            }
        } else {
            None
        };
        self.expect_and_consume(TokenType::OBrack)?;
        let mut members = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            members.push(self.parse_node()?.into_stmt()?);
        }
        self.expect_and_consume(TokenType::CBrack)?;
        self.expect_and_consume(TokenType::SemiColon)?;
        Ok(ImplBlock {
            target_name,
            impl_name,
            members,
        })
    }

    /// Parses the body of a function.
    fn parse_function_body(&mut self) -> Result<FunctionBody> {
        let mut statements = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            let node = self.parse_node()?;
            match node {
                Node::Stmt(stmt) => statements.push(stmt),
                Node::Expr(expr) => statements.push(Stmt::Expr(expr)),
                Node::EOF => break,
            }
        }
        Ok(FunctionBody { statements })
    }

    /// Parses an if statement.
    fn parse_if_stmt(&mut self) -> Result<IfBranch> {
        self.expect_and_consume(TokenType::If)?;
        self.expect_and_consume(TokenType::OParen)?;
        let condition = self.parse_expr()?;
        self.expect_and_consume(TokenType::CParen)?;

        let capture_body = if self.peek().map(|t| &t.kind) == Some(&TokenType::Colon) {
            self.next_token();
            self.expect_and_consume(TokenType::VertBar)?;
            let capture = self.expect_ident()?;
            self.expect_and_consume(TokenType::VertBar)?;
            Some(capture)
        } else {
            None
        };

        self.expect_and_consume(TokenType::OBrack)?;
        let body = self.parse_function_body()?;
        self.expect_and_consume(TokenType::CBrack)?;

        let else_branch = if self.peek().map(|t| &t.kind) == Some(&TokenType::Else) {
            self.next_token();
            if self.peek().map(|t| &t.kind) == Some(&TokenType::OBrack) {
                self.next_token();
                let else_body = self.parse_function_body()?;
                self.expect_and_consume(TokenType::CBrack)?;
                Some(ElseBranch::Block(else_body))
            } else if self.peek().map(|t| &t.kind) == Some(&TokenType::If) {
                Some(ElseBranch::If(Box::new(self.parse_if_stmt()?)))
            } else {
                bail!("Expected '{{' or 'if' after 'else'")
            }
        } else {
            None
        };

        Ok(IfBranch {
            condition: Box::new(condition),
            capture_body,
            body,
            else_branch,
        })
    }

    /// Parses a switch statement.
    fn parse_switch_stmt(&mut self) -> Result<Expr> {
        self.expect_and_consume(TokenType::Switch)?;
        self.expect_and_consume(TokenType::OParen)?;
        let condition = self.parse_expr()?;
        self.expect_and_consume(TokenType::CParen)?;
        self.expect_and_consume(TokenType::OBrack)?;

        let mut cases = Vec::new();
        while self.peek().map(|t| &t.kind) != Some(&TokenType::CBrack) {
            self.expect_and_consume(TokenType::Dot)?;
            let variant_name = self.expect_ident()?;
            self.expect_and_consume(TokenType::Colon)?;
            let capture = if self.peek().map(|t| &t.kind) == Some(&TokenType::VertBar) {
                self.next_token();
                let c = self.expect_ident()?;
                self.expect_and_consume(TokenType::VertBar)?;
                Some(c)
            } else {
                None
            };
            self.expect_and_consume(TokenType::FatRightArrow)?;
            self.expect_and_consume(TokenType::OBrack)?;
            let body = self.parse_function_body()?;
            self.expect_and_consume(TokenType::CBrack)?;
            cases.push(SwitchCase {
                variant_name,
                capture,
                body,
            });
            if self.peek().map(|t| &t.kind) == Some(&TokenType::Comma) {
                self.next_token();
            }
        }
        self.expect_and_consume(TokenType::CBrack)?;

        Ok(Expr::Switch {
            condition: Box::new(condition),
            cases,
        })
    }

    /// Parses a for loop.
    fn parse_for_stmt(&mut self) -> Result<Expr> {
        self.expect_and_consume(TokenType::For)?;
        self.expect_and_consume(TokenType::OParen)?;
        let mut kind = ForLoopKind::Simple;

        let mut init_decl = None;
        let mut condition = None;
        let mut update_expr = None;
        let mut iterable = None;
        let mut capture = None;

        if let Some(token) = self.peek() {
            if matches!(
                token.kind,
                TokenType::Mut | TokenType::Val | TokenType::Const
            ) {
                let decl_node = self.parse_decl()?;
                init_decl = Some(Box::new(decl_node));
                condition = Some(Box::new(self.parse_expr()?));
                self.expect_and_consume(TokenType::SemiColon)?;
                update_expr = Some(Box::new(self.parse_expr()?));
            } else {
                iterable = Some(Box::new(self.parse_expr()?));
                if self.peek().map(|t| &t.kind) == Some(&TokenType::Colon) {
                    self.next_token();
                    self.expect_and_consume(TokenType::VertBar)?;
                    capture = Some(self.expect_ident()?);
                    self.expect_and_consume(TokenType::VertBar)?;
                    kind = ForLoopKind::ForEach;
                }
            }
        }
        self.expect_and_consume(TokenType::CParen)?;

        self.expect_and_consume(TokenType::OBrack)?;
        let body = self.parse_function_body()?;
        self.expect_and_consume(TokenType::CBrack)?;

        Ok(Expr::For {
            kind,
            init_decl,
            condition,
            update_expr,
            iterable,
            capture,
            body,
        })
    }

    /// Parses a while loop.
    fn parse_while_stmt(&mut self) -> Result<Expr> {
        self.expect_and_consume(TokenType::While)?;
        self.expect_and_consume(TokenType::OParen)?;
        let condition = self.parse_expr()?;
        self.expect_and_consume(TokenType::CParen)?;

        let mut kind = WhileLoopKind::Simple;
        let mut capture = None;
        let mut update_body = None;

        if self.peek().map(|t| &t.kind) == Some(&TokenType::Colon) {
            self.next_token();
            if self.peek().map(|t| &t.kind) == Some(&TokenType::VertBar) {
                self.next_token();
                capture = Some(self.expect_ident()?);
                self.expect_and_consume(TokenType::VertBar)?;
                kind = WhileLoopKind::WhileEach;
            } else if self.peek().map(|t| &t.kind) == Some(&TokenType::OParen) {
                self.next_token();
                update_body = Some(Box::new(self.parse_expr()?));
                self.expect_and_consume(TokenType::CParen)?;
                kind = WhileLoopKind::WhileUpdate;
            } else {
                bail!("Expected '|' or '(' after ':' for while loop")
            }
        }

        self.expect_and_consume(TokenType::OBrack)?;
        let body = self.parse_function_body()?;
        self.expect_and_consume(TokenType::CBrack)?;

        Ok(Expr::While {
            kind,
            condition: Box::new(condition),
            capture,
            update_body,
            body,
        })
    }

    /// Expects and consumes an identifier token.
    fn expect_ident(&mut self) -> Result<Token> {
        match self.next_token() {
            Some(token) => {
                if let TokenType::Ident(_) = token.kind {
                    Ok(token)
                } else {
                    bail!(
                        "Expected an identifier, found {:?} at line {} col {}",
                        token.kind,
                        token.loc.line,
                        token.loc.col
                    )
                }
            }
            None => bail!("Expected an identifier, found EOF"),
        }
    }
}

#[derive(PartialEq)]
enum Associativity {
    Left,
    Right,
}

impl Iterator for Parser<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peek().is_none() {
            return None;
        }
        match self.parse_node() {
            Ok(Node::EOF) => None,
            Ok(n) => Some(n),
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }
}
