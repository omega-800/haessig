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
type Instructions = Vec<(Op, u8, u8, u8)>;

#[allow(dead_code)]
pub struct IntermRepr<'a> {
    main: Instructions,
    labels: HashMap<&'a str, Instructions>
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
