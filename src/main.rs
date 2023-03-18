#![allow(dead_code)]

mod assembler;
mod mips;
mod runtime;
mod term_ui;

use term_ui::console::Console;
use term_ui::VMState;

use crate::runtime::vm;

fn main() {
    let layout = vm::MemoryLayout::mars(0x1000, 0x1000);
    let mut vm = vm::VM::new(layout);

    let text_segment = vm.memory.mut_segment_by_name("text").unwrap();
    text_segment.allow_writes();

    {
        let text_segment = vm.memory.segment_by_name("text").unwrap();

        let text_ptr = vm.memory.align_address(
            text_segment.get_low_address(),
            4,
            runtime::memory::SegmentDirection::Up,
        );

        vm.memory.set_word(text_ptr, 0x2409ffff);
        vm.memory.set_word(text_ptr + 4, 0x240a0002);
        vm.memory.set_word(text_ptr + 8, 0x012a0018);
        vm.set_pc(text_ptr);
    }
    let mut console = Console::new();
    let mut paused = true;
    let mut halted = false;

    let mut ui = term_ui::make_crossterm_viewer();
    ui.init().unwrap();

    loop {
        if !paused && !halted {
            let instruction = vm.run_single_instruction();

            match instruction {
                Ok((instruction_data, trap)) => {
                    if instruction_data.is_null() {
                        halted = true;
                        console.execution_finished("Null instruction reached");
                    }

                    if let Some(trap) = trap {
                        halted = true;
                        console.trap_error(&trap.message);
                    }
                }
                Err(err) => {
                    halted = true;

                    console.runtime_error(&err);
                }
            }
        }

        match ui.update(
            &(VMState {
                vm: &vm,
                paused,
                halted,
                console: &console,
            }),
        ) {
            Ok(term_ui::VMViewerEvent::Quit) => break,
            Ok(term_ui::VMViewerEvent::None) => {}
            Ok(term_ui::VMViewerEvent::TogglePause) => {
                if !halted {
                    paused = !paused;
                }
            }
            Err(e) => panic!("Failed to update UI: {}", e),
        }
    }
    ui.exit().expect("Failed to exit UI");
}
