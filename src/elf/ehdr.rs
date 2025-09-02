use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{make_u64, make_u16, make_u32};

pub struct ELF64_Ehdr {
    e_ident: [u8; 16], // Magic number and other info
    e_type: u16,       // Object file type
    e_machine: u16,    // Architecture
    e_version: u32,    // Object file version
    e_entry: u64,      // Entry point virtual address
    e_phoff: u64,      // Program header table file offset
    e_shoff: u64,      // Section header table file offset
    e_flags: u32,      // Processor-specific flags
    e_ehsize: u16,     // ELF header size in bytes
    e_phentsize: u16,  // Program header table entry size
    e_phnum: u16,      // Program header table entry count
    e_shentsize: u16,  // Section header table entry size
    e_shnum: u16,      // Section header table entry count
    e_shstrndx: u16,   // Section header string table index
}

impl ELF64_Ehdr {
    pub fn load_from(&mut self, file: &mut File) {
        // Implementation to load ELF header from a file
        let mut buf: [u8; 64] = [0; 64];
        file.seek(SeekFrom::Start(0)).unwrap();
        file.read_exact(&mut buf).unwrap();

        for i in 0..16 {
            self.e_ident[i] = buf[i];
        }

        self.e_type = make_u16(&buf, 16);
        self.e_machine = make_u16(&buf, 18);
        self.e_version = make_u32(&buf, 20);
        self.e_entry = make_u64(&buf, 24);
        self.e_phoff = make_u64(&buf, 32);
        self.e_shoff = make_u64(&buf, 40);
        self.e_flags = make_u32(&buf, 48);
        self.e_ehsize = make_u16(&buf, 52);
        self.e_phentsize = make_u16(&buf, 54);
        self.e_phnum = make_u16(&buf, 56);
        self.e_shentsize = make_u16(&buf, 58);
        self.e_shnum = make_u16(&buf, 60);
        self.e_shstrndx = make_u16(&buf, 62);
    }

    pub fn zero_init() -> Self {
        Self {
            e_ident: [0; 16],
            e_type: 0,      // Object file type
            e_machine: 0,   // Architecture
            e_version: 0,   // Object file version
            e_entry: 0,     // Entry point virtual address
            e_phoff: 0,     // Program header table file offset
            e_shoff: 0,     // Section header table file offset
            e_flags: 0,     // Processor-specific flags
            e_ehsize: 0,    // ELF header size in bytes
            e_phentsize: 0, // Program header table entry size
            e_phnum: 0,     // Program header table entry count
            e_shentsize: 0, // Section header table entry size
            e_shnum: 0,     // Section header table entry count
            e_shstrndx: 0,  // Section header string table index
        }
    }
}
