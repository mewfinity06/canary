use std::fmt::{self, Debug};
use lexer::token::Token;

pub struct Program(pub Vec<Node>);

impl Program {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.0 {
            writeln!(f, "{:?}", node)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Stmt(Stmt),
    Expr(Expr),
    EOF,
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Stmt(stmt) => write!(f, "{:?}", stmt),
            Node::Expr(expr) => write!(f, "{:?}", expr),
            Node::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    Decl(Decl),
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Decl(decl) => write!(f, "{:?}", decl),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Atom(Token),
    BinOp(Box<BinOp>),
    UnaryOp,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(token) => write!(f, "{:?}", token.kind),
            Expr::BinOp(binop) => write!(f, "{:?}", binop),
            Expr::UnaryOp => write!(f, "UnaryOp"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BinOp {
    pub op: Token,
    pub lhs: Expr,
    pub rhs: Expr,
}

impl Debug for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?} {:?} {:?})", self.lhs, self.op.kind, self.rhs)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Decl {
    Variable(Variable),
    Fn,
    Struct,
    Enum,
    Interface,
    Macro,
}

impl Debug for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::Variable(var) => write!(f, "{:?}", var),
            _ => write!(f, "Other Decl"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Variable {
    pub prot: Token,
    pub name: Token,
    pub type_hint: Option<Type>,
    pub expr: Expr,
}

impl Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?} {:?} := {:?})",
            self.prot.kind,
            self.name.kind,
            self.expr
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {}