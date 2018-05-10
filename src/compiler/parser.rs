use compiler::Lexer;
use compiler::ast;

use compiler::Token;
use std::error::Error;
use std::fmt;
use std::iter::{Iterator, Peekable};

pub struct Parser {
    input: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(mut input: Lexer) -> Self {
        Parser {
            input: input.get_all_tokens(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.position + 1 < self.input.len() {
            Some(&self.input[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn next(&mut self) -> Option<&Token> {
        let next = self.peek();
        match next {
            Some(tok) => Some(tok),
            None => None,
        }
    }

    fn parse_number_expr(&mut self) -> impl ast::ExprAST {
        let token = self.next().unwrap();
        match token {
            Token::Number(n) => ast::NumberExprAST::new(*n),
            _ => panic!("unexpeted token {:?}", token),
        }
    }

    fn parse_identifier_expr(&mut self) -> Box<ast::ExprAST> {
        let name = match self.next().unwrap() {
            Token::Identity(s) => s.to_string(),
            _ => panic!("todo"),
        };
        if let Token::LParen = self.peek().unwrap() {
            // throw the one we peeked on
            self.next().unwrap();
            self.parse_call_expression(name)
        } else {
            self.parse_variable_expression(name)
        }
    }

    fn parse_variable_expression(&mut self, name: String) -> Box<ast::ExprAST> {
        Box::new(ast::VariableExprAST::new(name))
    }

    fn parse_call_expression(&mut self, callee: String) -> Box<ast::ExprAST> {
        let mut args: Vec<Box<ast::ExprAST>> = Vec::new();
        while true {
            let next = self.peek().unwrap();
            match next {
                Token::RParen => break,
                Token::Identity(_) => args.push(self.parse_identifier_expr()),
                Token::Number(_) => args.push(Box::new(self.parse_number_expr())),
                token @ _ => panic!("Unexpected token {:?}", token),
            };
            let next = self.peek().unwrap();
            match next {
                Token::RParen => break,
                Token::Comma => continue,
                token @ _ => panic!("unexpceted token {:?}", token),
            };
        }
        Box::new(ast::CallExprAST::new(callee, args))
    }
}
