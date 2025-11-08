use std::{collections::HashMap, fmt::Display};

use crate::parser::{Block, Expr, Prim, PrimType, Program, StEx, Stmt, VarAss, AST};

const BUILTINS: [&str; 3] = ["schreie", "gib", "verlang"];

type Scope<'a> = HashMap<String, &'a Option<PrimType>>;

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

pub struct SemanticAnalyzer<'a> {
    ast: &'a Vec<AST<'a>>,
    scope_stack: Vec<Scope<'a>>,
}

// TODO: annotate AST
impl<'a> SemanticAnalyzer<'a> {
    pub fn new(Program(ast): &'a Program<'a>) -> Self {
        Self {
            ast,
            scope_stack: vec![HashMap::new()],
        }
    }

    pub fn analyze(&'a mut self) -> Result<(), SemAnError> {
        // TODO: hoisting
        // TODO: type checking
        for item in self.ast.iter() {
            match item {
                AST::Stmt(stmt) => self.analyze_stmt(stmt)?,
                AST::Expr(expr) => self.analyze_expr(expr)?,
            }
        }
        Ok(())
    }

    fn analyze_stmt(&mut self, stmt: &'a Stmt) -> Result<(), SemAnError> {
        match stmt {
            Stmt::FunAss(fun_ass) => {
                self.add_symbol(fun_ass.id.to_string(), &fun_ass.ret);
                self.scope_stack.push(HashMap::new());
                self.analyze_block(&fun_ass.body)?;
                self.scope_stack.pop();
            }
            Stmt::VarAss(var_ass) => self.analyze_var_ass(var_ass)?,
            Stmt::StEx(st_ex) => self.analyze_st_ex(st_ex)?,
        }
        Ok(())
    }
    fn add_prim(&mut self, prim: &'a Prim, symbol: String) -> Result<(), SemAnError> {
        match prim {
            Prim::Str(_) => self.add_symbol(symbol, &Some(PrimType::String)),
            Prim::U8(_) => self.add_symbol(symbol, &Some(PrimType::U8)),
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
    fn analyze_prim(&mut self, prim: &'a Prim) -> Result<(), SemAnError> {
        if let Prim::Id(id) = prim {
            if !self.has_symbol(id) {
                return Err(SemAnError::new(format!("Token `{}` not found", id)));
            }
        }
        Ok(())
    }

    fn analyze_expr(&mut self, expr: &'a Expr) -> Result<(), SemAnError> {
        match expr {
            Expr::StEx(st_ex) => self.analyze_st_ex(st_ex)?,
            Expr::Prim(prim) => self.analyze_prim(prim)?,
        }
        Ok(())
    }

    fn analyze_st_ex(&mut self, st_ex: &'a StEx) -> Result<(), SemAnError> {
        match st_ex {
            StEx::Call(call) => {
                if !self.has_symbol(call.id) && !BUILTINS.contains(&call.id) {
                    return Err(SemAnError::new(format!(
                        "Trying to call function `{}` which is not defined",
                        call.id
                    )));
                }
                for arg in call.args.iter() {
                    if let Err(e) = self.analyze_expr(arg) {
                        return Err(SemAnError::new(format!(
                            "Trying to call function `{}` with argument that is not defined: {}",
                            call.id, e.message
                        )));
                    }
                }
            }
            StEx::Block(block) => self.analyze_block(block)?,
        }
        Ok(())
    }

    fn analyze_var_ass(&mut self, var_ass: &'a VarAss) -> Result<(), SemAnError> {
        match &var_ass.value {
            Expr::StEx(st_ex) => self.analyze_st_ex(st_ex)?,
            Expr::Prim(prim) => self.add_prim(prim, var_ass.id.to_string())?,
        }
        Ok(())
    }

    fn analyze_block(&mut self, block: &'a Block) -> Result<(), SemAnError> {
        for stmt in block.stmts.iter() {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    pub fn add_symbol(&mut self, symbol: String, value: &'a Option<PrimType>) {
        if let Some(s) = self.get_symbol(&symbol) {
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
    pub fn has_symbol(&self, symbol: &str) -> bool {
        for item in self.scope_stack.iter().rev() {
            if item.contains_key(symbol) {
                return true;
            }
        }
        false
    }
    pub fn get_symbol(&self, symbol: &str) -> &'a Option<PrimType> {
        for item in self.scope_stack.iter().rev() {
            if let Some(v) = item.get(symbol) {
                return v;
            }
        }
        &None
    }
}
