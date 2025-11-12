use haessig::{lexer::Lexer, parser::Parser};

#[test]
fn var_ass() {
    assert!(Parser::new(&Lexer::new("dä x isch 5;").lex())
        .parse()
        .is_ok());
}

#[test]
fn var_ass_str() {
    assert!(Parser::new(&Lexer::new("dä x isch \"5\";").lex())
        .parse()
        .is_ok());
}

#[test]
fn var_ass_bool() {
    assert!(Parser::new(&Lexer::new("dä x isch wahr;").lex())
        .parse()
        .is_ok());
}

#[test]
fn block() {
    assert!(Parser::new(&Lexer::new("{dä x isch 5;};").lex())
        .parse()
        .is_ok());
}

#[test]
fn fun_ass() {
    assert!(Parser::new(&Lexer::new("funktion f {};").lex())
        .parse()
        .is_ok());
}

#[test]
fn fun_ass_with_ret() {
    assert!(Parser::new(&Lexer::new("funktion f git N8 {};").lex())
        .parse()
        .is_ok());
}

#[test]
fn fun_ass_with_args() {
    assert!(
        Parser::new(&Lexer::new("funktion f het N8 x, N8 y {};").lex())
            .parse()
            .is_ok()
    );
}

#[test]
fn fun_ass_with_args_and_ret() {
    assert!(
        Parser::new(&Lexer::new("funktion f het N8 x, N8 y git N8 {};").lex())
            .parse()
            .is_ok()
    );
}
