use std::collections::HashMap;

use crate::parser::{Program, AST};


enum Op {
    Add, 
    Sub, 
    Div,
    Mul,
    Mod
}

type Instructions = Vec<(Op, u8, u8, u8)>;

pub struct IntermRepr {
    main: Instructions,
    labels: HashMap<String, Instructions>
}

pub struct IRGen<'a> {
    ir: IntermRepr,
    ast: &'a Program<'a>
}

impl<'a> IRGen<'a> {
    pub fn new(ast: &'a Program<'a>) -> Self {
        Self {ir: IntermRepr {main: Vec::new(), labels: HashMap::new()}, ast}
    }
    pub fn generate(&self) -> IntermRepr {
        todo!()
    }
}
