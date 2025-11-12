use haessig::lexer::{Lexer, Token,  TT};

#[test]
fn var_ass() {
    let res = Lexer::new(
        "
funktion test git Wahrheit {
    gib falsch;
}
"
    )
    .lex();
    let exp = vec![
        (Token {
            token_type: TT::Funktion,
            value: None,
            row: 1,
            col: 0,
        }),
        (Token {
            token_type: TT::Id,
            value: Some("test"),
            row: 1,
            col: 9,
        }),
        (Token {
            token_type: TT::Git,
            value: None,
            row: 1,
            col: 14,
        }),
        (Token {
            token_type: TT::TypWahrheit,
            value: None,
            row: 1,
            col: 18,
        }),
        (Token {
            token_type: TT::LBrace,
            value: None,
            row: 1,
            col: 27,
        }),
        (Token {
            token_type: TT::Gib,
            value: None,
            row: 2,
            col: 4,
        }),
        (Token {
            token_type: TT::Id,
            value: Some("falsch"),
            row: 2,
            col: 8,
        }),
        (Token {
            token_type: TT::Semicolon,
            value: None,
            row: 2,
            col: 14,
        }),
        (Token {
            token_type: TT::RBrace,
            value: None,
            row: 3,
            col: 0,
        }),
    ];
    assert_eq!(res, exp);
}
