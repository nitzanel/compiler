// Empty trait to mark an ExprAST object.
use compiler::Token;
use std::any::Any;
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

#[derive(Debug, PartialEq)]
pub struct BinaryExprAST<T: ExprAST, S: ExprAST> {
    op: Token,
    left: T,
    right: S,
}

impl<T: 'static, S: 'static> ExprAST for BinaryExprAST<T, S>
where
    T: ExprAST,
    S: ExprAST,
{
    fn as_any(&self) -> &Any {
        self
    }
}

impl<T, S> BinaryExprAST<T, S>
where
    T: ExprAST,
    S: ExprAST,
{
    pub fn new(op: Token, left: T, right: S) -> Self {
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
pub struct FunctionAST<T: ExprAST> {
    proto: PrototypeAST,
    body: T,
}

impl<T: 'static> ExprAST for FunctionAST<T>
where
    T: ExprAST,
{
    fn as_any(&self) -> &Any {
        self
    }
}

impl<T> FunctionAST<T>
where
    T: ExprAST,
{
    pub fn new(proto: PrototypeAST, body: T) -> Self {
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
        let left = NumberExprAST::new(4.9);
        let right = NumberExprAST::new(8.200);
        let other = NumberExprAST::new(9.0);
        let expr = BinaryExprAST::new(Token::Add, left, right);
        assert_eq!(expr.op, Token::Add);
        assert_eq!(expr.left.value, 4.9);
        assert_eq!(expr.right.value, 8.200);
        let expr2 = BinaryExprAST::new(Token::Add, expr, other);
        assert_eq!(expr2.left.left.value, 4.9);
    }
}
