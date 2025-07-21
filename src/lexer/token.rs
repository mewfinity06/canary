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

    // Literals
    Ident(String),
    Keyword(String),
    Number(String),
    String(String),
    Invalid(char),
    EOF,
}
