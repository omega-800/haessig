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
pub enum ParseError<'a> {
    NoTokensLeft,
    TokensLeft,
    UnexpectedToken(String, Token<'a>),
    ExpectedToken(String, TT, Token<'a>),
    ExpectedType(String, Token<'a>),
    ExpectedPrim(String, Token<'a>),
    MissingValue(String, TT, Token<'a>),
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = "Your code is bonkers";
        let pos = |tok: &Token| format!("row {} col {}", tok.row, tok.col);
        match self {
            ParseError::NoTokensLeft => write!(f, "{}: No tokens left", msg),
            ParseError::TokensLeft => write!(f, "{}: Tokens couldn't be parsed", msg),
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
    ( $t:expr, $self:ident, $pos:ident ) => {{
        // TODO: wrap my brain around why this doesn't work
        //$self.consume();
        *$pos += 1;
        // TODO: as well as this
        //let id_tok = $self.cur_tok()?;
        let id_tok = $self.get(*$pos).ok_or(ParseError::NoTokensLeft)?;
        if id_tok.token_type != TT::Id {
            return Err(ParseError::ExpectedToken($t, TT::Id, $self[*$pos].clone()));
        }
        let Some(id) = id_tok.value.as_ref() else {
            return Err(ParseError::MissingValue($t, TT::Id, $self[*$pos].clone()));
        };
        id
    }};
}
macro_rules! consume_next_tok {
    ( $t:expr, $self:ident, $pos:ident, $y:expr ) => {{
        let cur_tok = $self.get(*$pos).ok_or(ParseError::NoTokensLeft)?;
        if cur_tok.token_type != $y {
            return Err(ParseError::ExpectedToken($t, $y, $self[*$pos].clone()));
        }
        //$self.consume();
        *$pos += 1;
    }};
}

pub struct Parser<'a> {
    tokens: &'a Tokens<'a>,
    //pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens<'a>) -> Self {
        Self {
            tokens, /*, pos: 0 */
        }
    }
    pub fn parse(&mut self) -> Result<Program<'a>, ParseError> {
        let mut ast = vec![];
        let mut pos: usize = 0;
        /*
                while let Ok(stmt) = parse_stmt(self.tokens, &mut pos) {
                    ast.push(stmt);
                }
                if pos < self.tokens.len() {
                    return Err(ParseError::TokensLeft);
                }
        */
                while pos < self.tokens.len() {
                    let stmt = parse_stmt(self.tokens, &mut pos)?;
                    ast.push(stmt);
                }
        Ok(ast)
    }
    /*
    pub fn consume(&mut self) {
        self.pos += 1;
    }
    pub fn cur_tok(&self, pos: usize) -> Result<&'a Token<'a>, ParseError> {
        self.tokens.get(pos).ok_or(ParseError::NoTokensLeft)
    }
    pub fn get_tok(&self, pos: usize) -> Token<'a> {
        self.tokens[pos].clone()
    }
    */
}

pub fn parse_stmt<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<Stmt<'a>, ParseError<'a>> {
    let tok = tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?;
    let ret = match tok.token_type {
        TT::Funktion => Ok(Stmt::FunAss(parse_fun_ass(tokens, pos)?)),
        TT::Dä => Ok(Stmt::VarAss(parse_var_ass(tokens, pos)?)),
        TT::Gib => Ok(Stmt::Ret(parse_ret(tokens, pos)?)),
        TT::Tuen | TT::LBrace => Ok(Stmt::StEx(parse_st_ex(tokens, pos)?)),
        _ => Err(ParseError::UnexpectedToken(
            "Stmt".to_string(),
            tokens[*pos].clone(),
        )),
    };

    consume_next_tok!("Stmt".to_string(), tokens, pos, TT::Semicolon);
    ret
}
pub fn parse_fun_ass<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<FunAss<'a>, ParseError<'a>> {
    let id = expect_id_next!("FunAss".to_string(), tokens, pos);
    *pos += 1;
    //TODO: args
    let mut ret = None;
    let git_tok = tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?;
    if git_tok.token_type == TT::Git {
        *pos += 1;
        let type_tok = tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?;

        if let Some(prim_type) = PrimType::from_tt(type_tok.token_type) {
            *pos += 1;
            ret = Some(prim_type);
        } else {
            return Err(ParseError::ExpectedType(
                "FunAss".to_string(),
                tokens[*pos].clone(),
            ));
        }
    }
    let body = parse_block(tokens, pos)?;

    Ok(FunAss { id, body, ret })
}
pub fn parse_var_ass<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<VarAss<'a>, ParseError<'a>> {
    let id = expect_id_next!("VarAss".to_string(), tokens, pos);
    *pos += 1;
    consume_next_tok!("VarAss".to_string(), tokens, pos, TT::Isch);
    let value = parse_expr(tokens, pos)?;
    let mut pt = None;
    if (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type == TT::Als {
        *pos += 1;
        pt = PrimType::from_tt((tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type);
        *pos += 1;
        if pt.is_none() {
            return Err(ParseError::ExpectedType(
                "VarAss".to_string(),
                tokens[*pos].clone(),
            ));
        }
    }
    Ok(VarAss { id, value, pt })
}
pub fn parse_expr<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<Expr<'a>, ParseError<'a>> {
    match (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type {
        TT::Tuen | TT::LBrace => Ok(Expr::StEx(parse_st_ex(tokens, pos)?)),
        _ => Ok(Expr::Prim(parse_prim(tokens, pos)?)),
    }
}
pub fn parse_block<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<Block<'a>, ParseError<'a>> {
    *pos += 1;
    let mut stmts = vec![];
    while (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type != TT::RBrace {
        stmts.push(parse_stmt(tokens, pos)?);
    }
    consume_next_tok!("Block".to_string(), tokens, pos, TT::RBrace);
    Ok(Block { stmts })
}
pub fn parse_prim<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<Prim<'a>, ParseError<'a>> {
    let tok = tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?;
    match tok.token_type {
        // TODO: error handling
        TT::Num => Ok(Prim::R8(
            tok.value.as_ref().map_or(0, |v| v.parse().unwrap_or(0)),
        )),
        TT::Wahr => Ok(Prim::Bool(true)),
        TT::Falsch => Ok(Prim::Bool(false)),
        // FIXME: fight the borrow checker harder
        // TT::Str => Ok(Prim::Str(tok.value.as_ref().map_or("", |v| v))),
        // TT::Id => Ok(Prim::Id(tok.value.as_ref().map_or("", |v| v))),
        _ => Err(ParseError::ExpectedPrim(
            "Prim".to_string(),
            tokens[*pos].clone(),
        )),
    }
}
pub fn parse_call<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<Call<'a>, ParseError<'a>> {
    let id = expect_id_next!("Call".to_string(), tokens, pos);
    *pos += 1;
    consume_next_tok!("Call".to_string(), tokens, pos, TT::Mit);
    let mut args = vec![];
    let mut prev = *pos;
    loop {
        if let Ok(a) = parse_expr(tokens, pos) {
            args.push(a);
            prev = *pos;
        } else {
            *pos = prev;
        }
        if (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type != TT::Comma {
            break;
        }
        *pos += 1;
    }
    Ok(Call { id, args })
}
pub fn parse_ret<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<Ret<'a>, ParseError<'a>> {
    *pos += 1;
    Ok(Ret {
        expr: parse_expr(tokens, pos)?,
    })
}
pub fn parse_st_ex<'a>(
    tokens: &'a [Token<'a>],
    pos: &mut usize,
) -> Result<StEx<'a>, ParseError<'a>> {
    match (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type {
        TT::Tuen => Ok(StEx::Call(parse_call(tokens, pos)?)),
        TT::LBrace => Ok(StEx::Block(parse_block(tokens, pos)?)),
        _ => Err(ParseError::UnexpectedToken(
            "StEx".to_string(),
            tokens[*pos].clone(),
        )),
    }
}
