// Empty trait to mark an ExprAST object.
//

use std::any::{Any, TypeId};

use compiler::Token;
use std::fmt::Debug;
pub trait ExprAST: Debug {
    fn as_any(&self) -> &Any;
}

#[derive(Debug, PartialEq)]
pub struct EmptyExpr {}

impl ExprAST for EmptyExpr {
    fn as_any(&self) -> &Any {
        self
    }
}

#[derive(Debug, PartialEq)]
pub struct NumberExprAST {
    value: f64,
}

impl ExprAST for NumberExprAST {
    fn as_any(&self) -> &Any {
        self
    }
}

impl NumberExprAST {
    pub fn new(value: f64) -> Self {
        NumberExprAST { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableExprAST {
    name: String,
}

impl ExprAST for VariableExprAST {
    fn as_any(&self) -> &Any {
        self
    }
}

impl VariableExprAST {
    pub fn new(name: String) -> Self {
        VariableExprAST { name }
    }
}

#[derive(Debug)]
pub struct BinaryExprAST {
    op: Token,
    left: Box<ExprAST>,
    right: Box<ExprAST>,
}

impl ExprAST for BinaryExprAST {
    fn as_any(&self) -> &Any {
        self
    }
}

impl BinaryExprAST {
    pub fn new(op: Token, left: Box<ExprAST>, right: Box<ExprAST>) -> Self {
        BinaryExprAST { op, left, right }
    }
}

#[derive(Debug)]
pub struct CallExprAST {
    pub callee: String,
    pub args: Vec<Box<ExprAST>>,
}

impl ExprAST for CallExprAST {
    fn as_any(&self) -> &Any {
        self
    }
}

impl CallExprAST {
    pub fn new(callee: String, args: Vec<Box<ExprAST>>) -> Self {
        CallExprAST { callee, args }
    }
}

#[derive(Debug)]
pub struct PrototypeAST {
    name: String,
    args: Vec<String>,
}

impl ExprAST for PrototypeAST {
    fn as_any(&self) -> &Any {
        self
    }
}

impl PrototypeAST {
    pub fn new(name: String, args: Vec<String>) -> Self {
        PrototypeAST { name, args }
    }
}

#[derive(Debug)]
pub struct FunctionAST {
    proto: PrototypeAST,
    body: Box<ExprAST>,
}

impl ExprAST for FunctionAST {
    fn as_any(&self) -> &Any {
        self
    }
}

impl FunctionAST {
    pub fn new(proto: PrototypeAST, body: Box<ExprAST>) -> Self {
        FunctionAST { proto, body }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_expr() {
        let num_expr = NumberExprAST::new(3.14);
        assert_eq!(num_expr.value, 3.14);
    }

    #[test]
    fn test_binary_expr() {
        let left = Box::new(NumberExprAST::new(4.9));
        let right = Box::new(NumberExprAST::new(8.200));
        let other = Box::new(NumberExprAST::new(9.0));
        let expr = BinaryExprAST::new(Token::Add, left, right);
        assert_eq!(expr.op, Token::Add);
        assert_eq!(
            expr.left
                .as_any()
                .downcast_ref::<NumberExprAST>()
                .unwrap()
                .value,
            4.9
        );
        assert_eq!(
            expr.right
                .as_any()
                .downcast_ref::<NumberExprAST>()
                .unwrap()
                .value,
            8.200
        );
        let expr2 = BinaryExprAST::new(Token::Add, Box::new(expr), other);
        assert_eq!(
            expr2
                .left
                .as_any()
                .downcast_ref::<BinaryExprAST>()
                .unwrap()
                .left
                .as_any()
                .downcast_ref::<NumberExprAST>()
                .unwrap()
                .value,
            4.9
        );
    }
}
