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
    Let,
    Mut,
    Static,
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

    Error(String),
}

impl TokenType {
    pub fn into_str(self) -> &'static str {
        use TokenType::*;
        match self {
            Static => "Static",
            Return => "Return",
            DotDotDot => "...",
            Assign => ":=",
            PlusEql => "+=",
            MinusEql => "-=",
            StarEql => "*=",
            DivEql => "/=",
            LessEql => "<=",
            GreaterEql => ">=",
            DoubleEql => "==",
            RightArrow => "->",
            FatRightArrow => "=>",
            Pipe => "|>",
            Colon => ":",
            SemiColon => ";",
            Eql => "=",
            Plus => "+",
            Minus => "-",
            Star => "*",
            Div => "/",
            Less => ">",
            Greater => "<",
            VertBar => "|",
            Dot => ".",
            Comma => ",",
            Question => "?",
            Bang => "!",
            Pound => "#",
            OParen => "(",
            CParen => ")",
            OBrack => "{",
            CBrack => "}",
            OSquare => "[",
            CSquare => "]",
            Const => "Const",
            Let => "Let",
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
            Break => "Break",
            Continue => "Continue",
            Unreachable => "Unreachable",
            Ident(_) => "Ident",
            Number(_) => "Number",
            String(_) => "String",
            Invalid(_) => "Invalid",
            EOF => "EOF",
            Error(_) => unreachable!(),
        }
    }
}

impl From<&str> for TokenType {
    fn from(s: &str) -> Self {
        match s {
            "static" => Self::Static,
            "return" => Self::Return,
            "const" => Self::Const,
            "let" => Self::Let,
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
            "break" => Self::Break,
            "continue" => Self::Continue,
            "unreachable" => Self::Unreachable,
            _ => Self::Ident(s.to_string()),
        }
    }
}
