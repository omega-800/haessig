use std::fmt::Display;

use crate::lexer::{Token, Tokens, TT};

pub type Program<'a> = Vec<Stmt<'a>>;

#[derive(Debug, Clone, Copy)]
pub enum PrimType {
    String,
    R8,
    N8,
    Z8,
    Boolean,
}

impl Display for PrimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum StEx<'a> {
    Call(Call<'a>),
    Block(Block<'a>),
}
#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    FunAss(FunAss<'a>),
    VarAss(VarAss<'a>),
    StEx(StEx<'a>),
    Ret(Ret<'a>),
}
#[derive(Debug, Clone)]
pub enum Expr<'a> {
    StEx(StEx<'a>),
    Prim(Prim<'a>),
}
#[derive(Debug, Clone)]
pub struct Ret<'a> {
    pub expr: Expr<'a>,
}
#[derive(Debug, Clone)]
pub enum Prim<'a> {
    Bool(bool),
    Str(&'a str),
    R8(u8),
    Id(&'a str),
}
#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub stmts: Vec<Stmt<'a>>,
}
#[derive(Debug, Clone)]
pub struct VarAss<'a> {
    pub id: &'a str,
    pub value: Expr<'a>,
    pub pt: Option<PrimType>,
}
#[derive(Debug, Clone)]
pub struct FunAss<'a> {
    pub id: &'a str,
    pub body: Block<'a>,
    pub ret: Option<PrimType>,
}
#[derive(Debug, Clone)]
pub struct Call<'a> {
    pub id: &'a str,
    pub args: Vec<Expr<'a>>,
}

impl PrimType {
    fn from_tt(value: TT) -> Option<Self> {
        match value {
            TT::TypZeiche => Some(PrimType::String),
            TT::TypR8 => Some(PrimType::R8),
            TT::TypN8 => Some(PrimType::N8),
            TT::TypZ8 => Some(PrimType::Z8),
            TT::TypWahrheit => Some(PrimType::Boolean),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    NoTokensLeft,
    UnexpectedToken(String, Token),
    ExpectedToken(String, TT, Token),
    ExpectedType(String, Token),
    ExpectedPrim(String, Token),
    MissingValue(String, TT, Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = "Your code is bonkers";
        let pos = |tok: &Token| format!("row {} col {}", tok.row, tok.col);
        match self {
            ParseError::NoTokensLeft => write!(f, "{}: No tokens left", msg),

            ParseError::UnexpectedToken(t, token) => {
                write!(
                    f,
                    "{}: Unexpected token at {} parsing {}",
                    msg,
                    pos(token),
                    t
                )
            }
            ParseError::ExpectedToken(t, tt, token) => write!(
                f,
                "{}: Expected token {:?} at {}, got {:?} parsing {}",
                msg,
                tt,
                pos(token),
                token.token_type,
                t
            ),
            ParseError::ExpectedType(t, token) => {
                write!(f, "{}: Expected type at {} parsing {}", msg, pos(token), t)
            }
            ParseError::ExpectedPrim(t, token) => {
                write!(
                    f,
                    "{}: Expected primitive at {} parsing {}",
                    msg,
                    pos(token),
                    t
                )
            }
            ParseError::MissingValue(t, tt, token) => {
                write!(
                    f,
                    "{}: Expected value for {:?} at {} parsing {}",
                    msg,
                    tt,
                    pos(token),
                    t
                )
            }
        }
    }
}

macro_rules! expect_id_next {
    ( $t:expr, $self:ident ) => {{
        //$self.consume();
        $self.pos += 1;
        //let id_tok = $self.cur_tok()?;
        let id_tok = $self.tokens.get($self.pos).ok_or(ParseError::NoTokensLeft)?;
        if id_tok.token_type != TT::Id {
            return Err(ParseError::ExpectedToken($t, TT::Id, $self.get_tok()));
        }
        let Some(id) = id_tok.value.as_ref() else {
            return Err(ParseError::MissingValue($t, TT::Id, $self.get_tok()));
        };
        id
    }};
}
macro_rules! consume_next_tok {
    ( $t:expr, $self:ident, $y:expr ) => {{
        if ($self.cur_tok()?).token_type != $y {
            return Err(ParseError::ExpectedToken($t, $y, $self.get_tok()));
        }
        $self.consume();
    }};
}

pub struct Parser<'a> {
    tokens: &'a Tokens,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        Self { tokens, pos: 0 }
    }
    pub fn parse(&mut self) -> Result<Program<'a>, ParseError> {
        let mut ast = vec![];
        while self.pos < self.tokens.len() {
            let stmt = self.parse_stmt()?;
            ast.push(stmt);
        }
        Ok(ast)
    }
    pub fn consume(&mut self) {
        self.pos += 1;
    }
    pub fn cur_tok(&self) -> Result<&Token, ParseError> {
        self.tokens.get(self.pos).ok_or(ParseError::NoTokensLeft)
    }
    pub fn get_tok(&self) -> Token {
        self.tokens[self.pos].clone()
    }
    pub fn parse_fun_ass(&mut self) -> Result<FunAss<'a>, ParseError> {
        let id = expect_id_next!("FunAss".to_string(), self);
        self.consume();
        //TODO: args
        let mut ret = None;
        let git_tok = self.cur_tok()?;
        if git_tok.token_type == TT::Git {
            self.consume();

            let type_tok = self.cur_tok()?;
            if let Some(prim_type) = PrimType::from_tt(type_tok.token_type) {
                self.consume();
                ret = Some(prim_type);
            } else {
                return Err(ParseError::ExpectedType(
                    "FunAss".to_string(),
                    self.get_tok(),
                ));
            }
        }
        let body = self.parse_block()?;

        // FIXME: fight the borrow checker harder
        Ok(FunAss {id, body, ret})
        //Err(ParseError::NoTokensLeft)
    }
    pub fn parse_var_ass(&mut self) -> Result<VarAss<'a>, ParseError> {
        let id = expect_id_next!("VarAss".to_string(), self);
        self.consume();
        consume_next_tok!("VarAss".to_string(), self, TT::Isch);
        let value = self.parse_expr()?;
        let mut pt = None;
        if (self.cur_tok()?).token_type == TT::Als {
            self.consume();
            pt = PrimType::from_tt((self.cur_tok()?).token_type);
            self.consume();
            if pt.is_none() {
                return Err(ParseError::ExpectedType(
                    "VarAss".to_string(),
                    self.get_tok(),
                ));
            }
        }
        // FIXME: fight the borrow checker harder
        Ok(VarAss { id, value, pt })
        //Err(ParseError::NoTokensLeft)
    }
    pub fn parse_expr(&mut self) -> Result<Expr<'a>, ParseError> {
        match (self.cur_tok()?).token_type {
            TT::Tuen | TT::LBrace => Ok(Expr::StEx(self.parse_st_ex()?)),
            _ => Ok(Expr::Prim(self.parse_prim()?)),
        }
    }
    pub fn parse_block(&mut self) -> Result<Block<'a>, ParseError> {
        self.consume();
        let mut stmts = vec![];
        while (self.cur_tok()?).token_type != TT::RBrace {
            stmts.push(self.parse_stmt()?);
        }
        consume_next_tok!("Block".to_string(), self, TT::RBrace);
        Ok(Block { stmts })
    }
    pub fn parse_prim(&mut self) -> Result<Prim<'a>, ParseError> {
        let tok = self.cur_tok()?;
        match tok.token_type {
            // TODO: error handling
            TT::Num => Ok(Prim::R8(
                tok.value.as_ref().map_or(0, |v| v.parse().unwrap_or(0)),
            )),
            TT::Wahr => Ok(Prim::Bool(true)),
            TT::Falsch => Ok(Prim::Bool(false)),
            // FIXME: fight the borrow checker harder
            //TT::Str => Ok(Prim::Str(tok.value.as_ref().map_or("", |v| v))),
            //TT::Id => Ok(Prim::Id(tok.value.as_ref().map_or("", |v| v))),
            _ => Err(ParseError::ExpectedPrim("Prim".to_string(), self.get_tok())),
        }
    }
    pub fn parse_call(&mut self) -> Result<Call<'a>, ParseError> {
        let id = expect_id_next!("Call".to_string(), self);
        self.consume();
        consume_next_tok!("Call".to_string(), self, TT::Mit);
        let mut args = vec![];
        let mut prev = self.pos;
        loop {
            if let Ok(a) = self.parse_expr() {
                args.push(a);
                prev = self.pos;
            } else {
                self.pos = prev;
            }
            if (self.cur_tok()?).token_type != TT::Comma {
                break;
            }
            self.consume();
        }
        Ok(Call { id, args })
    }
    pub fn parse_ret(&mut self) -> Result<Ret<'a>, ParseError> {
        self.consume();
        Ok(Ret {
            expr: self.parse_expr()?,
        })
    }
    pub fn parse_st_ex(&mut self) -> Result<StEx<'a>, ParseError> {
        match (self.cur_tok()?).token_type {
            TT::Tuen => Ok(StEx::Call(self.parse_call()?)),
            TT::LBrace => Ok(StEx::Block(self.parse_block()?)),
            _ => Err(ParseError::UnexpectedToken(
                "StEx".to_string(),
                self.get_tok(),
            )),
        }
    }
    pub fn parse_stmt(&mut self) -> Result<Stmt<'a>, ParseError> {
        let tok = self.cur_tok()?;
        let ret = match tok.token_type {
            TT::Funktion => Ok(Stmt::FunAss(self.parse_fun_ass()?)),
            TT::Dä => Ok(Stmt::VarAss(self.parse_var_ass()?)),
            TT::Gib => Ok(Stmt::Ret(self.parse_ret()?)),
            TT::Tuen | TT::LBrace => Ok(Stmt::StEx(self.parse_st_ex()?)),
            _ => Err(ParseError::UnexpectedToken(
                "Stmt".to_string(),
                self.get_tok(),
            )),
        };
        consume_next_tok!("Stmt".to_string(), self, TT::Semicolon);
        ret
    }
}
