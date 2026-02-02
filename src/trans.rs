pub struct Transpiler<'a> {
    ast &'a Program<'a>
}

impl<'a> Transpiler<'a> {
    pub fn new(ast: &'a Program<'a>) -> Self {
        Self { ast }
    }

    pub fn generate (&self) -> &'a str {
""
    }
}
