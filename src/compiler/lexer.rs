// Languge keywords
extern crate itertools;

static DEF_STRING: &str = "def";
static EXTERN_STRING: &str = "extern";

use self::itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

use compiler::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn get_identifier_string(&mut self) -> String {
        self.input
            .take_while_ref(|c| c.is_alphanumeric())
            .collect::<String>()
    }

    fn get_number(&mut self) -> f64 {
        self.input
            .take_while_ref(|c| c.is_digit(10) || *c == '.')
            .collect::<String>()
            .parse::<f64>()
            .unwrap()
    }

    fn skip_line(&mut self) -> String {
        self.input
            .by_ref()
            .take_while(|c| *c != '\n' && *c != '\r')
            .collect::<String>()
    }

    pub fn get_token(&mut self) -> Token {
        self.input
            .take_while_ref(|c| c.is_whitespace())
            .collect::<String>();

        let last_char = match self.input.peek() {
            Some(c) => *c,
            None => return Token::EOF,
        };

        if last_char.is_alphabetic() {
            let ident_string = self.get_identifier_string();
            // Alphabetic of [a-zA-Z][a-zA-Z0-9]*

            if ident_string == DEF_STRING {
                Token::Def
            } else if ident_string == EXTERN_STRING {
                Token::Extern
            } else {
                Token::Identity(ident_string)
            }
        } else if last_char.is_digit(10) || last_char == '.' {
            Token::Number(self.get_number())
        } else if last_char == '#' {
            Token::Comment(self.skip_line())
        } else if last_char == '(' {
            Token::LParen
        } else if last_char == ')' {
            Token::RParen
        } else if last_char == ',' {
            Token::Comma
        } else {
            Token::Unknown(last_char)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_token() {
        let mut lex = Lexer::new(" def hello");
        assert_eq!(lex.get_token(), Token::Def);
        assert_eq!(lex.get_token(), Token::Identity("hello".to_string()));
        assert_eq!(lex.get_token(), Token::EOF);
    }

    #[test]
    fn test_number_token() {
        let mut lex = Lexer::new("2 34. 51.23 .1");
        assert_eq!(lex.get_token(), Token::Number(2.));
        assert_eq!(lex.get_token(), Token::Number(34.));
        assert_eq!(lex.get_token(), Token::Number(51.23));
        assert_eq!(lex.get_token(), Token::Number(0.1));
        assert_eq!(lex.get_token(), Token::EOF);
    }

    #[test]
    fn test_unknown_token() {
        let mut lex = Lexer::new("?");
        assert_eq!(lex.get_token(), Token::Unknown('?'));
    }

    #[test]
    fn test_comment_token() {
        let mut lex = Lexer::new("#this is a comment\n 3.1");
        assert_eq!(
            lex.get_token(),
            Token::Comment("#this is a comment".to_string())
        );
        assert_eq!(lex.get_token(), Token::Number(3.1));
        assert_eq!(lex.get_token(), Token::EOF);
    }
}
