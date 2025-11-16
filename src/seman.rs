use std::{collections::HashMap, fmt::Display};

use crate::parser::{
    Bin, BinOperator, Block, Call, Expr, FunAss, Prim, PrimType, Program, StEx, Stmt, VarAss,
};

// TODO: check
// [ ] arg types match fn definition
// [ ] arg count matches fn definition
// [ ] return type matches fn definition
// [ ] value type matches variable type on assign
// [ ] if/while boolean predicate
// [x] no multiple declarations with same id
// [ ] no id is reserved keyword
// [ ] only one main method
// [ ] bin/un operators with correct types
// [ ] uninitialized vars can't be accessed
// [ ] division by zero
// [ ] null-dereferencing
// [ ] array index out of bounds

const BUILTINS: [&str; 2] = ["schreie", "verlange"];

type Scope<'a> = HashMap<&'a str, Option<&'a PrimType>>;

#[derive(Debug, Clone)]
pub enum SemAnError<'a> {
    /* TODO:
    pos: usize,
    row: usize,
    col: usize,
    */
    TokenNotDefined(&'a str),
    FunctionNotDefined(&'a str),
    SameFunctionArgs(&'a str, &'a str),
    ArgNotDefined(&'a str, &'a str),
    AssignTokenNotDefined(&'a str, &'a str),
}

impl<'a> Display for SemAnError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Your code is semantically incorrect :) {}",
            match self {
                SemAnError::TokenNotDefined(id) => format!("Token `{}` not defined", id),
                SemAnError::AssignTokenNotDefined(id, ass) =>
                    format!("Token `{}` not defined when assigning to `{}`", id, ass),
                SemAnError::FunctionNotDefined(id) => format!("Function `{}` not defined", id),
                SemAnError::SameFunctionArgs(id, fun) => format!(
                    "Duplicate id's `{}` passed as function args to `{}`",
                    id, fun
                ),
                SemAnError::ArgNotDefined(id, fun) =>
                    format!("Argument `{}` for function `{}` not defined", id, fun),
            }
        )
    }
}

pub trait Analyzable<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>>;
}

impl<'a> Analyzable<'a> for FunAss<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        ctx.add_symbol(self.id, self.ret.as_ref());
        for (i, x) in self.args.iter().enumerate() {
            for y in self.args.iter().skip(i + 1) {
                if x.id == y.id {
                    println!("{} {} {}", x.id, y.id, i);
                    return Err(SemAnError::SameFunctionArgs(x.id, self.id));
                }
            }
        }
        for arg in self.args.iter() {
            ctx.add_symbol(arg.id, Some(&arg.pt));
        }
        self.body.analyze(ctx)?;
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Stmt<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        match self {
            Stmt::FunAss(fun_ass) => fun_ass.analyze(ctx)?,
            Stmt::VarAss(var_ass) => var_ass.analyze(ctx)?,
            Stmt::StEx(st_ex) => st_ex.analyze(ctx)?,
            Stmt::Ret(ret) => ret.expr.analyze(ctx)?,
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Expr<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        match self {
            Expr::StEx(st_ex) => st_ex.analyze(ctx)?,
            Expr::Prim(prim) => prim.analyze(ctx)?,
            Expr::Bin(bin) => bin.analyze(ctx)?,
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Prim<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        if let Prim::Id(id) = self {
            if !ctx.has_symbol(id) {
                return Err(SemAnError::TokenNotDefined(id));
            }
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Block<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        ctx.scope_stack.push(HashMap::new());
        for stmt in self.stmts.iter() {
            stmt.analyze(ctx)?;
        }
        ctx.scope_stack.pop();
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Call<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        if !ctx.has_symbol(self.id) && !BUILTINS.contains(&self.id) {
            return Err(SemAnError::FunctionNotDefined(self.id));
        }
        for arg in self.args.iter() {
            if let Err(e) = arg.analyze(ctx) {
                return Err(SemAnError::ArgNotDefined(
                    self.id,
                    match e {
                        SemAnError::TokenNotDefined(t) => t,
                        SemAnError::FunctionNotDefined(t) => t,
                        _ => "",
                    },
                ));
            }
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for StEx<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        match self {
            StEx::Call(call) => call.analyze(ctx)?,
            StEx::Block(block) => block.analyze(ctx)?,
        }
        Ok(())
    }
}
impl<'a> Analyzable<'a> for Bin<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        //TODO: check type and operator
        self.lhs.analyze(ctx)?;
        self.rhs.analyze(ctx)?;
        /*
        match self.op {
            // Eq
            BinOperator::Gliich | BinOperator::Ungliich => todo!(),
            // Cmp
            BinOperator::GrösserGliich
            | BinOperator::Grösser
            | BinOperator::ChlinnerGliich
            | BinOperator::Chlinner => todo!(),
            // Bool
            BinOperator::Und | BinOperator::Oder => todo!(),
            // Num
            BinOperator::Rescht
            | BinOperator::Hoch
            | BinOperator::Mal
            | BinOperator::Durch
            | BinOperator::Plus
            | BinOperator::Minus => todo!(),
        };
*/
        Ok(())
    }
}

impl<'a> Analyzable<'a> for VarAss<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError<'a>> {
        //self.value.analyze(ctx)?;
        //FIXME: this is wrong. why do i not add these symbols to the scope
        match &self.value {
            Expr::StEx(st_ex) => st_ex.analyze(ctx)?,
            Expr::Prim(prim) => ctx.add_prim(prim, self.id)?,
            Expr::Bin(bin) => bin.analyze(ctx)?,
        }
        Ok(())
    }
}

pub struct SemanticAnalyzer<'a> {
    ast: &'a Program<'a>,
    scope_stack: Vec<Scope<'a>>,
}

// TODO: annotate AST
impl<'a> SemanticAnalyzer<'a> {
    pub fn new(ast: &'a Program<'a>) -> Self {
        Self {
            ast,
            scope_stack: vec![HashMap::new()],
        }
    }

    pub fn analyze(&'a mut self) -> Result<(), SemAnError<'a>> {
        // TODO: hoisting
        // TODO: type checking
        for stmt in self.ast.iter() {
            stmt.analyze(self)?;
        }
        Ok(())
    }

    fn add_prim(&mut self, prim: &'a Prim, symbol: &'a str) -> Result<(), SemAnError<'a>> {
        match prim {
            Prim::Str(_) => self.add_symbol(symbol, Some(PrimType::String).as_ref()),
            Prim::Bool(_) => self.add_symbol(symbol, Some(PrimType::Boolean).as_ref()),
            Prim::R8(_) => self.add_symbol(symbol, Some(PrimType::R8).as_ref()),
            Prim::Id(id) => {
                if self.has_symbol(id) {
                    self.add_symbol(symbol, self.get_symbol(id));
                } else {
                    return Err(SemAnError::AssignTokenNotDefined(id, symbol));
                }
            }
        }
        Ok(())
    }

    fn add_symbol(&mut self, symbol: &'a str, value: Option<&'a PrimType>) {
        // FIXME: confusion
        if let Some(l) = self.scope_stack.last() {
            if let Some(s) = l.get(symbol) {
                eprintln!(
                    "WARNING: Shadowing previously defined variable `{}` ({:?}) with new {}",
                    symbol,
                    s,
                    value.map_or("Unknown".to_string(), |v| v.to_string())
                )
            }
        }
        if let Some(cur) = self.scope_stack.last_mut() {
            cur.insert(symbol, value);
        }
    }

    fn has_symbol(&self, symbol: &str) -> bool {
        for item in self.scope_stack.iter().rev() {
            if item.contains_key(symbol) {
                return true;
            }
        }
        false
    }

    fn get_symbol(&self, symbol: &str) -> Option<&'a PrimType> {
        for item in self.scope_stack.iter().rev() {
            if let Some(v) = item.get(symbol) {
                return *v;
            }
        }
        None
    }
}
