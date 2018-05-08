use compiler::Lexer;
use compiler::ast;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    fn parse_number_expr(&mut self) -> ast::ExprAST {
        let token = self.lexer.get_token();
        if let Token::Number(n) = token {
            ast::NumberExprAST::new(n)
        }
        panic!("parse_number_expr expected Token::Number. recieved {}",);
    }

    fn parse_paren_expr(&mut self) -> ast::ExprAST {
        self.lexer.get_token();
        self.parse_expression()
    }

    fn parse_identifier_expr(&mut self) {
        token = self.lexer.get_token();
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
                ast::CallExprAST(
                args,
            }
            _ => panic!("parse_idnetifier_expr recieved unexpected token: {}", token)
        }
    }

    fn parse_expression(&mut self) -> ast::ExprAST {
        let token = self.lexer.get_token();
        if token == Token::RParen {
            EmptyExpr {}
        }
    }
}
