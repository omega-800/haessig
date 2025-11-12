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
    NotConvertible(String, TT, PrimType, Token<'a>),
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = "Your code is bonkers";
        let pos = |tok: &Token| format!("row {} col {}", tok.row, tok.col);
        match self {
            ParseError::NoTokensLeft => write!(f, "{}: No tokens left", msg),
            ParseError::TokensLeft => write!(f, "{}: Tokens couldn't be parsed", msg),
            ParseError::NotConvertible(t, from, to, token) => {
                write!(
                    f,
                    "{}: Couldn't convert value of token from {:?} to {} at {} parsing {}",
                    msg,
                    from,
                    to,
                    pos(token),
                    t
                )
            }
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
        let id_tok = $self.get(*$pos).ok_or(ParseError::NoTokensLeft)?;
        if id_tok.token_type != TT::Id {
            return Err(ParseError::ExpectedToken($t, TT::Id, $self[*$pos].clone()));
        }
        let Some(id) = id_tok.value.as_ref() else {
            return Err(ParseError::MissingValue($t, TT::Id, $self[*$pos].clone()));
        };
        *$pos += 1;
        id
    }};
}
macro_rules! consume_next_tok {
    ( $t:expr, $self:ident, $pos:ident, $y:expr ) => {{
        let cur_tok = $self.get(*$pos).ok_or(ParseError::NoTokensLeft)?;
        if cur_tok.token_type != $y {
            return Err(ParseError::ExpectedToken($t, $y, $self[*$pos].clone()));
        }
        *$pos += 1;
    }};
}

pub struct Parser<'a> {
    tokens: &'a Tokens<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens<'a>) -> Self {
        Self { tokens }
    }
    pub fn parse(&mut self) -> Result<Program<'a>, ParseError> {
        let mut ast = vec![];
        let mut pos: usize = 0;
        while pos < self.tokens.len() {
            let stmt = Stmt::parse(self.tokens, &mut pos)?;
            ast.push(stmt);
        }
        Ok(ast)
    }
}

pub trait Parseable<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>>
    where
        Self: Sized;
}

impl<'a> Parseable<'a> for Stmt<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        let tok = tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?;
        let ret = match tok.token_type {
            TT::Funktion => Ok(Stmt::FunAss(FunAss::parse(tokens, pos)?)),
            TT::Dä => Ok(Stmt::VarAss(VarAss::parse(tokens, pos)?)),
            TT::Gib => Ok(Stmt::Ret(Ret::parse(tokens, pos)?)),
            TT::Tuen | TT::LBrace => Ok(Stmt::StEx(StEx::parse(tokens, pos)?)),
            _ => Err(ParseError::UnexpectedToken(
                "Stmt".to_string(),
                tokens[*pos].clone(),
            )),
        };

        consume_next_tok!("Stmt".to_string(), tokens, pos, TT::Semicolon);
        ret
    }
}

impl<'a> Parseable<'a> for FunAss<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        *pos += 1;
        let id = expect_id_next!("FunAss".to_string(), tokens, pos);
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
        let body = Block::parse(tokens, pos)?;

        Ok(FunAss { id, body, ret })
    }
}

impl<'a> Parseable<'a> for VarAss<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        *pos += 1;
        let id = expect_id_next!("VarAss".to_string(), tokens, pos);
        consume_next_tok!("VarAss".to_string(), tokens, pos, TT::Isch);
        let value = Expr::parse(tokens, pos)?;
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
}
impl<'a> Parseable<'a> for Expr<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        match (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type {
            TT::Tuen | TT::LBrace => Ok(Expr::StEx(StEx::parse(tokens, pos)?)),
            _ => Ok(Expr::Prim(Prim::parse(tokens, pos)?)),
        }
    }
}
impl<'a> Parseable<'a> for Block<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        *pos += 1;
        let mut stmts = vec![];
        while (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type != TT::RBrace {
            stmts.push(Stmt::parse(tokens, pos)?);
        }
        consume_next_tok!("Block".to_string(), tokens, pos, TT::RBrace);
        Ok(Block { stmts })
    }
}
impl<'a> Parseable<'a> for Prim<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        let tok = tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?;
        match tok.token_type {
            TT::Num => Ok(Prim::R8(
                tok.value
                    .ok_or(ParseError::MissingValue(
                        "Prim".to_string(),
                        TT::Str,
                        tokens[*pos].clone(),
                    ))?
                    .parse()
                    .map_err(|_| {
                        ParseError::NotConvertible(
                            "Prim".to_string(),
                            TT::Str,
                            PrimType::R8,
                            tokens[*pos].clone(),
                        )
                    })?,
            )),
            TT::Wahr => Ok(Prim::Bool(true)),
            TT::Falsch => Ok(Prim::Bool(false)),
            TT::Str => Ok(Prim::Str(tok.value.ok_or(ParseError::MissingValue(
                "Prim".to_string(),
                TT::Str,
                tokens[*pos].clone(),
            ))?)),
            TT::Id => Ok(Prim::Id(tok.value.ok_or(ParseError::MissingValue(
                "Prim".to_string(),
                TT::Id,
                tokens[*pos].clone(),
            ))?)),
            _ => Err(ParseError::ExpectedPrim(
                "Prim".to_string(),
                tokens[*pos].clone(),
            )),
        }
    }
}
impl<'a> Parseable<'a> for Call<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        *pos += 1;
        let id = expect_id_next!("Call".to_string(), tokens, pos);
        consume_next_tok!("Call".to_string(), tokens, pos, TT::Mit);
        let mut args = vec![];
        let mut prev = *pos;
        loop {
            if let Ok(a) = Expr::parse(tokens, pos) {
                args.push(a);
                prev = *pos;
            } else {
                *pos = prev;
                break;
            }
            if (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type != TT::Comma {
                break;
            }
            *pos += 1;
        }
        Ok(Call { id, args })
    }
}
impl<'a> Parseable<'a> for Ret<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        *pos += 1;
        Ok(Ret {
            expr: Expr::parse(tokens, pos)?,
        })
    }
}
impl<'a> Parseable<'a> for StEx<'a> {
    fn parse(tokens: &'a [Token<'a>], pos: &mut usize) -> Result<Self, ParseError<'a>> {
        match (tokens.get(*pos).ok_or(ParseError::NoTokensLeft)?).token_type {
            TT::Tuen => Ok(StEx::Call(Call::parse(tokens, pos)?)),
            TT::LBrace => Ok(StEx::Block(Block::parse(tokens, pos)?)),
            _ => Err(ParseError::UnexpectedToken(
                "StEx".to_string(),
                tokens[*pos].clone(),
            )),
        }
    }
}
