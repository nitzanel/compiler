extern crate itertools;

// Languge keywords
const DEF_STRING: &str = "def";
const EXTERN_STRING: &str = "extern";

use self::itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

use compiler::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    reached_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            reached_eof: false,
        }
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        self.collect::<Vec<Token>>()
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
        let _dropped_whitespace = self.input
            .take_while_ref(|c| c.is_whitespace())
            .collect::<String>();

        let last_char = match self.input.peek() {
            Some(c) => *c,
            None => {
                self.reached_eof = true;
                return Token::EOF;
            }
        };

        if last_char.is_alphabetic() {
            let ident_string = self.get_identifier_string();
            // Alphabetic of [a-zA-Z][a-zA-Z0-9]*

            match ident_string.as_ref() {
                DEF_STRING => Token::Def,
                EXTERN_STRING => Token::Extern,
                _ => Token::Identity(ident_string),
            }
        } else if last_char.is_digit(10) || last_char == '.' {
            // Number of the form [.0-9]*
            Token::Number(self.get_number())
        } else {
            // Consume the char
            self.input.next().unwrap();
            match last_char {
                '#' => Token::Comment(self.skip_line()),
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                _ => Token::Unknown(last_char),
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        if self.reached_eof {
            None
        } else {
            Some(self.get_token())
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
    fn test_call_function_tokens() {
        let mut lex = Lexer::new("hello(49, 8.200)");
        assert_eq!(lex.get_token(), Token::Identity("hello".to_string()));
        assert_eq!(lex.get_token(), Token::LParen);
        assert_eq!(lex.get_token(), Token::Number(49.));
        assert_eq!(lex.get_token(), Token::Comma);
        assert_eq!(lex.get_token(), Token::Number(8.200));
        assert_eq!(lex.get_token(), Token::RParen);
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
            Token::Comment("this is a comment".to_string())
        );
        assert_eq!(lex.get_token(), Token::Number(3.1));
        assert_eq!(lex.get_token(), Token::EOF);
    }

    #[test]
    fn test_get_all_tokens() {
        let mut lex = Lexer::new("4.9");
        assert_eq!(lex.get_all_tokens(), vec![Token::Number(4.9), Token::EOF]);
    }

    #[test]
    fn test_lexer_as_iterator() {
        let mut lex = Lexer::new("4.9");
        assert_eq!(lex.next(), Some(Token::Number(4.9)));
        assert_eq!(lex.next(), Some(Token::EOF));
    }
}
