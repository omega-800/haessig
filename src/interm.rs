use std::collections::HashMap;

use crate::parser::{Program};


#[allow(dead_code)]
enum Op {
    Add, 
    Sub, 
    Div,
    Mul,
    Mod
}

#[allow(dead_code)]
enum Instruction<'a> {
    Bin(Op, &'a str, &'a str),
    Jmp(&'a str),
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
