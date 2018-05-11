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
    Add,
    Sub,
    Mul,
    Div,

    // paren
    RParen,
    LParen,

    Comma,

    // Unknown
    Unknown(char),

    Comment(String),
}
