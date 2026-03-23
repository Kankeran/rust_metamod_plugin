//! 32-bit trampoline

use std::ffi::c_void;

use region::Allocation;

#[allow(dead_code)]
mod bytecode {
    /// Prologue for a function
    pub const CODE_PROLOGUE: [u8; 3] = [
        0x55, // push ebp
        0x89, 0xE5, // mov ebp, esp
    ];

    /// Align stack on 16 byte boundary
    pub const CODE_ALIGN_STACK16: [u8; 3] = [
        0x83, 0xE4, 0xF0, // and esp, 0xFFFFFFF0
    ];

    /// Allocate stack space (8-bit) by adding to ESP
    pub const CODE_ALLOC_STACK: [u8; 3] = [
        0x83, 0xEC, 0xFF, // sub esp, 0xFF
    ];

    /// Offset of [CODE_ALLOC_STACK] to modify at runtime
    /// to contain amount of stack space to allocate.
    pub const CODE_ALLOC_STACK_REPLACE: usize = 2;

    /// Takes a paramter from the trampoline's stack
    /// and pushes it onto the target's stack.
    pub const CODE_PUSH_PARAM: [u8; 3] = [
        0xFF, 0x75, 0xFF, // pushl [ebp+0xFF]
    ];

    /// Offset of [CODE_PUSH_PARAM] to modify at runtime
    /// that contains the stack offset
    pub const CODE_PUSH_PARAM_REPLACE: usize = 2;

    #[cfg(target_os = "windows")]
    pub const CODE_PUSH_THIS: [u8; 1] = [
        0x51, // push ecx
    ];

    #[cfg(not(target_os = "windows"))]
    pub const CODE_PUSH_THIS: [u8; 3] = [
        0xFF, 0x75, 0x04, // pushl [ebp+0x08h]
    ];

    #[cfg(not(target_os = "windows"))]
    pub const CODE_PUSH_THIS_REPLACE: usize = 2;

    /// Pushes a raw number onto the target's stack
    pub const CODE_PUSH_ID: [u8; 5] = [
        0x68, 0xDE, 0xFA, 0xAD, 0xDE, // push	DEADFADEh
    ];

    /// Offset of [CODE_PUSH_ID] to modify at runtime
    /// to contain the number to push
    pub const CODE_PUSH_ID_REPLACE: usize = 1;

    /// Call our procedure
    pub const CODE_CALL: [u8; 7] = [
        0xB8, 0xDE, 0xFA, 0xAD, 0xDE, // mov eax, DEADFADEh
        0xFF, 0xD0, // call eax
    ];

    /// Offset of [CODE_CALL] to modify at runtime
    /// to contain the pointer to the function
    pub const CODE_CALL_REPLACE: usize = 1;

    /// Adds to ESP, freeing up stack space
    pub const CODE_FREE_STACK: [u8; 6] = [
        0x81, 0xC4, 0xFF, 0xFF, 0xFF, 0xFF, // add esp REPLACEME
    ];

    /// Offset of [CODE_FREE_STACK] to modify at runtime
    /// to contain how much data to free
    pub const CODE_FREE_STACK_REPLACE: usize = 2;

    /// Epilogue of a simple function
    pub const CODE_EPILOGUE: [u8; 4] = [
        0x89, 0xEC, // mov esp, ebp
        0x5D, // pop ebp
        0xC3, // ret
    ];

    pub const CODE_EPILOGUE_N: [u8; 6] = [
        0x89, 0xEC, // mov esp, ebp
        0x5D, // pop ebp
        0xC2, 0xCD, 0xAB, // retn 0xABCD
    ];

    pub const CODE_EPILOGUE_NREPLACE: usize = 4;

    pub const CODE_BREAKPOINT: [u8; 1] = [
        0xCC, // int 3
    ];
}

mod ke {
    #[inline]
    pub fn is_power_of_two(value: usize) -> bool {
        if value == 0 {
            false
        } else {
            value * (value - 1) != 0
        }
    }

    pub fn align(count: usize, aligment: usize) -> usize {
        assert!(is_power_of_two(aligment));
        count + (aligment - (count % aligment)) % aligment
    }
}

union Replacement {
    i: usize,
    b: [u8; 4],
}

struct TrampolineMaker {
    buffer: Vec<u8>,
    my_stack: usize,
    called_stack: usize,
    param_start: usize,
    this_call: bool,
}

impl TrampolineMaker {
    fn new() -> Self {
        TrampolineMaker {
            buffer: Vec::with_capacity(512),
            my_stack: 0,
            called_stack: 0,
            param_start: 0,
            this_call: false,
        }
    }

    fn breakpoint(&mut self) {
        self.buffer.extend(bytecode::CODE_BREAKPOINT);
    }

    fn prologue(&mut self) {
        self.buffer.extend(bytecode::CODE_PROLOGUE);
    }

    fn this_prologue(&mut self) {
        self.prologue();
        self.this_call = true;
    }

    fn epilogue(&mut self) {
        self.buffer.extend(bytecode::CODE_EPILOGUE);
    }

    fn epilogue_and_free_all(&mut self) {
        self.epilogue_and_free(self.my_stack);
    }

    fn epilogue_and_free(&mut self, how_much: usize) {
        let mut code = bytecode::CODE_EPILOGUE_N;
        let bi = Replacement { i: how_much };
        code[bytecode::CODE_EPILOGUE_NREPLACE] = unsafe { bi.b[0] };
        code[bytecode::CODE_EPILOGUE_NREPLACE + 1] = unsafe { bi.b[1] };
        self.buffer.extend(code);
    }

    fn align_stack_16(&mut self, slots: usize) {
        let stack_needed = slots * size_of::<*const c_void>();
        let stack_reserve = ke::align(stack_needed, 16);
        let stack_extra = stack_reserve - stack_needed;

        assert!(stack_extra <= 0xff);

        let mut code = Vec::from(bytecode::CODE_ALIGN_STACK16);
        if stack_extra > 0 {
            code.extend(bytecode::CODE_ALLOC_STACK);
            let bi = Replacement { i: stack_extra };
            code[bytecode::CODE_ALIGN_STACK16.len() + bytecode::CODE_ALLOC_STACK_REPLACE] =
                unsafe { bi.b[0] };
            self.buffer.extend(code);
        } else {
            self.buffer.extend(code);
        }
    }

    fn push_this(&mut self) {
        if !self.this_call {
            return;
        }

        #[allow(unused_mut)]
        let mut code = bytecode::CODE_PUSH_THIS;

        #[cfg(not(target_os = "windows"))]
        {
            let bi = Replacement {
                i: self.param_start + 8,
            };
            code[bytecode::CODE_PUSH_THIS_REPLACE] = unsafe { bi.b[0] };
            self.my_stack += 4;
        }

        self.buffer.extend(code);
        self.called_stack += 4;
    }

    fn free_my_stack(&mut self) {
        self.free_stack(self.my_stack);
    }

    fn free_target_stack(&mut self) {
        self.free_stack(self.called_stack);
    }

    fn free_both_stacks(&mut self) {
        self.free_stack(self.called_stack + self.my_stack);
    }

    fn free_stack(&mut self, how_much: usize) {
        let mut code = bytecode::CODE_FREE_STACK;
        let bi = Replacement { i: how_much };
        code[bytecode::CODE_FREE_STACK_REPLACE] = unsafe { bi.b[0] };
        code[bytecode::CODE_FREE_STACK_REPLACE + 1] = unsafe { bi.b[1] };
        code[bytecode::CODE_FREE_STACK_REPLACE + 2] = unsafe { bi.b[2] };
        code[bytecode::CODE_FREE_STACK_REPLACE + 3] = unsafe { bi.b[3] };
        self.buffer.extend(code);
    }

    fn push_num(&mut self, number: usize) {
        let mut code = bytecode::CODE_PUSH_ID;
        let bi = Replacement { i: number };
        code[bytecode::CODE_PUSH_ID_REPLACE] = unsafe { bi.b[0] };
        code[bytecode::CODE_PUSH_ID_REPLACE + 1] = unsafe { bi.b[1] };
        code[bytecode::CODE_PUSH_ID_REPLACE + 2] = unsafe { bi.b[2] };
        code[bytecode::CODE_PUSH_ID_REPLACE + 3] = unsafe { bi.b[3] };
        self.buffer.extend(code);
        self.called_stack += 4;
    }

    fn push_param(&mut self, mut which: usize) {
        #[cfg(not(target_os = "windows"))]
        if self.this_call {
            which += 1;
        }
        which = which * 4;
        which += self.param_start + 4;
        let value = which as u8;
        let mut code = bytecode::CODE_PUSH_PARAM;
        code[bytecode::CODE_PUSH_PARAM_REPLACE] = value;
        self.buffer.extend(code);
        self.called_stack += 4;
        self.my_stack += 4;
    }

    fn call(&mut self, ptr: *const c_void) {
        let mut code = bytecode::CODE_CALL;
        let bi = Replacement { i: ptr.addr() };
        code[bytecode::CODE_CALL_REPLACE] = unsafe { bi.b[0] };
        code[bytecode::CODE_CALL_REPLACE + 1] = unsafe { bi.b[1] };
        code[bytecode::CODE_CALL_REPLACE + 2] = unsafe { bi.b[2] };
        code[bytecode::CODE_CALL_REPLACE + 3] = unsafe { bi.b[3] };
        self.buffer.extend(code);
    }

    fn finish(self) -> Allocation {
        use region::Protection;

        let mut ret = region::alloc(self.buffer.len(), Protection::READ_WRITE_EXECUTE).unwrap();
        let slice = unsafe {
            std::slice::from_raw_parts_mut(ret.as_mut_ptr::<u8>(), ret.len())
        };
        slice[..self.buffer.len()].copy_from_slice(&self.buffer);
        ret
    }
}

pub fn create_generic_trampoline(
    this_call: bool,
    _void_call: bool,
    _ret_buf: bool,
    param_count: usize,
    extra_ptr: *const c_void,
    func_ptr: *const c_void,
) -> Allocation {
    let mut tramp = TrampolineMaker::new();
    if this_call {
        tramp.this_prologue();
        tramp.align_stack_16(param_count + 2);
    } else {
        tramp.prologue();
        tramp.align_stack_16(param_count + 1);
    }

    for i in 0..param_count {
        tramp.push_param(param_count - i);
    }

    if this_call {
        tramp.push_this();
    }
    tramp.push_num(extra_ptr.addr());
    tramp.call(func_ptr);
    tramp.free_target_stack();

    #[cfg(target_os = "windows")]
    {
        tramp.epilogue_and_free_all();
    }
    #[cfg(not(target_os = "windows"))]
    {
        if _ret_buf {
            tramp.epilogue_and_free(4);
        } else {
            tramp.epilogue();
        }
    }

    tramp.finish()
}
