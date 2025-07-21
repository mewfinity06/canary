#![allow(dead_code)]

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Token {
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

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        match s {
            "..." => Self::DotDotDot,
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
