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

    // Unknown
    Unknown(char),

    Comment(String),
}
