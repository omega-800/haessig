use crate::parser::{
    Arg, Bin, BinOperator, Block, Call, Expr, FunAss, Prim, PrimType, Program, Ret, StEx, Stmt,
    VarAss,
};

pub trait Transpileable<'a> {
    fn transpile(&'a self) -> String;
}

impl<'a> Transpileable<'a> for Stmt<'a> {
    fn transpile(&'a self) -> String {
        match self {
            Stmt::FunAss(fun_ass) => fun_ass.transpile(),
            Stmt::VarAss(var_ass) => var_ass.transpile() + ";\n",
            Stmt::StEx(st_ex) => st_ex.transpile() + ";\n",
            Stmt::Ret(ret) => ret.transpile() + ";\n",
        }
    }
}

impl<'a> Transpileable<'a> for Arg<'a> {
    fn transpile(&'a self) -> String {
        let pt = self.pt.transpile();
        let id = self.id;
        format!("{pt} {id}")
    }
}

impl<'a> Transpileable<'a> for Block<'a> {
    fn transpile(&'a self) -> String {
        self.stmts
            .iter()
            .map(|s| s.transpile())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<'a> Transpileable<'a> for PrimType {
    fn transpile(&'a self) -> String {
        match self {
            PrimType::String => "char*".to_string(),
            PrimType::R8 => "float".to_string(),
            PrimType::N8 => "unsigned int".to_string(),
            PrimType::Z8 => "int".to_string(),
            PrimType::Boolean => "int".to_string(),
        }
    }
}

impl<'a> Transpileable<'a> for FunAss<'a> {
    fn transpile(&'a self) -> String {
        let ret = self.ret.map_or("void".to_string(), |pt| pt.transpile());
        let args = self
            .args
            .iter()
            .map(|a| a.transpile())
            .collect::<Vec<String>>()
            .join(", ");
        let body = self.body.transpile();
        let id = match self.id {
            "chuchichäschtli" => "main",
            i => i,
        };
        format!("{ret} {id}({args}) {{\n{body}\n}}")
    }
}

impl<'a> Transpileable<'a> for Expr<'a> {
    fn transpile(&'a self) -> String {
        match self {
            Expr::StEx(st_ex) => st_ex.transpile(),
            Expr::Prim(prim) => prim.transpile(),
            Expr::Bin(bin) => bin.transpile(),
        }
    }
}

impl<'a> Transpileable<'a> for Prim<'a> {
    fn transpile(&'a self) -> String {
        match self {
            Prim::Bool(v) => (if *v { 1 } else { 0 }).to_string(),
            // Prim::Str(v) => format!("\"{v}\""),
            Prim::Str(v) => v.to_string(),
            Prim::R8(v) => v.to_string(),
            Prim::Id(v) => v.to_string(),
        }
    }
}

impl<'a> Transpileable<'a> for BinOperator {
    fn transpile(&'a self) -> String {
        (match self {
            BinOperator::Gliich => "==",
            BinOperator::GrösserGliich => ">=",
            BinOperator::Grösser => ">",
            BinOperator::ChlinnerGliich => "<=",
            BinOperator::Chlinner => "<",
            BinOperator::Ungliich => "!=",
            BinOperator::Und => "&&",
            BinOperator::Oder => "||",
            BinOperator::Rescht => "%",
            // TODO:
            BinOperator::Hoch => "^",
            BinOperator::Mal => "*",
            BinOperator::Durch => "/",
            BinOperator::Plus => "+",
            BinOperator::Minus => "-",
        })
        .to_string()
    }
}

impl<'a> Transpileable<'a> for Bin<'a> {
    fn transpile(&'a self) -> String {
        let lhs = (*self.lhs).transpile();
        let rhs = (*self.rhs).transpile();
        let op = self.op.transpile();
        format!("{lhs} {op} {rhs}")
    }
}

impl<'a> Transpileable<'a> for VarAss<'a> {
    fn transpile(&'a self) -> String {
        let id = self.id;
        let pt = self.pt.map_or("void".to_string(), |pt| pt.transpile());
        let value = self.value.transpile();
        format!("{pt} {id} = {value}")
    }
}

impl<'a> Transpileable<'a> for Call<'a> {
    fn transpile(&'a self) -> String {
        let id = match self.id {
            "schreie" => "printf",
            i => i,
        };
        let args = self
            .args
            .iter()
            .map(|a| a.transpile())
            .collect::<Vec<String>>()
            .join(", ");
        format!("{id}({args})")
    }
}

impl<'a> Transpileable<'a> for StEx<'a> {
    fn transpile(&'a self) -> String {
        match self {
            StEx::Call(call) => call.transpile(),
            StEx::Block(block) => block.transpile(),
        }
    }
}

impl<'a> Transpileable<'a> for Ret<'a> {
    fn transpile(&'a self) -> String {
        let expr = self.expr.transpile();
        format!("return {expr}")
    }
}

pub struct Transpiler<'a> {
    ast: &'a Program<'a>,
}

impl<'a> Transpiler<'a> {
    pub fn new(ast: &'a Program<'a>) -> Self {
        Self { ast }
    }

    pub fn generate(&self) -> String {
        let mut tl = "".to_string();
        let mut main = "".to_string();
        for stmt in self.ast.iter() {
            match stmt {
                Stmt::FunAss(fun_ass) => {
                    tl += &fun_ass.transpile();
                    tl += "\n"
                }
                Stmt::VarAss(var_ass) => {
                    tl += &var_ass.transpile();
                    tl += ";\n"
                }
                Stmt::StEx(st_ex) => main += &st_ex.transpile(),
                Stmt::Ret(ret) => {
                    main += &ret.transpile();
                    main += ";\n"
                }
            }
        }
        // format!("#include <stdio.h>\n{tl}\n{main}")
        format!("#include <stdio.h>\n{tl}")
    }
}
