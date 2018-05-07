// Languge keywords
extern crate itertools;

static DEF_STRING: &str = "def";
static EXTERN_STRING: &str = "extern";

use self::itertools::Itertools;
use std::str::Chars;

use compiler::Token;

pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars(),
        }
    }

    fn get_identifier_string(&mut self, last_char: char) -> String {
        let mut iden = String::new();
        iden.push(last_char);
        iden.extend(self.input.take_while_ref(|c| c.is_alphanumeric()));
        iden
    }

    fn get_number(&mut self, last_char: char) -> f64 {
        let mut concat_string = String::new();
        concat_string.push(last_char);
        concat_string.extend(self.input.take_while_ref(|c| c.is_digit(10) || *c == '.'));
        concat_string.parse::<f64>().unwrap()
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

        let last_char = match self.input.next() {
            Some(c) => c,
            None => return Token::EOF,
        };

        if last_char.is_alphabetic() {
            let ident_string = self.get_identifier_string(last_char);
            // Alphabetic of [a-zA-Z][a-zA-Z0-9]*

            if ident_string == DEF_STRING {
                Token::Def
            } else if ident_string == EXTERN_STRING {
                Token::Extern
            } else {
                Token::Identity(ident_string)
            }
        } else if last_char.is_digit(10) || last_char == '.' {
            Token::Number(self.get_number(last_char))
        } else if last_char == '#' {
            Token::Comment(self.skip_line())
        } else {
            Token::Unknown(last_char)
        }
    }
}