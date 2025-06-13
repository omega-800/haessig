#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;
mod lexer;
use lexer::Tokens;
mod parser;
use parser::Program;

fn main() {
    let matches =
        command!() // requires `cargo` feature
            .arg(arg!([file] "File to evaluate").value_parser(value_parser!(PathBuf)))
            //.arg(
            //    arg!(
            //        -f --file <FILE> "Evaluates a file"
            //    )
            //    .required(false)
            //    .value_parser(value_parser!(PathBuf)),
            //)
            .arg(arg!(
                -d --debug ... "evaluates in debug mode"
            ))
            .subcommand(
                Command::new("repl")
                    .about("starts repl")
                    .arg(arg!([file] "File to load").value_parser(value_parser!(PathBuf)))
                    .arg(arg!(-d --debug "starts in debug mode").action(ArgAction::SetTrue)),
            )
            .get_matches();

    if let Some(file_path) = matches.get_one::<PathBuf>("file") {
        println!("Value for file: {}", file_path.display());
    }

    if let Some(matches) = matches.subcommand_matches("repl") {
        if let Some(file_path) = matches.get_one::<PathBuf>("file") {
            println!("Loading file: {}", file_path.display());
        }
        if matches.get_flag("debug") {
            println!("Starting in debug mode...");
        } else {
            println!("Not printing testing lists...");
        }
    }
    dothething();
}

fn dothething() {
    let input = "
dä wert isch \"sowas\";

funktion hallo_sege {
   tuen schreie mit \"Hallo welt\";
};

tuen hallo_sege;
";
    println!("INPUT:\n{input}");
    let toks = Tokens::from(input);
    println!("TOKS: {}", toks);
    let ast = Program::try_from(&toks);
    println!("AST: {:#?}", ast)
}
