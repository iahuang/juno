#![allow(dead_code)]

mod runtime;

use crate::runtime::vm;

fn main() {
    let layout = vm::MemoryLayout::mars(0x1000, 0x1000);
    let mut vm = vm::VM::new(layout);

    let stack = vm.memory.segment_by_name("stack").unwrap();

    vm.memory.set_word(stack.get_high_address() + 4, 329488293);

    // print some memory
    for addr in stack.get_high_address() - 4..stack.get_high_address() {
        println!("{:#010x}: {:#010x}", addr, vm.memory.get_word(addr));
    }
}
