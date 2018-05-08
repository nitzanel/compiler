// Empty trait to mark an ExprAST object.
pub trait ExprAST {}

pub struct EmptyExpr {}

impl ExprAST for EmptyExpr {}

pub struct NumberExprAST {
    value: f64,
}

impl ExprAST for NumberExprAST {}

impl NumberExprAST {
    pub fn new(value: f64) -> Self {
        NumberExprAST { value }
    }
}

pub struct VariableExprAST {
    name: String,
}

impl ExprAST for VariableExprAST {}

impl VariableExprAST {
    pub fn new(name: String) -> Self {
        VariableExprAST { name }
    }
}

pub struct BinaryExprAST<T: ExprAST, S: ExprAST> {
    op: char,
    left: T,
    right: S,
}

impl<T, S> ExprAST for BinaryExprAST<T, S>
where
    T: ExprAST,
    S: ExprAST,
{
}

impl<T, S> BinaryExprAST<T, S>
where
    T: ExprAST,
    S: ExprAST,
{
    pub fn new(op: char, left: T, right: S) -> Self {
        BinaryExprAST { op, left, right }
    }
}

pub struct CallExprAST<T: ExprAST> {
    callee: String,
    args: Vec<T>,
}

impl<T> ExprAST for CallExprAST<T>
where
    T: ExprAST,
{
}

impl<T> CallExprAST<T>
where
    T: ExprAST,
{
    pub fn new(callee: String, args: Vec<T>) -> Self {
        CallExprAST { callee, args }
    }
}

pub struct PrototypeAST {
    name: String,
    args: Vec<String>,
}

impl ExprAST for PrototypeAST {}

impl PrototypeAST {
    pub fn new(name: String, args: Vec<String>) -> Self {
        PrototypeAST { name, args }
    }
}

pub struct FunctionAST<T: ExprAST> {
    proto: PrototypeAST,
    body: T,
}

impl<T> ExprAST for FunctionAST<T>
where
    T: ExprAST,
{
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
        let op = '+';
        let expr = BinaryExprAST::new(op, left, right);
        assert_eq!(expr.left.value, 4.9);
        assert_eq!(expr.right.value, 8.200);
        let expr2 = BinaryExprAST::new(op, expr, other);
        assert_eq!(expr2.left.left.value, 4.9);
    }
}
