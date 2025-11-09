use std::fmt::Display;

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TT {
    Funktion,
    Tuen,
    Mit,
    Dä,
    Isch,
    Git,
    Gib,
    Als,
    Wahr,
    Falsch,
    TypR8,
    TypN8,
    TypZ8,
    TypZeiche,
    TypWahrheit,
    Str,       // ".*"
    Id,        //
    Num,       //
    LBrace,    // {
    RBrace,    // }
    Semicolon, // ;
    Invalid,   // invalid
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TT,
    pub value: Option<String>,
    pub row: usize,
    pub col: usize,
}

const TOKSTR: [&str; 15] = [
    "funktion ",
    "tuen ",
    "mit ",
    "dä ",
    "isch ",
    "git ",
    "gib ",
    "als ",
    "wahr ",
    "falsch ",
    "R8 ",
    "N8 ",
    "Z8 ",
    "Zahl ",
    "Wahrheit ",
];

impl Token {
    pub fn from_char(ch: char, row: usize, col: &mut usize) -> Self {
        *col += 1;
        Self {
            token_type: match ch {
                '{' => TT::LBrace,
                '}' => TT::RBrace,
                ';' => TT::Semicolon,
                _ => TT::Invalid,
            },
            row,
            col: *col - 1,
            value: None,
        }
    }
    fn new(token_type: TT, value: Option<String>, row: usize, col: usize) -> Self {
        Self {
            token_type,
            row,
            col,
            value,
        }
    }
    fn new_prim(token_type: TT, row: usize, col: usize) -> Self {
        Self {
            token_type,
            row,
            col,
            value: None,
        }
    }
    pub fn from_string(input: &mut str, row: usize, col: &mut usize) -> Self {
        for (i, t) in TOKSTR.iter().enumerate() {
            if input.starts_with(t) {
                let tok =
                    Token::new_prim(unsafe { std::mem::transmute::<u8, TT>(i as u8) }, row, *col);
                *col += t.chars().count();
                return tok;
            }
        }

        let mut try_find = |re: &str, ttype: TT| -> Option<Token> {
            if let Ok(re) = Regex::new(re) {
                if let Some(m) = re.find(input) {
                    let tok = Token::new(ttype, Some(m.as_str().to_owned()), row, *col);
                    *col += m.as_str().chars().count();
                    return Some(tok);
                }
            }
            None
        };

        if let Some(tok) = try_find(r#"^"([^"]|\\")*""#, TT::Str) {
            return tok;
        }
        if let Some(tok) = try_find(r#"^-?[0-9]*([0-9]|([0-9].[0-9]))[0-9]*"#, TT::Num) {
            return tok;
        }
        if let Some(tok) = try_find(r#"^[\p{alpha}_][\p{alpha}0-9_'-]*"#, TT::Id) {
            return tok;
        }
        *col += input.chars().take_while(|c| !c.is_whitespace()).count() + 1;
        Token::new_prim(TT::Invalid, row, *col)
    }
}

#[derive(Debug, PartialEq)]
pub struct Tokens(pub Vec<Token>);

impl<'a> From<&'a str> for Tokens {
    fn from(value: &'a str) -> Self {
        Lexer::new(value.to_string()).lex()
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Tokens(v) = self;
        write!(
            f,
            "[ {} ]",
            v.iter()
                .map(|t| {
                    format!(
                        "{:?}{}",
                        t.token_type,
                        t.value
                            .clone()
                            .map_or("".to_string(), |v| format!(" ({})", v))
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct Lexer {
    input: String,
    row: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            row: 0,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Tokens {
        let mut res: Vec<Token> = Vec::new();
        self.input.lines().for_each(|l| {
            while let Some(ch) = l.chars().nth(self.col) {
                match ch {
                    '{' | '}' | ';' => res.push(Token::from_char(ch, self.row, &mut self.col)),
                    ' ' => self.col += 1,
                    _ => res.push(Token::from_string(
                        &mut l.chars().skip(self.col).collect::<String>(),
                        self.row,
                        &mut self.col,
                    )),
                }
            }
            self.row += 1;
            self.col = 0;
        });
        Tokens(res)
    }
}
