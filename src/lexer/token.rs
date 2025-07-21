#![allow(dead_code)]

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub loc: Location,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum TokenType {
    // 3 char tokens
    DotDotDot, // ...

    // 2 char tokens
    Assign,        // :=
    PlusEql,       // +=
    MinusEql,      // -=
    StarEql,       // *=
    DivEql,        // /=
    LessEql,       // <=
    GreaterEql,    // >=
    DoubleEql,     // ==
    RightArrow,    // ->
    FatRightArrow, // =>
    Pipe,          // |>

    // 1 char tokens
    Colon,     // :
    SemiColon, // ;
    Eql,       // =
    Plus,      // +
    Minus,     // -
    Star,      // *
    Div,       // /
    Less,      // <
    Greater,   // >
    VertBar,   // |
    Dot,       // .
    Comma,     // ,
    Question,  // ?
    Bang,      // !
    Pound,     // #
    OParen,    // (
    CParen,    // )
    OBrack,    // {
    CBrack,    // }
    OSquare,   // [
    CSquare,   // ]

    // Keywords
    Const,
    Val,
    Mut,
    Struct,
    Enum,
    Macro,
    Impl,
    Interface,
    Priv,
    Pub,
    Override,
    Fn,
    Defer,
    If,
    Else,
    Switch,
    For,
    While,
    Return,
    Break,
    Continue,
    Unreachable,

    // Literals
    Ident(String),
    Number(String),
    String(String),
    Invalid(char),
    EOF,
}

impl TokenType {
    pub fn into_str(self) -> &'static str {
        use TokenType::*;
        match self {
            Return => "Return",
            While => "While",
            DotDotDot => "DotDotDot",
            Assign => "Assign",
            PlusEql => "PlusEql",
            MinusEql => "MinusEql",
            StarEql => "StarEql",
            DivEql => "DivEql",
            LessEql => "LessEql",
            GreaterEql => "GreaterEql",
            DoubleEql => "DoubleEql",
            RightArrow => "RightArrow",
            FatRightArrow => "FatRightArrow",
            Pipe => "Pipe",
            Colon => "Colon",
            SemiColon => "SemiColon",
            Eql => "Eql",
            Plus => "Plus",
            Minus => "Minus",
            Star => "Star",
            Div => "Div",
            Less => "Less",
            Greater => "Greater",
            VertBar => "VertBar",
            Dot => "Dot",
            Comma => "Comma",
            Question => "Question",
            Bang => "Bang",
            Pound => "Pound",
            OParen => "OParen",
            CParen => "CParen",
            OBrack => "OBrack",
            CBrack => "CBrack",
            OSquare => "OSquare",
            CSquare => "CSquare",
            Const => "Const",
            Val => "Val",
            Mut => "Mut",
            Struct => "Struct",
            Enum => "Enum",
            Macro => "Macro",
            Impl => "Impl",
            Interface => "Interface",
            Priv => "Priv",
            Pub => "Pub",
            Override => "Override",
            Fn => "Fn",
            Defer => "Defer",
            If => "If",
            Else => "Else",
            Switch => "Switch",
            For => "For",
            Break => "Break",
            Continue => "Continue",
            Unreachable => "Unreachable",
            Ident(_) => "Ident",
            Number(_) => "Number",
            String(_) => "String",
            Invalid(_) => "Invalid",
            EOF => "EOF",
        }
    }
}

impl From<&str> for TokenType {
    fn from(s: &str) -> Self {
        match s {
            "return" => Self::Return,
            "while" => Self::While,
            "const" => Self::Const,
            "val" => Self::Val,
            "mut" => Self::Mut,
            "struct" => Self::Struct,
            "enum" => Self::Enum,
            "macro" => Self::Macro,
            "impl" => Self::Impl,
            "interface" => Self::Interface,
            "priv" => Self::Priv,
            "pub" => Self::Pub,
            "override" => Self::Override,
            "fn" => Self::Fn,
            "defer" => Self::Defer,
            "if" => Self::If,
            "else" => Self::Else,
            "switch" => Self::Switch,
            "for" => Self::For,
            "break" => Self::Break,
            "continue" => Self::Continue,
            "unreachable" => Self::Unreachable,
            _ => Self::Ident(s.to_string()),
        }
    }
}