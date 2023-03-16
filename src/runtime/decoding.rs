use crate::mips::instruction;
use crate::mips::instruction::instructions::*;
use crate::runtime::logging;
use crate::runtime::vm;

impl vm::VM {
    pub fn decode_instruction(&self, instruction: u32) -> instruction::InstructionData {
        let base_instruction = self.decode_base_instruction(instruction);

        instruction::InstructionData {
            base: base_instruction,
            args: match base_instruction.format {
                instruction::InstructionFormat::R => {
                    let rs = ((instruction << 6) >> 27) as u8;
                    let rt = ((instruction << 11) >> 27) as u8;
                    let rd = ((instruction << 16) >> 27) as u8;
                    let shamt = ((instruction << 21) >> 27) as u8;
                    let funct = ((instruction << 26) >> 26) as u8;

                    instruction::InstructionArgs::RFormat(instruction::RFormat {
                        rs,
                        rt,
                        rd,
                        shamt,
                        funct,
                    })
                }
                instruction::InstructionFormat::I => {
                    let rs = ((instruction << 6) >> 27) as u8;
                    let rt = ((instruction << 11) >> 27) as u8;
                    let imm = ((instruction << 16) >> 16) as u16;

                    instruction::InstructionArgs::IFormat(instruction::IFormat { rs, rt, imm })
                }
                instruction::InstructionFormat::J => {
                    let address = instruction << 6 >> 6;

                    instruction::InstructionArgs::JFormat(instruction::JFormat { address })
                }
            },
        }
    }

    fn decode_base_instruction(&self, instruction: u32) -> &instruction::Instruction {
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
