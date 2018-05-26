use compiler::ast::ASTNode;
use std::collections::HashMap;

pub struct ASMInstruction {
    instruction: String,
}


pub struct ASMBuilder {
    state: HashMap<String, String>
}

pub trait ASMGenerator {
    fn gen_asm(ast: Vec<ASTNode>) -> Vec<ASMInstruction>;
    fn gen_instruction(node: &ASTNode) -> Vec<ASMInstruction>;
}

impl ASMGenerator for ASMBuilder {
    fn gen_asm(ast: Vec<ASTNode>) -> Vec<ASMInstruction> {
        let mut instructions = vec![];
        for node in ast {
            instructions.extend(Self::gen_instruction(&node));
        }
        instructions
    }
    fn gen_instruction(node: &ASTNode) -> Vec<ASMInstruction> {
        println!("{:#?}", node);
        vec![]
    }
}
