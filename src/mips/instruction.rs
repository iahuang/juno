#[derive(Debug, Clone, Copy)]
pub struct RFormat {
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct IFormat {
    pub rs: u8,
    pub rt: u8,
    pub imm: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct JFormat {
    pub address: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionArgs {
    RFormat(RFormat),
    IFormat(IFormat),
    JFormat(JFormat),
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionFormat {
    R,
    I,
    J,
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionData<'a> {
    pub base: &'a Instruction<'a>,
    pub args: InstructionArgs,
}

/// Coerces R-format arguments from an instruction. Panics if the instruction is not R-format.
pub fn coerece_r_format<'a>(instruction: &'a InstructionData) -> &'a RFormat {
    match &instruction.args {
        InstructionArgs::RFormat(args) => args,
        _ => panic!("Given instruction is not R-format"),
    }
}

/// Coerces I-format arguments from an instruction. Panics if the instruction is not I-format.
pub fn coerce_i_format<'a>(instruction: &'a InstructionData) -> &'a IFormat {
    match &instruction.args {
        InstructionArgs::IFormat(args) => args,
        _ => panic!("Given instruction is not I-format"),
    }
}

/// Coerces J-format arguments from an instruction. Panics if the instruction is not J-format.
pub fn coerce_j_format<'a>(instruction: &'a InstructionData) -> &'a JFormat {
    match &instruction.args {
        InstructionArgs::JFormat(args) => args,
        _ => panic!("Given instruction is not J-format"),
    }
}

impl<'a> InstructionData<'a> {
    /// Returns true if the instruction is a sll instruction with all zero arguments.
    pub fn is_null(&self) -> bool {
        match self.args {
            InstructionArgs::RFormat(r_args) => {
                if self.base.opc_func == 0 {
                    r_args.rs == 0 && r_args.rt == 0 && r_args.rd == 0 && r_args.shamt == 0
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Instruction<'a> {
    /// The opcode or function code of the instruction. If the instruction is
    /// in the R format, this is the function code. Otherwise, it is the opcode.
    pub opc_func: u8,
    pub name: &'a str,
    pub format: InstructionFormat,
}

/// Supported MIPS instructions.
pub mod instructions {
    use super::*;

    pub const ADD: Instruction = Instruction {
        opc_func: 0b100000,
        name: "add",
        format: InstructionFormat::R,
    };

    pub const ADDU: Instruction = Instruction {
        opc_func: 0b100001,
        name: "addu",
        format: InstructionFormat::R,
    };

    pub const ADDI: Instruction = Instruction {
        opc_func: 0b001000,
        name: "addi",
        format: InstructionFormat::I,
    };

    pub const ADDIU: Instruction = Instruction {
        opc_func: 0b001001,
        name: "addiu",
        format: InstructionFormat::I,
    };

    pub const AND: Instruction = Instruction {
        opc_func: 0b100100,
        name: "and",
        format: InstructionFormat::R,
    };

    pub const ANDI: Instruction = Instruction {
        opc_func: 0b001100,
        name: "andi",
        format: InstructionFormat::I,
    };

    pub const DIV: Instruction = Instruction {
        opc_func: 0b011010,
        name: "div",
        format: InstructionFormat::R,
    };

    pub const DIVU: Instruction = Instruction {
        opc_func: 0b011011,
        name: "divu",
        format: InstructionFormat::R,
    };

    pub const MULT: Instruction = Instruction {
        opc_func: 0b011000,
        name: "mult",
        format: InstructionFormat::R,
    };

    pub const MULTU: Instruction = Instruction {
        opc_func: 0b011001,
        name: "multu",
        format: InstructionFormat::R,
    };

    pub const NOR: Instruction = Instruction {
        opc_func: 0b100111,
        name: "nor",
        format: InstructionFormat::R,
    };

    pub const OR: Instruction = Instruction {
        opc_func: 0b100101,
        name: "or",
        format: InstructionFormat::R,
    };

    pub const ORI: Instruction = Instruction {
        opc_func: 0b001101,
        name: "ori",
        format: InstructionFormat::I,
    };

    pub const SLL: Instruction = Instruction {
        opc_func: 0b000000,
        name: "sll",
        format: InstructionFormat::R,
    };

    pub const SLLV: Instruction = Instruction {
        opc_func: 0b000100,
        name: "sllv",
        format: InstructionFormat::R,
    };

    pub const SRA: Instruction = Instruction {
        opc_func: 0b000011,
        name: "sra",
        format: InstructionFormat::R,
    };

    pub const SRAV: Instruction = Instruction {
        opc_func: 0b000111,
        name: "srav",
        format: InstructionFormat::R,
    };

    pub const SRL: Instruction = Instruction {
        opc_func: 0b000010,
        name: "srl",
        format: InstructionFormat::R,
    };

    pub const SRLV: Instruction = Instruction {
        opc_func: 0b000110,
        name: "srlv",
        format: InstructionFormat::R,
    };

    pub const SUB: Instruction = Instruction {
        opc_func: 0b100010,
        name: "sub",
        format: InstructionFormat::R,
    };

    pub const SUBU: Instruction = Instruction {
        opc_func: 0b100011,
        name: "subu",
        format: InstructionFormat::R,
    };

    pub const XOR: Instruction = Instruction {
        opc_func: 0b100110,
        name: "xor",
        format: InstructionFormat::R,
    };

    pub const XORI: Instruction = Instruction {
        opc_func: 0b001110,
        name: "xori",
        format: InstructionFormat::I,
    };

    pub const SLT: Instruction = Instruction {
        opc_func: 0b101010,
        name: "slt",
        format: InstructionFormat::R,
    };

    pub const SLTU: Instruction = Instruction {
        opc_func: 0b101001,
        name: "sltu",
        format: InstructionFormat::R,
    };

    pub const SLTI: Instruction = Instruction {
        opc_func: 0b001010,
        name: "slti",
        format: InstructionFormat::I,
    };

    pub const SLTIU: Instruction = Instruction {
        opc_func: 0b001001,
        name: "sltiu",
        format: InstructionFormat::I,
    };

    pub const BEQ: Instruction = Instruction {
        opc_func: 0b000100,
        name: "beq",
        format: InstructionFormat::I,
    };

    pub const BGTZ: Instruction = Instruction {
        opc_func: 0b000111,
        name: "bgz",
        format: InstructionFormat::I,
    };

    pub const BLEZ: Instruction = Instruction {
        opc_func: 0b000110,
        name: "blez",
        format: InstructionFormat::I,
    };

    pub const BNE: Instruction = Instruction {
        opc_func: 0b000101,
        name: "bne",
        format: InstructionFormat::I,
    };

    pub const J: Instruction = Instruction {
        opc_func: 0b000010,
        name: "j",
        format: InstructionFormat::J,
    };

    pub const JAL: Instruction = Instruction {
        opc_func: 0b000011,
        name: "jal",
        format: InstructionFormat::J,
    };

    pub const JALR: Instruction = Instruction {
        opc_func: 0b001001,
        name: "jalr",
        format: InstructionFormat::R,
    };

    pub const JR: Instruction = Instruction {
        opc_func: 0b001000,
        name: "jr",
        format: InstructionFormat::R,
    };

    pub const LB: Instruction = Instruction {
        opc_func: 0b100000,
        name: "lb",
        format: InstructionFormat::I,
    };

    pub const LBU: Instruction = Instruction {
        opc_func: 0b100100,
        name: "lbu",
        format: InstructionFormat::I,
    };

    pub const LH: Instruction = Instruction {
        opc_func: 0b100001,
        name: "lh",
        format: InstructionFormat::I,
    };

    pub const LHU: Instruction = Instruction {
        opc_func: 0b100101,
        name: "lhu",
        format: InstructionFormat::I,
    };

    pub const LW: Instruction = Instruction {
        opc_func: 0b100011,
        name: "lw",
        format: InstructionFormat::I,
    };

    pub const SB: Instruction = Instruction {
        opc_func: 0b101000,
        name: "sb",
        format: InstructionFormat::I,
    };

    pub const SH: Instruction = Instruction {
        opc_func: 0b101001,
        name: "sh",
        format: InstructionFormat::I,
    };

    pub const SW: Instruction = Instruction {
        opc_func: 0b101011,
        name: "sw",
        format: InstructionFormat::I,
    };

    pub const MFHI: Instruction = Instruction {
        opc_func: 0b010000,
        name: "mfhi",
        format: InstructionFormat::R,
    };

    pub const MFLO: Instruction = Instruction {
        opc_func: 0b010010,
        name: "mflo",
        format: InstructionFormat::R,
    };

    pub const MTHI: Instruction = Instruction {
        opc_func: 0b010001,
        name: "mthi",
        format: InstructionFormat::R,
    };

    pub const MTLO: Instruction = Instruction {
        opc_func: 0b010011,
        name: "mtlo",
        format: InstructionFormat::R,
    };

    pub const SYSCALL: Instruction = Instruction {
        opc_func: 0b001100,
        name: "syscall",
        format: InstructionFormat::R,
    };

    pub const ALL_INSTRUCTIONS: [Instruction; 48] = [
        ADD, ADDU, ADDI, ADDIU, AND, ANDI, DIV, DIVU, MULT, MULTU, NOR, OR, ORI, SLL, SLLV, SRA,
        SRAV, SRL, SRLV, SUB, SUBU, XOR, XORI, SLT, SLTU, SLTI, SLTIU, BEQ, BGTZ, BLEZ, BNE, J,
        JAL, JALR, JR, LB, LBU, LH, LHU, LW, SB, SH, SW, MFHI, MFLO, MTHI, MTLO, SYSCALL,
    ];
}
