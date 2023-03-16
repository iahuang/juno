#![allow(dead_code)]

mod runtime;
mod mips;
use crate::runtime::vm;

fn main() {
    let layout = vm::MemoryLayout::mars(0x1000, 0x1000);
    let mut vm = vm::VM::new(layout);

    {
        let stack = vm.memory.segment_by_name("stack").unwrap();
        vm.memory.set_word(stack.get_high_address() - 3, 0xffeeddcc);
    }

    let stack = vm.memory.segment_by_name("stack").unwrap();

    // print some memory
    for addr in stack.get_high_address() - 3..stack.get_high_address()+1 {
        println!("{:#010x}: {}", addr, vm.memory.get_byte(addr));
    }

    println!("{:?}", vm.decode_instruction(0x8fa20000));
}
