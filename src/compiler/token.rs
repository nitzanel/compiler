#[derive(PartialEq, Debug)]
pub enum Token {
    // EOF
    EOF,

    // Commands
    Def,
    Extern,

    // Primary
    Identity(String),
    Number(f64),

    // paren
    RParen,
    LParen,

    // comman
    Comma,

    // Unknown
    Unknown(char),

    Comment(String),
}
