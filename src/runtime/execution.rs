use crate::mips::instruction::{coerce_i_format, coerece_r_format, InstructionData};
use crate::runtime::errors::{FatalErrorType, RuntimeError, Trap};
use crate::runtime::vm::VM;

#[derive(Debug, Copy, Clone)]
enum ShiftDirection {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum ShiftType {
    Logical,
    Arithmetic,
}

#[derive(Debug, Copy, Clone)]
enum ExecutionTask {
    /* Arithmetic and logical operations */
    Add {
        dest: Target,
        a: Target,
        b: Target,
        overflow_trap: bool,
    },
    Sub {
        dest: Target,
        a: Target,
        b: Target,
        overflow_trap: bool,
    },
    And {
        dest: Target,
        a: Target,
        b: Target,
    },
    Or {
        dest: Target,
        a: Target,
        b: Target,
    },
    Xor {
        dest: Target,
        a: Target,
        b: Target,
    },
    Nor {
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
        signed: bool,
    },
    Shift {
        dest: Target,
        a: Target,
        b: Target,
        direction: ShiftDirection,
        shift_type: ShiftType,
    },

    /* Memory operations */
    Load {
        dest: Target,
        src_addr: Target,
        offset: Target,
        signed: bool,
        size: u8,
    },

    Store {
        dest_addr: Target,
        src: Target,
        offset: Target,
        size: u8,
    },

    /* Control flow operations */
    Jump {
        dest: Target,
    },

    /* Other */
    Syscall,

    Nop, // No operation; used for conditional instructions that don't execute.
}

/// Represents a target for an instruction.
///
/// This can be a register, memory, or immediate value.
#[derive(Debug, Copy, Clone)]
enum Target {
    Register(u8),
    Memory(u32),
    Immediate(u16, HalfWordExtension),
}

/// Represents whether an immediate value should be sign-extended or zero-extended
/// when used as a 32-bit value.
#[derive(Debug, Copy, Clone)]
enum HalfWordExtension {
    Sign,
    Zero,
}

impl VM {
    /// Run a single instruction, and return the instruction that was executed, and any trap
    /// that was triggered, if any.
    pub fn run_single_instruction(
        &mut self,
    ) -> Result<(InstructionData, Option<Trap>), RuntimeError> {
        let instruction = self.fetch_instruction_code()?;
        self.execute_instruction(instruction)
    }

    /// Fetch the next instruction from memory, and increment the program counter.
    /// 
    /// Return the four byte instruction code.
    pub fn fetch_instruction_code(&mut self) -> Result<u32, RuntimeError> {
        let pc = self.get_pc();
        let instruction = self.memory.get_word(pc as usize)?;
        self.set_pc(pc + 4);

        Ok(instruction)
    }

    /// Execute the instruction given by the four byte instruction code.
    /// 
    /// Return the decoded instruction that was executed, and any trap that was triggered,
    /// if any.
    pub fn execute_instruction(
        &mut self,
        instruction: u32,
    ) -> Result<(InstructionData, Option<Trap>), RuntimeError> {
        let inst = self.decode_instruction(instruction)?;
        let task = self.get_task(&inst)?;
        let mut trap: Option<Trap> = None;

        if !inst.is_null() {
            if let Ok(t) = self.execute_task(task) {
                trap = t;
            }
        }

        Ok((self.decode_instruction(instruction)?, trap)) // re-decode instruction because borrow checker or whatever
    }

    /// Gets the execution task for the given instruction.
    fn get_task(&self, instruction: &InstructionData) -> Result<ExecutionTask, RuntimeError> {
        if let Some(task) = self.get_add_task(instruction) {
            return Ok(task);
        }

        if let Some(task) = self.get_sub_task(instruction) {
            return Ok(task);
        }

        if let Some(task) = self.get_mult_task(instruction) {
            return Ok(task);
        }

        if let Some(task) = self.get_boolean_task(instruction) {
            return Ok(task);
        }

        if let Some(task) = self.get_shift_task(instruction) {
            return Ok(task);
        }

        Err(RuntimeError::new(
            FatalErrorType::IllegalInstruction,
            format!("Unsupported instruction \"{}\"", instruction.base.name),
        ))
    }

    /// Gets the execution task for an add-type instruction. Returns None if the instruction is not
    /// an add-type instruction.
    fn get_add_task(&self, instruction: &InstructionData) -> Option<ExecutionTask> {
        match instruction.base.name {
            "add" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    overflow_trap: true,
                })
            }
            "addu" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    overflow_trap: false,
                })
            }
            "addi" => {
                let args = coerce_i_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm, HalfWordExtension::Sign),
                    overflow_trap: true,
                })
            }
            "addiu" => {
                let args = coerce_i_format(instruction);

                Some(ExecutionTask::Add {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm, HalfWordExtension::Sign),
                    overflow_trap: false,
                })
            }
            _ => None,
        }
    }

    fn get_sub_task(&self, instruction: &InstructionData) -> Option<ExecutionTask> {
        match instruction.base.name {
            "sub" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Sub {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    overflow_trap: true,
                })
            }
            "subu" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Sub {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    overflow_trap: false,
                })
            }

            _ => None,
        }
    }

    fn get_mult_task(&self, instruction: &InstructionData) -> Option<ExecutionTask> {
        match instruction.base.name {
            "mult" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Mult {
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    signed: true,
                })
            }
            "multu" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Mult {
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                    signed: false,
                })
            }
            _ => None,
        }
    }

    fn get_boolean_task(&self, instruction: &InstructionData) -> Option<ExecutionTask> {
        match instruction.base.name {
            "and" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::And {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                })
            }
            "andi" => {
                let args = coerce_i_format(instruction);

                Some(ExecutionTask::And {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm, HalfWordExtension::Zero),
                })
            }
            "or" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Or {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                })
            }
            "ori" => {
                let args = coerce_i_format(instruction);

                Some(ExecutionTask::Or {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm, HalfWordExtension::Zero),
                })
            }
            "nor" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Nor {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                })
            }
            "xor" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Xor {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rs),
                    b: Target::Register(args.rt),
                })
            }
            "xori" => {
                let args = coerce_i_format(instruction);

                Some(ExecutionTask::Xor {
                    dest: Target::Register(args.rt),
                    a: Target::Register(args.rs),
                    b: Target::Immediate(args.imm, HalfWordExtension::Zero),
                })
            }
            _ => None,
        }
    }

    fn get_shift_task(&self, instruction: &InstructionData) -> Option<ExecutionTask> {
        match instruction.base.name {
            "sll" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Shift {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rt),
                    b: Target::Immediate(args.shamt as u16, HalfWordExtension::Zero),
                    direction: ShiftDirection::Left,
                    shift_type: ShiftType::Logical,
                })
            }
            "sllv" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Shift {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rt),
                    b: Target::Register(args.rs),
                    direction: ShiftDirection::Left,
                    shift_type: ShiftType::Logical,
                })
            }
            "sra" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Shift {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rt),
                    b: Target::Immediate(args.shamt as u16, HalfWordExtension::Zero),
                    direction: ShiftDirection::Right,
                    shift_type: ShiftType::Arithmetic,
                })
            }
            "srav" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Shift {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rt),
                    b: Target::Register(args.rs),
                    direction: ShiftDirection::Right,
                    shift_type: ShiftType::Arithmetic,
                })
            }
            "srl" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Shift {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rt),
                    b: Target::Immediate(args.shamt as u16, HalfWordExtension::Zero),
                    direction: ShiftDirection::Right,
                    shift_type: ShiftType::Logical,
                })
            }
            "srlv" => {
                let args = coerece_r_format(instruction);

                Some(ExecutionTask::Shift {
                    dest: Target::Register(args.rd),
                    a: Target::Register(args.rt),
                    b: Target::Register(args.rs),
                    direction: ShiftDirection::Right,
                    shift_type: ShiftType::Logical,
                })
            }

            _ => None,
        }
    }

    /// Gets the value of the given memory target.
    fn get_value_of_target(&self, target: &Target) -> Result<u32, RuntimeError> {
        match target {
            Target::Register(reg) => self.get_register(*reg),
            Target::Memory(address) => self.memory.get_word(*address as usize),
            Target::Immediate(value, hw_ext) => match hw_ext {
                HalfWordExtension::Sign => Ok((*value as i16) as u32),
                HalfWordExtension::Zero => Ok(*value as u32),
            },
        }
    }

    /// Sets the value of the given memory target.
    ///
    /// Panics if the target is an immediate value.
    fn set_value_of_target(&mut self, target: Target, value: u32) -> Result<(), RuntimeError> {
        match target {
            Target::Register(reg) => self.set_register(reg, value),
            Target::Memory(address) => self.memory.set_word(address as usize, value),
            Target::Immediate(_, _) => panic!("Cannot set value of immediate target"),
        }
    }

    /// Executes the given execution task.
    #[allow(unreachable_patterns)]
    fn execute_task(&mut self, task: ExecutionTask) -> Result<Option<Trap>, RuntimeError> {
        match task {
            ExecutionTask::Add {
                dest,
                a,
                b,
                overflow_trap: overflow,
            } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                let (result, overflowed) = a.overflowing_add(b);

                if overflow && overflowed {
                    return Ok(Some(Trap::new(format!(
                        "Overflowed when adding {} and {}",
                        a, b
                    ))));
                }

                self.set_value_of_target(dest, result)?;
            }
            ExecutionTask::Sub {
                dest,
                a,
                b,
                overflow_trap: overflow,
            } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                let (result, overflowed) = a.overflowing_sub(b);

                if overflow && overflowed {
                    return Ok(Some(Trap::new(format!(
                        "Overflowed when subtracting {} from {}",
                        a, b
                    ))));
                }

                self.set_value_of_target(dest, result)?;
            }
            ExecutionTask::Mult { a, b, signed } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                let result = if signed {
                    (a as i32 as u64).overflowing_mul(b as i32 as u64).0
                } else {
                    (a as u64).overflowing_mul(b as u64).0 // a and b are both u32 already
                };

                self.set_hi((result >> 32) as u32);
                self.set_lo(result as u32);
            }
            ExecutionTask::And { dest, a, b } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                self.set_value_of_target(dest, a & b)?;
            }
            ExecutionTask::Or { dest, a, b } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                self.set_value_of_target(dest, a | b)?;
            }
            ExecutionTask::Xor { dest, a, b } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                self.set_value_of_target(dest, a ^ b)?;
            }
            ExecutionTask::Shift {
                dest,
                a,
                b,
                direction,
                shift_type,
            } => {
                let a = self.get_value_of_target(&a)?;
                let b = self.get_value_of_target(&b)?;

                let result = match (direction, shift_type) {
                    (ShiftDirection::Left, ShiftType::Logical) => a << b,
                    (ShiftDirection::Left, ShiftType::Arithmetic) => ((a as i32) << b) as u32,
                    (ShiftDirection::Right, ShiftType::Logical) => a >> b,
                    (ShiftDirection::Right, ShiftType::Arithmetic) => ((a as i32) >> b) as u32,
                };

                self.set_value_of_target(dest, result)?;
            }
            _ => panic!("Unsupported execution task {:?}", task),
        }

        Ok(None)
    }
}
