#![allow(dead_code)]

mod mips;
mod runtime;

use crate::runtime::vm;

fn main() {
    let layout = vm::MemoryLayout::mars(0x1000, 0x1000);
    let mut vm = vm::VM::new(layout);

    const TEST_INSTRUCTION: u32 = 0b001000_01001_01010_0000000000101010;

    let text_segment = vm.memory.mut_segment_by_name("text").unwrap();
    text_segment.allow_writes();

    {
        let text_segment = vm.memory.segment_by_name("text").unwrap();

        let text_ptr = vm.memory.align_address(
            text_segment.get_low_address(),
            4,
            runtime::memory::SegmentDirection::Up,
        );

        vm.memory.set_word(text_ptr, TEST_INSTRUCTION);
        vm.set_pc(text_ptr);
    }

    vm.set_register(9, 1);
    vm.run_single_instruction();

    // print registers

    for i in 0..32 {
        println!("Register {}: {}", i, vm.get_register(i));
    }
}
