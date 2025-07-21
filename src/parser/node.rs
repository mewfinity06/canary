use crate::lexer::token::Token;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Node {
    Stmt(Stmt),
    Expr(Expr),
    EOF,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Stmt {
    Decl(Decl),
    ImplBlock(ImplBlock),
    If(IfBranch),
    Switch(Expr),
    For(Expr),
    While(Expr),
    MacroDef(MacroDefinition),
    StructDef(Decl),
    EnumDef(Decl),
    InterfaceDef(Decl),
    Break,
    Continue,
    Return(Option<Expr>),
    Expr(Expr),
}

impl Stmt {
    pub fn into_expr(self) -> Option<Expr> {
        if let Stmt::Expr(expr) = self {
            Some(expr)
        } else {
            None
        }
    }
}

impl Node {
    pub fn into_stmt(self) -> Result<Stmt, anyhow::Error> {
        if let Node::Stmt(stmt) = self {
            Ok(stmt)
        } else {
            anyhow::bail!("Expected a statement, found an expression or EOF")
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Expr {
    Atom(Atom),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    MemberAccess {
        object: Box<Expr>,
        member: Token,
    },
    IndexAccess {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    ArrayLiteral(Vec<Expr>),
    StructLiteral(Vec<(Token, Expr)>),
    EnumVariantLiteral {
        variant_name: Token,
        value: Option<Box<Expr>>,
    },
    MacroCall {
        name: Token,
        args: Vec<MacroArg>,
    },
    Switch {
        condition: Box<Expr>,
        cases: Vec<SwitchCase>,
    },
    For {
        kind: ForLoopKind,
        init_decl: Option<Box<Decl>>,
        condition: Option<Box<Expr>>,
        update_expr: Option<Box<Expr>>,
        iterable: Option<Box<Expr>>,
        capture: Option<Token>,
        body: FunctionBody,
    },
    While {
        kind: WhileLoopKind,
        condition: Box<Expr>,
        capture: Option<Token>,
        update_body: Option<Box<Expr>>,
        body: FunctionBody,
    },
    Unreachable,
    AnonEnum(Vec<EnumVariant>),
    AnonStruct(Vec<StructField>),
    Parenthesized(Box<Expr>),
    Struct(Vec<StructField>),
    Enum {
        variants: Vec<EnumVariant>,
        generics: Vec<Token>,
    },
    Interface(Vec<InterfaceMember>),
    FunctionType {
        params: Vec<(Token, Type)>,
        return_type: Box<Type>,
    },
    Empty,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Atom {
    Ident(Token),
    Number(Token),
    String(Token),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Decl {
    pub special_type: bool,
    pub protection: Token,
    pub name: Token,
    pub type_: Option<Type>,
    pub expr: Expr,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Type {
    pub base: Expr,
    pub generics: Vec<Token>,
    pub is_optional: bool,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct StructField {
    pub name: Token,
    pub type_: Option<Type>,
    pub is_optional: bool,
    pub is_private: bool,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct EnumVariant {
    pub name: Token,
    pub type_: Option<Type>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct ImplBlock {
    pub target_name: Token,
    pub impl_name: Option<Token>,
    pub members: Vec<Stmt>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct FunctionBody {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct MacroDefinition {
    pub name: Token,
    pub arms: Vec<MacroArm>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct MacroArm {
    pub params: Vec<(Token, Token)>,
    pub body: FunctionBody,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum MacroArg {
    Expr(Expr),
    String(Token),
    InterpolatedIdent(Token),
    VariadicInterpolation(Token),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct InterfaceMember {
    pub name: Token,
    pub params: Vec<(Token, Type)>,
    pub return_type: Type,
    pub body: Option<FunctionBody>,
    pub is_override: bool,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct SwitchCase {
    pub variant_name: Token,
    pub capture: Option<Token>,
    pub body: FunctionBody,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct IfBranch {
    pub condition: Box<Expr>,
    pub capture_body: Option<Token>,
    pub body: FunctionBody,
    pub else_branch: Option<ElseBranch>,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum ElseBranch {
    Block(FunctionBody),
    If(Box<IfBranch>),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum ForLoopKind {
    Simple,
    ForEach,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum WhileLoopKind {
    Simple,
    WhileEach,
    WhileUpdate,
}
