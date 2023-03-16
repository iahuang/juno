use colored::*;

pub enum FatalErrorType {
    IllegalMemoryAccess,
    IllegalInstruction,
    IllegalRegister,
}

/// Log a fatal error and exit the program.
pub fn fatal_error(err_type: FatalErrorType, message: String) -> ! {
    eprintln!(
        "{} {}: {}",
        "[runtime error]".red().bold(),
        match err_type {
            FatalErrorType::IllegalMemoryAccess => "ILLEGAL_MEMORY_ACCESS",
            FatalErrorType::IllegalInstruction => "ILLEGAL_INSTRUCTION",
            FatalErrorType::IllegalRegister => "ILLEGAL_REGISTER",
        },
        message
    );

    std::process::exit(1);
}
