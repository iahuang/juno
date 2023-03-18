use colored::*;
use crate::runtime::errors::{RuntimeError, FatalErrorType, Trap};

pub struct Logger {
    
}

impl Logger {
    /// Log a fatal error.
    pub fn fatal_error(&self, err: &RuntimeError) {
        eprintln!(
            "{} {}: {}",
            "[runtime error]".red().bold(),
            match err.err_type {
                FatalErrorType::IllegalMemoryAccess => "ILLEGAL_MEMORY_ACCESS",
                FatalErrorType::IllegalInstruction => "ILLEGAL_INSTRUCTION",
                FatalErrorType::IllegalRegisterAccess => "ILLEGAL_REGISTER",
            },
            err.message
        );
    }

    /// Log a trap error.
    pub fn trap_error(&self, trap: &Trap) {
        eprintln!("{} {}", "[trap]".red().bold(), trap.message);
    }
}
