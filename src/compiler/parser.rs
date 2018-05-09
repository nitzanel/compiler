use compiler::Lexer;
use compiler::ast;

use compiler::Token;
use std::error::Error;
use std::fmt;
use std::iter::{Iterator, Peekable};

#[derive(Debug)]
pub struct UnexpectedTokenError {
    expected: Token,
    found: Token,
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, format!("Expected token {}, Recieved {}", self.expected, self.found))
    }
}

impl Error for UnexpectedTokenError {
    fn description(&self) -> &str {
        format!("Expected {}, Recieved {}", self.expected, self.found)
    }
}

pub struct Parser {
    input: Peekable<Iterator<Item=Token>>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
        input: lexer.get_all_tokens().iter().peekable()
        }
    }

    fn eat(&self, token: Token) -> Result<Token, UnexpectedTokenError> {
        let tok = self.input.next().unwrap();
        if tok == token {
            Ok(tok)
        } else { 
            Err(UnexpectedTokenError { expected: token, found: tok })
        }
    }

    fn parse_number_expr(&mut self) -> impl ast::ExprAST {
        match self.eat(Token::Number) {
            Ok(Token::Number(n)) => ast::NumberExprAST::new(n),
            Err(err) => panic!(err)
        }
    }

    fn parse_paren_expr(&mut self) -> impl ast::ExprAST {
        self.lexer.get_token();
        self.parse_expression()
    }

    fn parse_identifier_expr(&mut self) -> impl ast::ExprAST {
        if let Some(
        match token {
            Token::Identity(s) => ast::VariableExprAST::new(s),
            Token::LParen => {
                let mut args = vec::<String>new();
                while true {
                    args.push(self.parse_expression());
                    let next = self.lexer.get_token();
                    match next {
                        Token::RParen => break,
                        Token::Comma => continue,
                        _ => panic!("parse_identifier_expr recieved unexpeceted token. expected ',' or ')'. got {}", next)
                    };
                }
                ast::CallExprAST::new(
                args,
            }
            _ => panic!("parse_idnetifier_expr recieved unexpected token: {}", token)
        }
    }

    fn parse_expression(&mut self) -> impl ast::ExprAST {
        let token = self.lexer.get_token();
        if token == Token::RParen {
            EmptyExpr {}
        }
    }
}
