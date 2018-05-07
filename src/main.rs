mod token;
use token::{Lexer, Token};

fn main() {
    let mut lex = Lexer::new(" def hello");
    assert_eq!(lex.get_token(), Token::TokDef);
    assert_eq!(lex.get_token(), Token::TokIdentity);
    assert_eq!(lex.get_token(), Token::TokEOF);
}
