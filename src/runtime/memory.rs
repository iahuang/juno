use crate::runtime::logging::fatal_error;
use crate::runtime::logging::FatalErrorType;

pub enum SegmentDirection {
    Up,
    Down,
}

/// A representation of a segment of the MIPS memory layout.
///
/// MIPS may address up to 4GiB of memory, but allocating that much memory is often not
/// efficient, since much of it will not be used. A `MemorySegment` represents a contiguous
/// range of memory, and only allocates memory for the parts of the segment that are actually
/// used.
pub struct MemorySegment {
    pub name: String,

    /// The address of the first byte of this segment.
    /// If direction is Up, this is the lowest address.
    /// If direction is Down, this is the highest address.
    start_address: usize,

    size: usize,
    pub is_static: bool,
    pub direction: SegmentDirection,

    data: Vec<u8>,

    read_only: bool,
}

impl MemorySegment {
    /// If `is_static` is true, the segment will be allocated with the given size and filled with zeros.
    /// If `is_static` is false, the segment will be allocated with size 0 and will grow as needed.
    ///
    /// If `direction` is Up, the segment will grow upwards from the start address,
    /// and the start address will be the lowest address.
    ///
    /// If `direction` is Down, the segment will grow downwards from the start address,
    /// and the start address will be the highest address.
    pub fn new(
        name: String,
        start_address: usize,
        size: usize,
        is_static: bool,
        direction: SegmentDirection,
        read_only: bool,
    ) -> MemorySegment {
        MemorySegment {
            name,
            start_address,
            size,
            is_static,
            direction,
            data: match is_static {
                true => vec![0; size],
                false => Vec::new(),
            },
            read_only: read_only,
        }
    }

    pub fn allow_writes(&mut self) {
        self.read_only = false;
    }

    pub fn set_read_only(&mut self) {
        self.read_only = true;
    }

    /// Return the offset of the given address within this segment.
    /// If the address is out of bounds, panic.
    fn get_offset(&self, address: usize) -> usize {
        if address < self.get_low_address() || address > self.get_high_address() {
            fatal_error(
                FatalErrorType::IllegalMemoryAccess,
                format!(
                    "Address 0x{:x} out of bounds for segment \"{}\" from {:#010x} to {:#010x}",
                    address,
                    self.name,
                    self.get_low_address(),
                    self.get_high_address()
                ),
            );
        }

        address - self.get_low_address()
    }

    pub fn get_byte(&self, address: usize) -> u8 {
        let offset = self.get_offset(address);

        // check if the offset is in the data vector
        if offset >= self.data.len() {
            // if not, return 0
            0
        } else {
            // if so, return the byte at that offset
            self.data[offset]
        }
    }

    pub fn get_halfword(&self, address: usize) -> u16 {
        let hi = self.get_byte(address) as u16;
        let lo = self.get_byte(address + 1) as u16;

        (hi << 8) | lo
    }

    pub fn get_word(&self, address: usize) -> u32 {
        let b1 = self.get_byte(address) as u32;
        let b2 = self.get_byte(address + 1) as u32;
        let b3 = self.get_byte(address + 2) as u32;
        let b4 = self.get_byte(address + 3) as u32;

        (b1 << 24) | (b2 << 16) | (b3 << 8) | b4
    }

    /// Set the byte at the given address to the given value.
    pub fn set_byte(&mut self, address: usize, value: u8) {
        let offset = self.get_offset(address);

        if self.read_only {
            fatal_error(
                FatalErrorType::IllegalMemoryAccess,
                format!(
                    "Attempted to write to read-only segment \"{}\" at address {:#010x}",
                    self.name, address
                ),
            );
        }

        // check if the offset is in the data vector
        if offset >= self.data.len() {
            // if not, check if the segment is static
            if self.is_static {
                // if so, panic
                panic!("Attempted to grow static segment \"{}\"", self.name);
            } else {
                // if not, grow the data vector to the offset
                self.data.resize(offset + 1, 0);
            }
        }

        // set the byte at the offset
        self.data[offset] = value;
    }

    /// Set the halfword at the given address to the given value.
    ///
    /// If the address is not aligned to a halfword boundary, throw a fatal error.
    pub fn set_halfword(&mut self, address: usize, value: u16) {
        if address % 2 != 0 {
            fatal_error(
                FatalErrorType::IllegalMemoryAccess,
                format!(
                    "Attempted to write halfword to unaligned address {:#010x}",
                    address
                ),
            );
        }

        let hi = (value >> 8) as u8;
        let lo = value as u8;

        self.set_byte(address, hi);
        self.set_byte(address + 1, lo);
    }

    /// Set the word at the given address to the given value.
    ///
    /// If the address is not aligned to a word boundary, throw a fatal error.
    pub fn set_word(&mut self, address: usize, value: u32) {
        if address % 4 != 0 {
            fatal_error(
                FatalErrorType::IllegalMemoryAccess,
                format!(
                    "Attempted to write word to unaligned address {:#010x}",
                    address
                ),
            );
        }

        let b1 = (value >> 24) as u8;
        let b2 = (value >> 16) as u8;
        let b3 = (value >> 8) as u8;
        let b4 = value as u8;

        self.set_byte(address, b1);
        self.set_byte(address + 1, b2);
        self.set_byte(address + 2, b3);
        self.set_byte(address + 3, b4);
    }

    pub fn get_start_address(&self) -> usize {
        self.start_address
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_high_address(&self) -> usize {
        if matches!(self.direction, SegmentDirection::Up) {
            self.start_address + self.size - 1
        } else {
            self.start_address
        }
    }

    pub fn get_low_address(&self) -> usize {
        if matches!(self.direction, SegmentDirection::Up) {
            self.start_address
        } else {
            self.start_address - self.size + 1
        }
    }
}

/// A memory map is a collection of segments.
pub struct MemoryMap {
    segments: Vec<MemorySegment>,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    pub fn add_segment(&mut self, segment: MemorySegment) {
        self.segments.push(segment);

        // check if the segment overlaps with any other segment; if so, panic

        for segment in &self.segments {
            for other_segment in &self.segments {
                if segment as *const MemorySegment == other_segment as *const MemorySegment {
                    continue;
                }

                if segment.get_low_address() < other_segment.get_high_address()
                    && segment.get_high_address() > other_segment.get_low_address()
                {
                    panic!(
                        "Segment \"{}\" overlaps with segment \"{}\"",
                        segment.name, other_segment.name
                    );
                }
            }
        }
    }

    pub fn get_segment(&self, address: usize) -> Option<&MemorySegment> {
        for segment in &self.segments {
            if address >= segment.get_low_address() && address <= segment.get_high_address() {
                return Some(segment);
            }
        }

        None
    }

    pub fn get_segment_mut(&mut self, address: usize) -> Option<&mut MemorySegment> {
        for segment in &mut self.segments {
            if address >= segment.get_low_address() && address <= segment.get_high_address() {
                return Some(segment);
            }
        }

        None
    }

    fn invalid_read(&self, address: usize) -> ! {
        fatal_error(
            FatalErrorType::IllegalMemoryAccess,
            format!("Invalid read at {:#010x}", address),
        );
    }

    fn invalid_write(&self, address: usize) -> ! {
        fatal_error(
            FatalErrorType::IllegalMemoryAccess,
            format!("Invalid write at {:#010x}", address),
        );
    }

    pub fn get_byte(&self, address: usize) -> u8 {
        if let Some(segment) = self.get_segment(address) {
            segment.get_byte(address)
        } else {
            self.invalid_read(address);
        }
    }

    pub fn get_halfword(&self, address: usize) -> u16 {
        if let Some(segment) = self.get_segment(address) {
            segment.get_halfword(address)
        } else {
            self.invalid_read(address);
        }
    }

    pub fn get_word(&self, address: usize) -> u32 {
        if let Some(segment) = self.get_segment(address) {
            segment.get_word(address)
        } else {
            self.invalid_read(address);
        }
    }

    pub fn set_byte(&mut self, address: usize, value: u8) {
        if let Some(segment) = self.get_segment_mut(address) {
            segment.set_byte(address, value);
        } else {
            self.invalid_write(address);
        }
    }

    pub fn set_halfword(&mut self, address: usize, value: u16) {
        if let Some(segment) = self.get_segment_mut(address) {
            segment.set_halfword(address, value);
        } else {
            self.invalid_write(address);
        }
    }

    pub fn set_word(&mut self, address: usize, value: u32) {
        if let Some(segment) = self.get_segment_mut(address) {
            segment.set_word(address, value);
        } else {
            self.invalid_write(address);
        }
    }

    pub fn get_segments(&self) -> &Vec<MemorySegment> {
        &self.segments
    }

    pub fn segment_by_name(&self, name: &str) -> Option<&MemorySegment> {
        for segment in &self.segments {
            if segment.name == name {
                return Some(segment);
            }
        }

        None
    }

    pub fn mut_segment_by_name(&mut self, name: &str) -> Option<&mut MemorySegment> {
        for segment in &mut self.segments {
            if segment.name == name {
                return Some(segment);
            }
        }

        None
    }

    /// Return the first address that is aligned to the given alignment, starting from `address`,
    /// and moving in the given direction.
    pub fn align_address(
        &self,
        address: usize,
        alignment: u8,
        direction: SegmentDirection,
    ) -> usize {
        let mut address = address;

        while address % alignment as usize != 0 {
            match direction {
                SegmentDirection::Up => {
                    address += 1;
                }
                SegmentDirection::Down => {
                    address -= 1;
                }
            }
        }

        address
    }
}
