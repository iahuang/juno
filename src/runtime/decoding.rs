use crate::mips::instruction::Instruction;
use crate::mips::instruction::Instructions::*;
use crate::runtime::logging;
use crate::runtime::vm;

impl vm::VM {
    pub fn decode_instruction(&self, instruction: u32) -> &Instruction {
        let b1 = (instruction >> 24) as u8;

        let opcode: u8 = b1 >> 2;

        if opcode == 0 {
            // R-type

            let func_code: u8 = ((instruction << 26) >> 26) as u8;

            for inst in ALL_INSTRUCTIONS.iter() {
                if inst.opc_func == func_code {
                    return inst;
                }
            }

            logging::fatal_error(
                logging::FatalErrorType::IllegalInstruction,
                format!(
                    "Unknown R-type instruction: {:#010x} (function code {:#08b})",
                    instruction, func_code
                ),
            );
        } else {
            // I-type

            for inst in ALL_INSTRUCTIONS.iter() {
                if inst.opc_func == opcode {
                    return inst;
                }
            }
        }

        logging::fatal_error(
            logging::FatalErrorType::IllegalInstruction,
            format!(
                "Unknown instruction: {:#010x} (opcode {:#08b})",
                instruction, opcode
            ),
        );
    }
}
