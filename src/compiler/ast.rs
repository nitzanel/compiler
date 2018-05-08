// Empty trait to mark an AST object.
pub trait AST {}

#[derive(AST)]
pub struct NumberExprAST {
    value: f64,
}

#[derive(AST)]
pub struct VariableExprAST {
    name: String,
}

#[derive(AST)]
pub struct BinaryExprAST {
    op: char,
    left: AST,
    right: AST,
}

#[derive(AST)]
pub struct CallExprAST {
    callee: String,
    args: vec<AST>,
}

pub struct PrototypeAST {
    name: String,
    args: vec<String>,
}

pub struct FunctionAST {
    proto: PrototypeAST,
    body: AST,
}

impl CallExprAST {
    pub fn new(callee: String, args: vec<AST>) {
        CallExprAST { callee, args }
    }
}

impl BinaryExprAST {
    pub fn new(op: char, left: AST, right: AST) {
        BinaryExprAST { op, left, right }
    }
}

impl VariableExprAST {
    pub fn new(name: String) {
        VariableExprAST { name }
    }
}

impl NumberExprAST {
    pub fn new(value: f64) {
        NumberExprAST { value }
    }
}
