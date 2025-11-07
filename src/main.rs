#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use clap::{arg, command, value_parser, ArgAction, Command};
use compiler::generate_fasm;
use std::{
    fs::{create_dir, exists, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
mod compiler;
mod lexer;
mod parser;
use lexer::Tokens;
use parser::Program;

fn main() {
    // TODO: remove or implement
    /*
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
    */
    dothething();
}

fn write(filename: &str, content: &str) {
    let path = &("./.build/".to_owned() + filename);
    match File::create(path) {
        Ok(mut file) => {
            if let Err(err) = write!(file, "{}", content) {
                eprintln!("Failed to write file {}: {}", path, err);
            } else {
                println!("Wrote file {}", path);
            }
        }
        Err(err) => {
            eprintln!("Failed to create file {}: {}", path, err);
        }
    }
}

fn dothething() {
    let input = "
funktion hallo_sege {
  tuen schreie mit \"Hallo welt\";
};

funktion chuchichäschtli {
  dä wert isch \"sowas\";
  tuen hallo_sege mit wert;
};
";

    if !exists("./.build").unwrap_or(false) {
        if let Err(err) = create_dir("./.build") {
            eprintln!("Failed to create build dir: {}", err);
        }
    }
    //println!("INPUT:\n{input}");
    write("input.hä", input);
    let toks = Tokens::from(input);
    write("tokens.txt", &format!("{}", toks));
    //println!("TOKS:\n{}", toks);
    let ast = Program::try_from(&toks);
    //println!("AST:\n{:#?}", ast);
    write("ast.txt", &format!("{:#?}", ast));
    match ast {
        Ok(ast) => {
            let fasm = generate_fasm(&ast);
            //println!("FASM:\n{fasm}");
            write("fasm.asm", &fasm);
            match std::process::Command::new("fasm")
                .arg("./.build/fasm.asm")
                .arg("./.build/out")
                .output()
            {
                Ok(out) => {
                    println!("Linking status: {}", out.status);
                    let _ = io::stdout().write_all(&out.stdout);
                    let _ = io::stderr().write_all(&out.stderr);
                }
                Err(err) => eprintln!("Failed to link executable: {}", err),
            }
        }
        Err(err) => eprintln!("Failed to parse: {}", err),
    }
}
