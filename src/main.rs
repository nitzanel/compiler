mod token;
use token::{Lexer, Token};

fn main() {
    let mut lex = Lexer::new(" def hello");
    assert_eq!(lex.get_token(), Token::Def);
    assert_eq!(lex.get_token(), Token::Identity);
    assert_eq!(lex.get_token(), Token::EOF);

    lex = Lexer::new("2 34. 51.23 .1");
    assert_eq!(lex.get_token(), Token::Number(2.));
    assert_eq!(lex.get_token(), Token::Number(34.));
    assert_eq!(lex.get_token(), Token::Number(51.23));
    assert_eq!(lex.get_token(), Token::Number(0.1));
    assert_eq!(lex.get_token(), Token::EOF);

    lex = Lexer::new("?");
    assert_eq!(lex.get_token(), Token::Unknown('?'));
}
