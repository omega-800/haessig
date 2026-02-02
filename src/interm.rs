use std::collections::HashMap;

use crate::parser::{Program};

#[allow(dead_code)]
enum BinOp {
    Add, 
    Sub, 
    Div,
    Mul,
    Cmp,
    Mod
}
#[allow(dead_code)]
enum JOp {
    Jmp, 
    Jne, 
    Jle, 
    Jlt, 
    Jge, 
    Jgt, 
}

#[allow(dead_code)]
enum Instruction<'a> {
    Bin(BinOp, &'a str, &'a str),
    Jmp(JOp, &'a str),
    Param(&'a str),
    Call(&'a str,&'a str),
    Return(&'a str),
    Pop(&'a str),
    Ass(&'a str,&'a str),
}


type Instructions<'a> = Vec<Instruction<'a>>;

#[allow(dead_code)]
pub struct IntermRepr<'a> {
    main: Instructions<'a>,
    labels: HashMap<&'a str, Instructions<'a>>
}

#[allow(dead_code)]
pub struct IRGen<'a> {
    ir: IntermRepr<'a>,
    ast: &'a Program<'a>
}

#[allow(dead_code)]
impl<'a> IRGen<'a> {
    pub fn new(ast: &'a Program<'a>) -> Self {
        Self {ir: IntermRepr {main: Vec::new(), labels: HashMap::new()}, ast}
    }
    pub fn generate(&self) /*-> IntermRepr<'a>*/ {
        println!("TODO");
    }
}
