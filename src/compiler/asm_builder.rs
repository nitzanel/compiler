use compiler::ast::{ASTNode, Function, Prototype, Expression};
use compiler::BinaryOp;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ASMInstruction {
    instruction: String,
}


#[derive(Default)]
pub struct ASMBuilder {
    state: HashMap<String, String>
}

pub trait ASMGenerator {
    fn gen_asm(&mut self, ast: Vec<ASTNode>) -> Vec<ASMInstruction>;
    fn gen_instruction(&mut self, node: &ASTNode) -> Vec<ASMInstruction>;
}


impl ASMBuilder {
    fn new() -> Self {
        ASMBuilder::default()
    }

    fn build_prototype(&mut self, prototype: &Prototype) -> Vec<ASMInstruction> {
        println!("{:#?}", prototype);
        vec![]

    }

    fn build_function(&mut self, function: &Function) -> Vec<ASMInstruction> {
        let mut instructions = vec![];
        if (function.prototype.name == "") {
            // top level dec
            instructions.extend(self.build_expression(function.body.clone()));
        }
        println!("{:#?}", function);
        instructions
    }

    // TODO
    fn get_empty_register(&self) -> String {
        "EBX".to_owned()
    }

    fn build_assignment_op(&mut self, var_name: String, expression: Box<Expression>) -> Vec<ASMInstruction>{
        let result = self.build_expression(*expression); 
        result
    }

    fn build_literal_expr(&mut self, value: f64) -> Vec<ASMInstruction> {

        vec![ASMInstruction{ instruction: format!("MOV eax, {}", value)}]
    }

    fn build_variable_expr(&mut self, name: String) -> Vec<ASMInstruction> {
        let register = self.get_empty_register();
        self.state.insert(name, register);
        vec![]
    }

    fn build_binary_expr(&mut self, op: BinaryOp, lhs: Box<Expression>, rhs: Box<Expression>) -> Vec<ASMInstruction> {
        vec![]
    }

    fn build_call_expr(&mut self, name: String, expressions: Vec<Expression>) -> Vec<ASMInstruction> {
        vec![]
    }

    fn build_expression(&mut self, expression: Expression) -> Vec<ASMInstruction> {
        match expression {
            Expression::LiteralExpr(value) => self.build_literal_expr(value),
            Expression::VariableExpr(name) => self.build_variable_expr(name),
            Expression::AssignmentOp(name, expression) => self.build_assignment_op(name, expression),
            Expression::CallExpr(name, expressions) => self.build_call_expr(name, expressions),
            Expression::BinaryExpr(op, lhs, rhs) => self.build_binary_expr(op, lhs, rhs),
            expr => panic!("Recieved unexpected expr {:#?}", expr),
        }

    }

}

impl ASMGenerator for ASMBuilder {
    fn gen_asm(&mut self, ast: Vec<ASTNode>) -> Vec<ASMInstruction> {
        let mut instructions = vec![];
        for node in ast {
            instructions.extend(self.gen_instruction(&node));
        }
        instructions
    }

    fn gen_instruction(&mut self, node: &ASTNode) -> Vec<ASMInstruction> {
        match node {
            ASTNode::ExternNode(prototype) => self.build_prototype(prototype),
            ASTNode::FunctionNode(function) => self.build_function(function),
        }
    }
}
