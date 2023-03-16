use crate::mips::instruction::{IFormat, InstructionArgs, InstructionData, JFormat, RFormat};
use crate::runtime::logging;
use crate::runtime::vm::VM;

#[derive(Debug)]
enum ExecutionTask {
    Add {
        dest: Target,
        a: Target,
        b: Target,
        overflow: bool,
    },
    And {
        dest: Target,
        a: Target,
        b: Target,
    },
    Div {
        a: Target,
        b: Target,
        overflow: bool,
    },
    Mult {
        a: Target,
        b: Target,
        overflow: bool,
    },
}

/// Represents a target for an instruction.
///
/// This can be a register, memory, or immediate value.
#[derive(Debug)]
enum Target {
    Register(u8),
    Memory(u32),
    Immediate(u32),
}

impl VM {
    pub fn run_single_instruction(&mut self) {
        let instruction = self.fetch_instruction();
        self.execute_instruction(instruction);
    }

    /// Fetches the next instruction from memory, and increments the program counter.
    pub fn fetch_instruction(&mut self) -> u32 {
        let pc = self.get_pc();
        let instruction = self.memory.get_word(pc as usize);
        self.set_pc(pc + 4);

        instruction
    }

    /// Execute the instruction given by the four byte instruction code.
    pub fn execute_instruction(&mut self, instruction: u32) {
        let inst = self.decode_instruction(instruction);
        let task = self.get_task(&inst);
        
        self.execute_task(task);
    }

    /// Coerces R-format arguments from an instruction. Panics if the instruction is not R-format.
    fn coerece_r_format<'a>(&self, instruction: &'a InstructionData) -> &'a RFormat {
        match &instruction.args {
            InstructionArgs::RFormat(args) => args,
            _ => panic!("Given instruction is not R-format"),
        }
    }

    /// Coerces I-format arguments from an instruction. Panics if the instruction is not I-format.
    fn coerce_i_format<'a>(&self, instruction: &'a InstructionData) -> &'a IFormat {
        match &instruction.args {
            InstructionArgs::IFormat(args) => args,
            _ => panic!("Given instruction is not I-format"),
        }
    }

    /// Coerces J-format arguments from an instruction. Panics if the instruction is not J-format.
    fn coerce_j_format<'a>(&self, instruction: &'a InstructionData) -> &'a JFormat {
        match &instruction.args {
            InstructionArgs::JFormat(args) => args,
            _ => panic!("Given instruction is not J-format"),
        }
    }

    /// Gets the execution task for the given instruction.
    fn get_task(&self, instruction: &InstructionData) -> ExecutionTask {
        if let Some(task) = self.get_add_task(instruction) {
            return task;
        }

        logging::fatal_error(
            logging::FatalErrorType::IllegalInstruction,
            format!("Unsupported instruction \"{}\"", instruction.base.name),
        );
    }

    /// Gets the execution task for an add-type instruction. Returns None if the instruction is not
    /// an add-type instruction.
    fn get_add_task(&self, instruction: &InstructionData) -> Option<ExecutionTask> {
        match instruction.base.name {
            "add" => {
                let args = self.coerece_r_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    overflow: true,
                })
            }
            "addu" => {
                let args = self.coerece_r_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    overflow: false,
                })
            }
            "addi" => {
                let args = self.coerce_i_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm as u32),
                    overflow: true,
                })
            }
            "addiu" => {
                let args = self.coerce_i_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm as u32),
                    overflow: false,
                })
            }
            _ => None,
        }
    }

    /// Gets the value of the given memory target.
    fn get_value_of_target(&self, target: &Target) -> u32 {
        match target {
            Target::Register(reg) => self.get_register(*reg),
            Target::Memory(address) => self.memory.get_word(*address as usize),
            Target::Immediate(value) => *value,
        }
    }

    /// Sets the value of the given memory target.
    /// 
    /// Panics if the target is an immediate value.
    fn set_value_of_target(&mut self, target: Target, value: u32) {
        match target {
            Target::Register(reg) => self.set_register(reg, value),
            Target::Memory(address) => self.memory.set_word(address as usize, value),
            Target::Immediate(_) => panic!("Cannot set value of immediate target"),
        }
    }

    /// Executes the given execution task.
    #[allow(unreachable_patterns)]
    fn execute_task(&mut self, task: ExecutionTask) {
        match task {
            ExecutionTask::Add {
                dest,
                a,
                b,
                overflow,
            } => {
                let a = self.get_value_of_target(&a);
                let b = self.get_value_of_target(&b);

                let (result, overflowed) = a.overflowing_add(b);

                if overflow && overflowed {
                    panic!("Overflowed when adding {} and {}", a, b);
                }

                self.set_value_of_target(dest, result);
            }
            _ => panic!("Unsupported execution task {:?}", task),
        }
    }
}
