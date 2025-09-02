use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{make_u64, make_u16, make_u32};

pub struct ELF64_Phdr {
    p_type: u32,
    p_flags: u32,
    p_off: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

impl ELF64_Phdr {
    pub fn zero_init() -> Self {
        Self {
            p_type: 0,
            p_flags: 0,
            p_off: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }


    /// Load from a ELF file object.
    pub fn load_from(&mut self, file: &mut File) {
        let mut buf: [u8; 56] = [0; 56];
        file.read_exact(&mut buf).unwrap();

        self.p_type = make_u32(& buf, 0);
        self.p_flags= make_u32(& buf, 4);
        self.p_off = make_u64(& buf, 8);
        self.p_vaddr = make_u64(& buf, 16);
        self.p_paddr = make_u64(& buf, 24);
        self.p_filesz = make_u64(& buf, 32);
        self.p_memsz = make_u64(& buf, 40);
        self.p_align = make_u64(& buf, 48);

        if self.p_memsz < self.p_filesz {
            panic!("Incorrect program header(memsz < filesz)");
        }
    }
}
