use crate::parser::{Prim, Program, Stmt, AST};

fn generate_print(out: &str) -> String {
    "
mov eax, 1                            ;; sys_write(               // Call the write(2) syscall
mov edi, 1                            ;;     STDOUT_FILENO,       // Write to stdout
mov esi, hello_world                  ;;     hello_world,         // Buffer to write to STDOUT_FILENO: hello_world
mov edx, hello_world_length           ;;     hello_world_length,  // Buffer length
syscall                               ;; );
".to_string()
}

fn generate_prim(prim: &Prim) -> String {
    match prim {
        Prim::Str(str) => format!(
                "
{}
",
                str
            ),
        Prim::Id(id) => todo!(),
        Prim::U8(_) => todo!(),
    }
}

fn generate_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::FunAss(fun_ass) => todo!(),
        Stmt::VarAss(var_ass) => todo!(),
        Stmt::StEx(st_ex) => todo!(),
    }
}

pub struct Compiler<'a> {
    input: Program<'a>,
    stack_size: u64,
    output: String,
}

impl<'a> Compiler<'a> {
    pub fn new(input: Program<'a>) -> Self {
        Self {
            input,
            stack_size: 0,
            output: "
format ELF64 executable 3                 ;; ELF64 Format for GNU+Linux
segment readable executable               ;; Executable code section

_start:                                   
"
            .to_string(),
        }
    }

    pub fn compile(&mut self) -> String {
        let Program(p) = &self.input;
        for s in p {
            match s {
                AST::Stmt(stmt) => self.output += &generate_stmt(stmt),
                AST::Expr(expr) => todo!(),
            }
        }

        self.output += "
    mov eax, 60                           ;; SYS_exit(                // Call the exit exit(2) syscall
    mov edi, 0                            ;;     EXIT_SUCCESS,        // Exit with success exit code, required if we don't want a segfault
    syscall                               ;; );
";
        self.output.clone()
    }
}
