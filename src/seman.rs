use std::{collections::HashMap, fmt::Display};

use crate::parser::{Block, Expr, Prim, PrimType, Program, StEx, Stmt, VarAss};

const BUILTINS: [&str; 3] = ["schreie", "gib", "verlang"];

type Scope<'a> = HashMap<&'a str, &'a Option<PrimType>>;

#[derive(Debug, Clone)]
pub struct SemAnError {
    /* TODO:
    pos: usize,
    row: usize,
    col: usize,
    */
    message: String,
}

impl SemAnError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for SemAnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Your code is semantically incorrect :)\nHere's a tip: {}",
            self.message
        )
    }
}

pub trait Analyzable<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError>;
}

impl<'a> Analyzable<'a> for Stmt<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError> {
        match self {
            Stmt::FunAss(fun_ass) => {
                ctx.add_symbol(fun_ass.id.as_str(), &fun_ass.ret);
                fun_ass.body.analyze(ctx)?;
            }
            Stmt::VarAss(var_ass) => var_ass.analyze(ctx)?,
            Stmt::StEx(st_ex) => st_ex.analyze(ctx)?,
            Stmt::Ret(ret) => ret.expr.analyze(ctx)?,
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Expr<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError> {
        match self {
            Expr::StEx(st_ex) => st_ex.analyze(ctx)?,
            Expr::Prim(prim) => prim.analyze(ctx)?,
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Prim<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError> {
        if let Prim::Id(id) = self {
            if !ctx.has_symbol(id) {
                return Err(SemAnError::new(format!("Token `{}` not found", id)));
            }
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for Block<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError> {
        ctx.scope_stack.push(HashMap::new());
        for stmt in self.stmts.iter() {
            stmt.analyze(ctx)?;
        }
        ctx.scope_stack.pop();
        Ok(())
    }
}

impl<'a> Analyzable<'a> for StEx<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError> {
        match self {
            StEx::Call(call) => {
                if !ctx.has_symbol(call.id.as_str()) && !BUILTINS.contains(&call.id.as_str()) {
                    return Err(SemAnError::new(format!(
                        "Trying to call function `{}` which is not defined",
                        call.id
                    )));
                }
                for arg in call.args.iter() {
                    if let Err(e) = arg.analyze(ctx) {
                        return Err(SemAnError::new(format!(
                            "Trying to call function `{}` with argument that is not defined: {}",
                            call.id, e.message
                        )));
                    }
                }
            }
            StEx::Block(block) => block.analyze(ctx)?,
        }
        Ok(())
    }
}

impl<'a> Analyzable<'a> for VarAss<'a> {
    fn analyze(&'a self, ctx: &mut SemanticAnalyzer<'a>) -> Result<(), SemAnError> {
        match &self.value {
            Expr::StEx(st_ex) => st_ex.analyze(ctx)?,
            Expr::Prim(prim) => ctx.add_prim(prim, self.id.as_str())?,
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

    pub fn analyze(&'a mut self) -> Result<(), SemAnError> {
        // TODO: hoisting
        // TODO: type checking
        for stmt in self.ast.iter() {
            stmt.analyze(self)?;
        }
        Ok(())
    }

    fn add_prim(&mut self, prim: &'a Prim, symbol: &'a str) -> Result<(), SemAnError> {
        match prim {
            Prim::Str(_) => self.add_symbol(symbol, &Some(PrimType::String)),
            Prim::Bool(_) => self.add_symbol(symbol, &Some(PrimType::Boolean)),
            Prim::R8(_) => self.add_symbol(symbol, &Some(PrimType::R8)),
            Prim::Id(id) => {
                if self.has_symbol(id) {
                    self.add_symbol(symbol, self.get_symbol(id));
                } else {
                    return Err(SemAnError::new(format!(
                        "Token `{}` not found when assigning to `{}`",
                        id, symbol
                    )));
                }
            }
        }
        Ok(())
    }

    fn add_symbol(&mut self, symbol: &'a str, value: &'a Option<PrimType>) {
        if let Some(s) = self.get_symbol(symbol) {
            eprintln!(
                "WARNING: Shadowing previously defined variable `{}` ({:?}) with new {}",
                symbol,
                s,
                value.map_or("Unknown".to_string(), |v| v.to_string())
            )
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

    fn get_symbol(&self, symbol: &str) -> &'a Option<PrimType> {
        for item in self.scope_stack.iter().rev() {
            if let Some(v) = item.get(symbol) {
                return v;
            }
        }
        &None
    }
}
