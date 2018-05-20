#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // EOF
    EOF,

    // Commands
    Def,
    Extern,

    // Flow control
    If,
    Then,
    Else,

    // Variables
    Let,

    // Semi colon
    Delimeter,

    // Primary
    Identity(String),
    Number(f64),

    // Binary Operations
    BinOp(BinaryOp),

    // paren
    RParen,
    LParen,

    Comma,

    // Unknown
    Unknown(char),

    Comment(String),
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Shift,
}
