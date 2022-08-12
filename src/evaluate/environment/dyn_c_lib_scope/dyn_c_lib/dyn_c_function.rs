use std::arch::asm;
use std::rc::Rc;
use super::{ DynCSym, LibPtr };

#[derive(Clone, Debug)]
pub struct DynCFunction {
    ptr: DynCSym,

    // This prevents lib ptr is in from dropping
    lib: Rc<LibPtr>,
}

impl DynCFunction {
    pub fn new(ptr: DynCSym, lib: Rc<LibPtr>) -> DynCFunction {
        DynCFunction {
            ptr,
            lib,
        }
    }

    pub fn call(&self, args: Vec<usize>) -> usize {
        let result;

        unsafe {
            match args.len() {
                0 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    out("rax") result,
                    clobber_abi("C"),
                ),
                1 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    in("rdi") args[0],
                    out("rax") result,
                    clobber_abi("C"),
                ),
                2 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    in("rdi") args[0],
                    in("rsi") args[1],
                    out("rax") result,
                    clobber_abi("C"),
                ),
                3 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    in("rdi") args[0],
                    in("rsi") args[1],
                    in("rdx") args[2],
                    out("rax") result,
                    clobber_abi("C"),
                ),
                4 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    in("rdi") args[0],
                    in("rsi") args[1],
                    in("rdx") args[2],
                    in("rcx") args[3],
                    out("rax") result,
                    clobber_abi("C"),
                ),
                5 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    in("rdi") args[0],
                    in("rsi") args[1],
                    in("rdx") args[2],
                    in("rcx") args[3],
                    in("r8") args[4],
                    out("rax") result,
                    clobber_abi("C"),
                ),
                6 => asm!(
                    "call {}",
                    in(reg) self.ptr,
                    in("rdi") args[0],
                    in("rsi") args[1],
                    in("rdx") args[2],
                    in("rcx") args[3],
                    in("r8") args[4],
                    in("r9") args[5],
                    out("rax") result,
                    clobber_abi("C"),
                ),
                len => {
                    let arr = args.into_boxed_slice();

                    asm!(
                        "push rbp",
                        "mov rbp, rsp",

                        "mov rax, {0}",
                        "2:",
                        "cmp rax, 5",
                        "jle 2f",
                        "mov r15, rax",
                        "shl r15, 3",
                        "add r15, {1}",

                        "push [r15]",
                        "sub rax, 1",
                        "jmp 2b",

                        "2:",
                        "xor rax, rax",
                        "call {2}",

                        "mov rsp, rbp",
                        "pop rbp",

                        in(reg) len - 1,
                        in(reg) arr.as_ptr(),
                        in(reg) self.ptr,
                        in("rdi") arr[0],
                        in("rsi") arr[1],
                        in("rdx") arr[2],
                        in("rcx") arr[3],
                        in("r8") arr[4],
                        in("r9") arr[5],
                        out("rax") result,
                        clobber_abi("C"),
                    );
                },
            }
        }

        result
    }
}

impl PartialEq for DynCFunction {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
