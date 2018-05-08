// Empty trait to mark an ExprAST object.
pub trait ExprAST {}

#[derive(ExprAST)]
pub struct EmptyExpr {}

#[derive(ExprAST)]
pub struct NumberExprAST {
    value: f64,
}

#[derive(ExprAST)]
pub struct VariableExprAST {
    name: String,
}

#[derive(ExprAST)]
pub struct BinaryExprAST {
    op: char,
    left: ExprAST,
    right: ExprAST,
}

#[derive(ExprAST)]
pub struct CallExprAST {
    callee: String,
    args: vec<ExprAST>,
}

#[derive(ExprAST)]
pub struct PrototypeAST {
    name: String,
    args: vec<String>,
}

#[derive(ExprAST)]
pub struct FunctionAST {
    proto: PrototypeAST,
    body: ExprAST,
}

impl FunctionAST {
    pub fn new(proto: PrototypeAST, body: ExprAST) {
        FunctionAST { proto, body }
    }
}

impl PrototypeAST {
    pub fn new(name: String, args: vec<String>) -> Self {
        PrototypeAST { name, args }
    }
}

impl CallExprAST {
    pub fn new(callee: String, args: vec<ExprAST>) -> Self {
        CallExprAST { callee, args }
    }
}

impl BinaryExprAST {
    pub fn new(op: char, left: ExprAST, right: ExprAST) -> Self {
        BinaryExprAST { op, left, right }
    }
}

impl VariableExprAST {
    pub fn new(name: String) -> Self {
        VariableExprAST { name }
    }
}

impl NumberExprAST {
    pub fn new(value: f64) -> Self {
        NumberExprAST { value }
    }
}
