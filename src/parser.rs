use std::fmt::Display;

use crate::lexer::{Token, Tokens, TT};

// FIXME: fix the wonky *pos+=1 and better schwiizerdütschi errors

#[derive(Debug, Clone)]
pub struct Program<'a>(pub Vec<AST<'a>>);

impl<'a> TryFrom<&'a Tokens> for Program<'a> {
    type Error = ParseError;
    fn try_from(value: &'a Tokens) -> Result<Self, Self::Error> {
        Parser::parse(value)
    }
}

pub trait Parseable<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub enum StEx<'a> {
    Call(Call<'a>),
    Block(Block<'a>),
}

impl<'a> Parseable<'a> for StEx<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        if let Ok(s) = Call::parse(tokens, pos) {
            return Ok(StEx::Call(s));
        }
        if let Ok(s) = Block::parse(tokens, pos) {
            return Ok(StEx::Block(s));
        }
        Err(ParseError::new(
            tokens,
            pos,
            "Couldn't parse `StEx`".to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    FunAss(FunAss<'a>),
    VarAss(VarAss<'a>),
    StEx(StEx<'a>),
}

impl<'a> Parseable<'a> for Stmt<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        // TODO: stop being shit at coding and dry
        let mut ret = None;
        if let Ok(s) = FunAss::parse(tokens, pos) {
            ret = Some(Stmt::FunAss(s));
        }
        if let Ok(s) = VarAss::parse(tokens, pos) {
            ret = Some(Stmt::VarAss(s));
        }
        if let Ok(s) = StEx::parse(tokens, pos) {
            ret = Some(Stmt::StEx(s));
        }
        if let Some(ret) = ret {
            if tokens
                .get(*pos)
                .map_or(false, |t| t.token_type == TT::Semicolon)
            {
                *pos += 1;
                return Ok(ret);
            } else {
                return Err(ParseError::new(
                    tokens,
                    pos,
                    "Expected semicolon `;`".to_string(),
                ));
            }
        }
        Err(ParseError::new(
            tokens,
            pos,
            "Couldn't parse `Stmt`".to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    StEx(StEx<'a>),
    Prim(Prim<'a>),
}

impl<'a> Parseable<'a> for Expr<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        if let Ok(s) = StEx::parse(tokens, pos) {
            return Ok(Expr::StEx(s));
        }
        if let Ok(s) = Prim::parse(tokens, pos) {
            return Ok(Expr::Prim(s));
        }
        Err(ParseError::new(
            tokens,
            pos,
            "Couldn't parse `Expr`".to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub enum AST<'a> {
    Stmt(Stmt<'a>),
    Expr(Expr<'a>),
}

#[derive(Debug, Clone)]
pub enum Prim<'a> {
    Str(&'a str),
    Id(&'a str),
}

impl<'a> Parseable<'a> for Prim<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        let Some(t) = tokens.get(*pos) else {
            return Err(ParseError {
                pos: *pos,
                row: 0,
                col: 0,
                message: "No tokens left".to_string(),
            });
        };
        *pos += 1;
        match t.token_type {
            TT::Str => Ok(Prim::Str(t.value.as_ref().map_or("", |v| v))),
            TT::Id => Ok(Prim::Id(t.value.as_ref().map_or("", |v| v))),
            _ => {
                *pos -= 1;
                Err(ParseError::new(
                    tokens,
                    pos,
                    "Couldn't parse `Prim`".to_string(),
                ))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block<'a> {
    stmts: Vec<Stmt<'a>>,
}
impl<'a> Parseable<'a> for Block<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        let Some(first) = tokens.get(*pos) else {
            return Err(ParseError {
                pos: *pos,
                row: 0,
                col: 0,
                message: "No tokens left".to_string(),
            });
        };
        if first.token_type != TT::LBrace {
            return Err(ParseError::new(
                tokens,
                pos,
                "Couldn't parse `Prim`".to_string(),
            ));
        }
        *pos += 1;
        let mut stmts = vec![];
        while let Ok(s) = Stmt::parse(tokens, pos) {
            // FIXME: pos fucked after consumed stmts
            stmts.push(s);
        }
        let Some(last) = tokens.get(*pos) else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Couldn't parse `Prim`".to_string(),
            ));
        };
        if last.token_type != TT::RBrace {
            return Err(ParseError::new(
                tokens,
                pos,
                "Couldn't parse `Prim`".to_string(),
            ));
        }
        *pos += 1;
        Ok(Block { stmts })
    }
}

#[derive(Debug, Clone)]
pub struct VarAss<'a> {
    id: &'a str,
    value: Expr<'a>,
}

impl<'a> Parseable<'a> for VarAss<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        let Some((fstthree, _rest)) = tokens[*pos..].split_at_checked(3) else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Not enough tokens left".to_string(),
            ));
        };
        if fstthree[0].token_type != TT::Dä
            || fstthree[1].token_type != TT::Id
            || fstthree[2].token_type != TT::Isch
        {
            return Err(ParseError::new(
                tokens,
                pos,
                "First token isn't `Dä` or second token isn't `Id` or third token isn't `Isch`"
                    .to_string(),
            ));
        }
        let Some(id) = fstthree[1].value.as_ref() else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Second token doesn't have value for `Id`".to_string(),
            ));
        };
        *pos += 3;
        if let Ok(value) = Expr::parse(tokens, pos) {
            return Ok(VarAss { id, value });
        }
        *pos -= 3;
        Err(ParseError::new(
            tokens,
            pos,
            "Couldn't parse `VarAss`".to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct FunAss<'a> {
    id: &'a str,
    body: Block<'a>,
}

impl<'a> Parseable<'a> for FunAss<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        let Some((fsttwo, _rest)) = tokens[*pos..].split_at_checked(2) else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Not enough tokens left".to_string(),
            ));
        };
        if fsttwo[0].token_type != TT::Funktion || fsttwo[1].token_type != TT::Id {
            return Err(ParseError::new(
                tokens,
                pos,
                "First token isn't `Funktion` or second token isn't `Id`".to_string(),
            ));
        }
        let Some(id) = fsttwo[1].value.as_ref() else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Second token doesn't have value for `Id`".to_string(),
            ));
        };
        *pos += 2;
        let Ok(body) = Block::parse(tokens, pos) else {
            *pos -= 2;
            return Err(ParseError::new(
                tokens,
                pos,
                "Couldn't parse body".to_string(),
            ));
        };
        Ok(FunAss { id, body })
    }
}

#[derive(Debug, Clone)]
pub struct Call<'a> {
    id: &'a str,
    args: Vec<Expr<'a>>,
}

impl<'a> Parseable<'a> for Call<'a> {
    fn parse(tokens: &'a [Token], pos: &mut usize) -> Result<Self, ParseError> {
        let Some((fsttwo, _rest)) = tokens[*pos..].split_at_checked(2) else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Not enough tokens left".to_string(),
            ));
        };
        if fsttwo[0].token_type != TT::Tuen || fsttwo[1].token_type != TT::Id {
            return Err(ParseError::new(
                tokens,
                pos,
                "First token isn't `Tuen` or second token isn't `Id`".to_string(),
            ));
        }
        let Some(id) = fsttwo[1].value.as_ref() else {
            return Err(ParseError::new(
                tokens,
                pos,
                "Second token doesn't have value for `Id`".to_string(),
            ));
        };
        *pos += 2;

        let mut args = vec![];
        if let Some(mit) = tokens.get(*pos) {
            if mit.token_type == TT::Mit {
                *pos += 1;
                while let Ok(a) = Expr::parse(tokens, pos) {
                    args.push(a);
                }
            }
        }

        Ok(Call { id, args })
    }
}

pub struct Parser<'a> {
    input: &'a Tokens,
    pos: usize,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pos: usize,
    row: usize,
    col: usize,
    message: String,
}

impl ParseError {
    fn new(tokens: &[Token], pos: &mut usize, message: String) -> Self {
        Self {
            pos: *pos,
            row: tokens.get(*pos).map_or(0, |t| t.row),
            col: tokens.get(*pos).map_or(0, |t| t.col),
            message,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Your code is shit :)")
    }
}

impl<'a> Parser<'a> {
    /*
    pub fn new(input: &'a Tokens) -> Self {
        Self { input, pos: 0 }
    }
    */
    pub fn parse(Tokens(t): &'a Tokens) -> Result<Program<'a>, ParseError> {
        let mut pos: usize = 0;
        let mut ast = vec![];
        while pos < t.len() {
            ast.push(AST::Stmt(Stmt::parse(t, &mut pos)?));
        }
        Ok(Program(ast))
    }
}
