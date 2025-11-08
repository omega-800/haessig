
enum Op {
    Add, 
    Sub, 
    Div,
    Mul,
    Mod
}

type Instructions = Vec<(Op, u8, u8, u8)>;

struct IntermRepr {
    main: Instructions,
    labels: HashMap<String, Instructions>
}

struct IRGen {
    ir: IntermRepr,
    ast: AST
}

impl IRGen {
    fn new(ast: AST) -> Self {
        Self {ir: IntermRepr {main: Vec::new(), labels: HashMap::new()}, ast}
    }
    fn generate(&self) -> IntermRepr {
        todo!()
    }
}
