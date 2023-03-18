use crate::runtime::memory::MemoryMap;
use crate::runtime::memory::MemorySegment;
use crate::runtime::memory::SegmentDirection;

use super::errors::{RuntimeError, FatalErrorType};

pub struct MemoryLayout {
    pub text_low: usize,
    pub data_low: usize,
    pub heap_low: usize,

    pub mmio_high: usize,
    pub stack_high: usize,

    pub heap_size: usize,
    pub stack_size: usize,
}

impl MemoryLayout {
    /// The MIPS memory layout as used in the MARS simulator.
    pub fn mars(heap_size: usize, stack_size: usize) -> MemoryLayout {
        MemoryLayout {
            text_low: 0x00400000,
            data_low: 0x10010000,
            heap_low: 0x10080000,
            mmio_high: 0xFFFF0000,
            stack_high: 0x7FFFFFFF,
            heap_size,
            stack_size,
        }
    }
}

pub struct VM {
    registers: [u32; 32],
    pub memory: MemoryMap,

    /// The program counter.
    pc: usize,

    hi: u32,
    lo: u32,
}

impl VM {
    pub fn new(layout: MemoryLayout) -> VM {
        let mut memory = MemoryMap::new();

        memory.add_segment(MemorySegment::new(
            String::from("text"),
            layout.text_low,
            layout.data_low - layout.text_low,
            false,
            SegmentDirection::Up,
            true,
        ));

        memory.add_segment(MemorySegment::new(
            String::from("data"),
            layout.data_low,
            layout.heap_low - layout.data_low,
            false,
            SegmentDirection::Up,
            true,
        ));

        memory.add_segment(MemorySegment::new(
            String::from("heap"),
            layout.heap_low,
            layout.heap_size,
            false,
            SegmentDirection::Up,
            false,
        ));

        memory.add_segment(MemorySegment::new(
            String::from("mmio"),
            layout.mmio_high,
            layout.mmio_high - layout.stack_high,
            false,
            SegmentDirection::Down,
            false,
        ));

        memory.add_segment(MemorySegment::new(
            String::from("stack"),
            layout.stack_high,
            layout.stack_size,
            false,
            SegmentDirection::Down,
            false,
        ));

        VM {
            registers: [0; 32],
            memory,
            pc: layout.text_low,
            hi: 0,
            lo: 0,
        }
    }

    /// Set the given register to the given value.
    ///
    /// Throw an error if the register number is invalid, or if attempting to set the zero register.
    pub fn set_register(&mut self, register: u8, value: u32) -> Result<(), RuntimeError> {
        if register == 0 {
            return Err(RuntimeError::new(
                FatalErrorType::IllegalRegisterAccess,
                String::from("Attempted to set the zero register"),
            ));
        }

        if register > 31 {
            return Err(RuntimeError::new(
                FatalErrorType::IllegalRegisterAccess,
                format!("Invalid register number {}", register),
            ));
        }

        self.registers[register as usize] = value;

        Ok(())
    }

    /// Get the value of the given register.
    ///
    /// Throw an error if the register number is invalid.
    pub fn get_register(&self, register: u8) -> Result<u32, RuntimeError> {
        if register > 31 {
            Err(RuntimeError::new(
                FatalErrorType::IllegalRegisterAccess,
                format!("Invalid register number {}", register),
            ))
        } else {
            Ok(self.registers[register as usize])
        }
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn set_pc(&mut self, value: usize) {
        self.pc = value;
    }

    pub fn get_hi(&self) -> u32 {
        self.hi
    }

    pub fn set_hi(&mut self, value: u32) {
        self.hi = value;
    }

    pub fn get_lo(&self) -> u32 {
        self.lo
    }

    pub fn set_lo(&mut self, value: u32) {
        self.lo = value;
    }
}
