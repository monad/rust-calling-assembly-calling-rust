global hello_world
extern rust_code
section .text
hello_world:
        mov rbx, rdi
        push rdi
        mov rdi, rbx
        call rust_code
        pop rdi
        mov rax, rbx
        ret
