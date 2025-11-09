use crate::interm::IntermRepr;

fn generate_print(out: &str) -> String {
    "
mov eax, 1                            ;; sys_write(               // Call the write(2) syscall
mov edi, 1                            ;;     STDOUT_FILENO,       // Write to stdout
mov esi, hello_world                  ;;     hello_world,         // Buffer to write to STDOUT_FILENO: hello_world
mov edx, hello_world_length           ;;     hello_world_length,  // Buffer length
syscall                               ;; );
".to_string()
}

pub struct Compiler<'a> {
    input: &'a IntermRepr<'a>,
    stack_size: u64,
    output: String,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'a IntermRepr<'a>) -> Self {
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
        self.output += "
    mov eax, 60                           ;; SYS_exit(                // Call the exit exit(2) syscall
    mov edi, 0                            ;;     EXIT_SUCCESS,        // Exit with success exit code, required if we don't want a segfault
    syscall                               ;; );
";
        self.output.clone()
    }
}
