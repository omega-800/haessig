#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use std::{
    fs::{create_dir, exists, File},
    io::{self, Write},
    process::Command,
};
mod compiler;
mod interm;
mod lexer;
mod parser;
mod seman;
mod trans;
use seman::SemanticAnalyzer;

use crate::{lexer::Lexer, parser::Parser, trans::Transpiler};

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
funktion test het N8 x, N8 y git Wahrheit {
    tuen schreie mit \"asdf\";
    gib falsch;
};
funktion chuchichäschtli git Z8 {
    dä x isch 5 plus 5 minus 7 als N8;
    tuen test mit 8, x;
};
"
    /*"
    funktion hallo_sege {
      tuen schreie mit \"Hallo welt\";
    };
    funktion chuchichäschtli {
      dä wert isch \"sowas\";
      tuen hallo_sege mit wert;
    };
    "*/
    .to_string();

    // TODO: ffi && raylib speedrun

    if !exists("./.build").unwrap_or(false) {
        if let Err(err) = create_dir("./.build") {
            eprintln!("Failed to create build dir: {}", err);
        }
    }
    //println!("INPUT:\n{input}");
    write("input.hä", &input);
    let toks = Lexer::new(&input).lex();
    write("tokens.txt", &format!("{:#?}", toks));
    //println!("TOKS:\n{}", toks);
    //FIXME: lifetime of the ast shouldn't be tied to the lifetime of the parser
    let mut parser = Parser::new(&toks);
    let ast = parser.parse();
    //println!("AST:\n{:#?}", ast);
    match ast {
        Ok(ast) => {
            write("ast.txt", &format!("{:#?}", ast));
            match SemanticAnalyzer::new(&ast).analyze() {
                Ok(_) => {
                    let c_99 = Transpiler::new(&ast).generate();
                    write("c99.c", &c_99);
                    match Command::new("gcc")
                        .arg("./.build/c99.c")
                        .arg("-o")
                        .arg("./.build/out")
                        .output()
                    {
                        Ok(out) => {
                            println!("Compilation status: {}", out.status);
                            let _ = io::stdout().write_all(&out.stdout);
                            let _ = io::stderr().write_all(&out.stderr);
                        }
                        Err(err) => eprintln!("Failed to compile executable: {}", err),
                    }

                    // //let ir = IRGen::new(&ast).generate();
                    // //let fasm = Compiler::new(&ir).compile();
                    // let fasm = "".to_string();
                    // //println!("FASM:\n{fasm}");
                    // write("fasm.asm", &fasm);
                    // match Command::new("fasm")
                    //     .arg("./.build/fasm.asm")
                    //     .arg("./.build/out")
                    //     .output()
                    // {
                    //     Ok(out) => {
                    //         println!("Linking status: {}", out.status);
                    //         let _ = io::stdout().write_all(&out.stdout);
                    //         let _ = io::stderr().write_all(&out.stderr);
                    //     }
                    //     Err(err) => eprintln!("Failed to link executable: {}", err),
                    // }
                }
                Err(err) => eprintln!("Failed to analyze: {}", err),
            }
        }
        Err(err) => eprintln!("Failed to parse: {}", err),
    }
}
