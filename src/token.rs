// Languge keywords
extern crate itertools;

static DEF_STRING: &str = "def";
static EXTERN_STRING: &str = "extern";

use self::itertools::{Itertools, PeekingNext};
use std::iter::FromIterator;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    // EOF
    TokEOF,

    // Commands
    TokDef,
    TokExtern,

    // Primary
    TokIdentity,
    TokNumber,

    // Unknown
    TokUnkown,
}

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
        let mut iden = String::from_iter(self.input.take_while_ref(|c| c.is_alphanumeric()));
        iden.insert(0, last_char);
        iden
    }

    pub fn get_token(&mut self) -> Token {
        self.input
            .take_while_ref(|c| c.is_whitespace())
            .collect::<String>();

        let last_char = match self.input.next() {
            Some(c) => c,
            None => return Token::TokEOF,
        };

        let mut ident_string = String::new();
        if last_char.is_alphabetic() {
            // Alphabetic of [a-zA-Z][a-zA-Z0-9]*
            ident_string = self.get_identifier_string(last_char);
        }

        if ident_string == DEF_STRING {
            Token::TokDef
        } else if ident_string == EXTERN_STRING {
            Token::TokExtern
        } else {
            Token::TokIdentity
        }
    }
}
