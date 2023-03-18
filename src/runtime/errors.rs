#[derive(Debug)]
pub enum FatalErrorType {
    IllegalMemoryAccess,
    IllegalInstruction,
    IllegalRegisterAccess,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub err_type: FatalErrorType,
    pub message: String,
}

impl RuntimeError {
    pub fn new(err_type: FatalErrorType, message: String) -> RuntimeError {
        RuntimeError { err_type, message }
    }

    pub fn err_invalid_read(address: usize) -> RuntimeError {
        RuntimeError::new(
            FatalErrorType::IllegalMemoryAccess,
            format!("Invalid read at {:#010x}", address),
        )
    }

    pub fn err_invalid_write(address: usize) -> RuntimeError {
        RuntimeError::new(
            FatalErrorType::IllegalMemoryAccess,
            format!("Invalid write at {:#010x}", address),
        )
    }
}

/// A trap is a non-fatal error that can be handled by the program.
pub struct Trap {
    pub message: String,
}

impl Trap {
    pub fn new(message: String) -> Trap {
        Trap { message }
    }
}