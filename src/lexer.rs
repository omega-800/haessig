use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum TT {
    Funktion,
    Tuen,
    Mit,
    Dä,
    Isch,
    Het,
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
    Comma,     // ,
    Invalid,   // invalid
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub token_type: TT,
    pub value: Option<&'a str>,
    pub row: usize,
    pub col: usize,
}

// TODO: use separators instead of space
const TOKSTR: [&str; 16] = [
    "funktion ",
    "tuen ",
    "mit ",
    "dä ",
    "isch ",
    "het ",
    "git ",
    "gib ",
    "als ",
    "wahr ",
    "falsch ",
    "R8 ",
    "N8 ",
    "Z8 ",
    "Zeiche ",
    "Wahrheit ",
];

impl<'a> Token<'a> {
    pub fn from_char(ch: char, row: usize, col: &mut usize) -> Self {
        *col += 1;
        Self {
            token_type: match ch {
                '{' => TT::LBrace,
                '}' => TT::RBrace,
                ';' => TT::Semicolon,
                ',' => TT::Comma,
                _ => TT::Invalid,
            },
            row,
            col: *col - 1,
            value: None,
        }
    }
    fn new(token_type: TT, value: Option<&'a str>, row: usize, col: usize) -> Self {
        Self {
            token_type,
            row,
            col,
            value,
        }
    }
    fn new_builtin(token_type: TT, row: usize, col: usize) -> Self {
        Self {
            token_type,
            row,
            col,
            value: None,
        }
    }
    pub fn from_string(input: &'a str, row: usize, col: &mut usize) -> Self {
        for (i, t) in TOKSTR.iter().enumerate() {
            if input.starts_with(t) {
                let tok = Token::new_builtin(
                    unsafe { std::mem::transmute::<u8, TT>(i as u8) },
                    row,
                    *col,
                );
                *col += t.chars().count();
                return tok;
            }
        }

        let mut try_find = |re: &str, ttype: TT| -> Option<Token> {
            if let Ok(re) = Regex::new(re) {
                if let Some(m) = re.find(input) {
                    let tok = Token::new(ttype, Some(m.into()), row, *col);
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
        Token::new_builtin(TT::Invalid, row, *col)
    }
}

pub type Tokens<'a> = Vec<Token<'a>>;

#[allow(dead_code)]
fn display_tokens(v: &Tokens) -> String {
    format!(
        "[ {} ]",
        v.iter()
            .map(|t| {
                format!(
                    "{:?}{}",
                    t.token_type,
                    t.value.map_or("".to_string(), |v| format!(" ({})", v))
                )
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub struct Lexer<'a> {
    input: &'a str,
    row: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            row: 0,
            col: 0,
        }
    }

    pub fn lex(&mut self) -> Tokens<'a> {
        let mut res: Vec<Token<'a>> = Vec::new();
        self.input.lines().for_each(|l| {
            while let Some(ch) = l.chars().nth(self.col) {
                match ch {
                    '{' | '}' | ';' | ',' => {
                        res.push(Token::from_char(ch, self.row, &mut self.col))
                    }
                    _ if ch.is_whitespace() => self.col += 1,
                    _ => {
                        let mut l_iter = l.chars();
                        (&mut l_iter).take(self.col).count();
                        res.push(Token::from_string(l_iter.as_str(), self.row, &mut self.col))
                    }
                }
            }
            self.row += 1;
            self.col = 0;
        });
        res
    }
}
