use compiler::Lexer;
use compiler::ast;
use compiler::ast::*;

use compiler::Token;
use std::iter::Iterator;

/* Grammar:
 * program          : [[statement | expression] Delimiter ? ]*;
 * statement        : [declaration | definition];
 * declaration      : Extern prototype;
 * definition       : Def prototype expression;
 * prototype        : Ident OpeningParenthesis [Ident Comma ?]* ClosingParenthesis;
 * expression       : [primary_expr (Op primary_expr)*];
 * primary_expr     : [Ident | Number | call_expr | parenthesis_expr];
 * call_expr        : Ident OpeningParenthesis [expression Comma ?]* ClosingParenthesis;
 * parenthesis_expr : OpeningParenthesis expression ClosingParenthesis;
 */

pub struct TokenIterator {
    tokens: Vec<Token>,
    position: isize,
}

impl TokenIterator {
    pub fn from_lexer(mut lexer: Lexer) -> Self {
        TokenIterator::new(lexer.get_all_tokens())
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        TokenIterator {
            tokens,
            position: -1,
        }
    }

    pub fn peek(&self) -> Option<Token> {
        if self.position + 1 < self.tokens.len() as isize {
            Some(self.tokens[self.position as usize + 1].clone())
        } else {
            None
        }
    }
}

impl Iterator for TokenIterator {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.position + 1 < self.tokens.len() as isize {
            self.position += 1;
            Some(self.tokens[self.position as usize].clone())
        } else {
            None
        }
    }
}

pub struct Parser {
    input: TokenIterator,
}

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            input: TokenIterator::from_lexer(lexer),
        }
    }

    pub fn parse(parsed_tree: Vec<
}
