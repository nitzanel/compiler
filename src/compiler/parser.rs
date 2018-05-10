use compiler::ast;
use compiler::Lexer;

use compiler::Token;
use std::iter::Iterator;

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

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            input: TokenIterator::from_lexer(lexer),
        }
    }

    fn parse_number_expr(&mut self) -> impl ast::ExprAST {
        match self.input.next().unwrap() {
            Token::Number(n) => ast::NumberExprAST::new(n),
            token => panic!("unexpeted token {:?}", token),
        }
    }

    fn parse_identifier_expr(&mut self) -> Box<ast::ExprAST> {
        let name = match self.input.next().unwrap() {
            Token::Identity(s) => s.to_string(),
            token => panic!("token {:?}", token),
        };
        if let Token::LParen = self.input.peek().unwrap() {
            // throw the one we peeked on
            self.input.next().unwrap();
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
        loop {
            // The functions here advances to the next token
            match self.input.peek().unwrap() {
                Token::RParen => break,
                Token::Identity(_) => args.push(self.parse_identifier_expr()),
                Token::Number(_) => args.push(Box::new(self.parse_number_expr())),
                token => panic!("Unexpected token {:?}", token),
            };
            // Nothing advances to the next token, use next instead of peek
            match self.input.next().unwrap() {
                Token::RParen => break,
                Token::Comma => continue,
                token => panic!("unexpceted token {:?}", token),
            };
        }
        Box::new(ast::CallExprAST::new(callee, args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_call_expression() {
        let lexer = Lexer::new("hello(1,2)");
        let mut parser = Parser::new(lexer);
        let expected = ast::CallExprAST::new(
            "hello".to_string(),
            vec![
                Box::new(ast::NumberExprAST::new(1.)),
                Box::new(ast::NumberExprAST::new(2.)),
            ],
        );
        let result = parser.parse_identifier_expr();
        let call = result.as_any().downcast_ref::<ast::CallExprAST>().unwrap();
        assert_eq!(call.callee, expected.callee);
        assert_eq!(call.args.len(), expected.args.len());
        for i in 0..expected.args.len() {
            let res = call.args[i]
                .as_any()
                .downcast_ref::<ast::NumberExprAST>()
                .unwrap();
            let exp = expected.args[i]
                .as_any()
                .downcast_ref::<ast::NumberExprAST>()
                .unwrap();
            assert_eq!(*res, *exp);
        }
    }
}
