use lexer::token::Token;
use std::fmt::{self, Debug};

pub struct Program(pub Vec<Node>);

impl Program {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.0 {
            writeln!(f, "{:#?}", node)?;
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
            Node::Stmt(stmt) => write!(f, "{:#?}", stmt),
            Node::Expr(expr) => write!(f, "{:#?}", expr),
            Node::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    Decl(Decl),
    ExprStmt(Expr), // Added for expression statements
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Decl(decl) => write!(f, "{:#?}", decl),
            Stmt::ExprStmt(expr) => write!(f, "{:#?}", expr), // Debug for ExprStmt
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Atom(Token),
    BinOp(Box<BinOp>),
    UnaryOp,
    Block(Block),
    Call(Call),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(token) => write!(f, "{:?}", token.kind),
            Expr::BinOp(binop) => write!(f, "{:#?}", binop),
            Expr::UnaryOp => write!(f, "UnaryOp"),
            Expr::Block(block) => write!(f, "{:#?}", block),
            Expr::Call(call) => write!(f, "{:#?}", call),
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
        write!(f, "({:#?} {:?} {:#?})", self.lhs, self.op.kind, self.rhs)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Decl {
    Variable(Variable),
    Function(Function),
    Struct,
    Enum,
    Interface,
    Macro,
}

impl Debug for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::Variable(var) => write!(f, "{:#?}", var),
            Decl::Function(func) => write!(f, "{:#?}", func),
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
        if let Some(type_hint) = &self.type_hint {
            write!(
                f,
                "({:?} {:?} : {:#?} = {:#?})",
                self.prot.kind, self.name.kind, type_hint, self.expr
            )
        } else {
            write!(
                f,
                "({:?} {:?} := {:#?})",
                self.prot.kind, self.name.kind, self.expr
            )
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Identifier(Token),
}

impl Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(t) => write!(f, "{:?}", t.kind),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
    pub prot: Token,
    pub name: Token,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}

impl Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?} {:?} : fn({:#?}) -> {:#?} = {:#?})",
            self.prot.kind, self.name.kind, self.params, self.return_type, self.body
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Parameter {
    pub name: Token,
    pub param_type: Type,
}

impl Debug for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:#?}", self.name.kind, self.param_type)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Block(pub Vec<Node>);

impl Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n")?;
        for node in &self.0 {
            write!(f, "    {:#?}\n", node)?;
        }
        write!(f, "}}")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Call {
    pub callee: Token,
    pub args: Vec<Expr>,
}

impl Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}({:#?}))", self.callee.kind, self.args)
    }
}
