use crate::mips::instruction::instructions::*;
use crate::mips::instruction::{
    self, Instruction, InstructionArgs, InstructionData, InstructionFormat,
};
use crate::runtime::errors::{FatalErrorType, RuntimeError};
use crate::runtime::vm;

impl vm::VM {
    pub fn decode_instruction(&self, instruction: u32) -> Result<InstructionData, RuntimeError> {
        let base_instruction = self.decode_base_instruction(instruction)?;

        Ok(InstructionData {
            base: base_instruction,
            args: match base_instruction.format {
                InstructionFormat::R => {
                    let rs = ((instruction << 6) >> 27) as u8;
                    let rt = ((instruction << 11) >> 27) as u8;
                    let rd = ((instruction << 16) >> 27) as u8;
                    let shamt = ((instruction << 21) >> 27) as u8;
                    let funct = ((instruction << 26) >> 26) as u8;

                    InstructionArgs::RFormat(instruction::RFormat {
                        rs,
                        rt,
                        rd,
                        shamt,
                        funct,
                    })
                }
                InstructionFormat::I => {
                    let rs = ((instruction << 6) >> 27) as u8;
                    let rt = ((instruction << 11) >> 27) as u8;
                    let imm = ((instruction << 16) >> 16) as u16;

                    InstructionArgs::IFormat(instruction::IFormat { rs, rt, imm })
                }
                InstructionFormat::J => {
                    let address = instruction << 6 >> 6;

                    InstructionArgs::JFormat(instruction::JFormat { address })
                }
            },
        })
    }

    fn decode_base_instruction(&self, instruction: u32) -> Result<&Instruction, RuntimeError> {
        let b1 = (instruction >> 24) as u8;

        let opcode: u8 = b1 >> 2;

        if opcode == 0 {
            // R-type

            let func_code: u8 = ((instruction << 26) >> 26) as u8;

            for inst in ALL_INSTRUCTIONS.iter() {
                if inst.opc_func == func_code {
                    return Ok(inst);
                }
            }

            return Err(RuntimeError::new(
                FatalErrorType::IllegalInstruction,
                format!(
                    "Unknown R-type instruction: {:#010x} (function code {:#08b})",
                    instruction, func_code
                ),
            ));
        } else {
            // I-type

            for inst in ALL_INSTRUCTIONS.iter() {
                if inst.opc_func == opcode {
                    return Ok(inst);
                }
            }
        }

        Err(RuntimeError::new(
            FatalErrorType::IllegalInstruction,
            format!(
                "Unknown instruction: {:#010x} (opcode {:#08b})",
                instruction, opcode
            ),
        ))
    }
}
