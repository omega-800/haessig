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
    assert!(Parser::new(&Lexer::new("{dä x isch 5;}").lex())
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
    assert!(Parser::new(&Lexer::new("funktion f git Zahl {};").lex())
        .parse()
        .is_ok());
}

#[test]
fn fun_ass_with_args() {
    assert!(
        Parser::new(&Lexer::new("funktion f het Zahl x, Zahl y {};").lex())
            .parse()
            .is_ok()
    );
}

#[test]
fn fun_ass_with_args_and_ret() {
    assert!(
        Parser::new(&Lexer::new("funktion f het Zahl x, Zahl y git Zahl {};").lex())
            .parse()
            .is_ok()
    );
}
