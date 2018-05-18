#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    // EOF
    EOF,

    // Commands
    Def,
    Extern,

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

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}
