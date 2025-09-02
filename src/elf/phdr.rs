use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::ehdr::ELF64_Ehdr;
use super::{make_u16, make_u32, make_u64};

#[derive(Clone)]
pub struct ELF64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_off: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

/// an executable segment.
pub const PF_X: u32 = 1 << 0;
/// an writable segment.
pub const PF_W: u32 = 1 << 1;
/// an readable segment.
pub const PF_R: u32 = 1 << 2;

/// the segment should be load.
pub const PT_LOAD: u32 = 1;

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

        self.p_type = make_u32(&buf, 0);
        self.p_flags = make_u32(&buf, 4);
        self.p_off = make_u64(&buf, 8);
        self.p_vaddr = make_u64(&buf, 16);
        self.p_paddr = make_u64(&buf, 24);
        self.p_filesz = make_u64(&buf, 32);
        self.p_memsz = make_u64(&buf, 40);
        self.p_align = make_u64(&buf, 48);

        if self.p_memsz < self.p_filesz {
            panic!("Incorrect program header(memsz < filesz)");
        }
    }

    pub fn is_load(&self) -> bool {
        self.p_type == PT_LOAD
    }

    pub fn is_readable(&self) -> bool {
        (self.p_flags & PF_R) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.p_flags & PF_W) != 0
    }

    pub fn is_executable(&self) -> bool {
        (self.p_flags & PF_X) != 0
    }

    pub fn data(&self, file: &mut File) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![0; self.p_memsz as usize];

        file.seek(SeekFrom::Start(self.p_off)).unwrap();
        file.read_exact(ret.as_mut_slice()).unwrap();

        for i in self.p_filesz..self.p_memsz {
            ret[i as usize] = 0 as u8;
        }

        ret
    }

    fn flagstr(&self) -> String {
        let r: String;
        if self.is_readable() {
            r = String::from("R").to_string();
        } else {
            r = String::from(" ").to_string();
        }
        let w: String;
        if self.is_writable() {
            w = String::from("W").to_string();
        } else {
            w = String::from(" ").to_string();
        }
        let x: String;
        if self.is_executable() {
            x = String::from("X").to_string();
        } else {
            x = String::from(" ").to_string();
        }

        return r + &w + &x;
    }

    pub fn display(&self) {
        let rwx = self.flagstr();
        println!("{} {} {}", self.p_off, self.p_vaddr, rwx);
    }

    pub fn headers(file: &mut File, ehdr: &ELF64_Ehdr) -> Vec<ELF64_Phdr> {
        let phnum: usize = ehdr.e_phnum as usize;
        let mut ret: Vec<ELF64_Phdr> = vec![ELF64_Phdr::zero_init(); phnum as usize];

        file.seek(SeekFrom::Start(ehdr.e_phoff)).unwrap();

        for i in 0..phnum {
            ret[i].load_from(file);
        }

        ret
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::from("Phdr {");

        ret += "\"vaddr\": ";
        ret += self.p_vaddr.to_string().as_str();
        ret += ",\"offset\": ";
        ret += self.p_off.to_string().as_str();
        ret += ",\"flags\": ";
        ret += self.flagstr().as_str();
        ret += "}";

        ret
    }
}
