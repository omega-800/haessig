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
        /*
                Prim::Num(num) => format!("
        mov rax, {}
        ", num),
                */
        Prim::Str(str) => format!(
            "
{}
",
            str
        ),
        Prim::Id(id) => todo!(),
    }
}

fn generate_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::FunAss(fun_ass) => todo!(),
        Stmt::VarAss(var_ass) => todo!(),
        Stmt::StEx(st_ex) => todo!(),
    }
}

pub fn generate_fasm(Program(p): &Program) -> String {
    let mut asm = "
format ELF64 executable 3                 ;; ELF64 Format for GNU+Linux
segment readable executable               ;; Executable code section

_start:                                   
"
    .to_string();

    for s in p {
        if let AST::Stmt(stmt) = s {
            asm += &generate_stmt(stmt);
        }
    }

    asm += "
    mov eax, 60                           ;; SYS_exit(                // Call the exit exit(2) syscall
    mov edi, 0                            ;;     EXIT_SUCCESS,        // Exit with success exit code, required if we don't want a segfault
    syscall                               ;; );
";
    asm
}
