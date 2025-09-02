//! Definition for ELF Section Header.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::{ehdr::ELF64_Ehdr, sym::ELF64_Sym};
use super::{make_u16, make_u32, make_u64};

#[derive(Clone)]
pub struct ELF64_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;

impl ELF64_Shdr {
    pub fn zero_init() -> Self {
        Self {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }

    pub fn load_from(&mut self, file: &mut File) {
        let mut buf: [u8; 64] = [0; 64];
        file.read_exact(&mut buf).unwrap();

        self.sh_name = make_u32(&buf, 0);
        self.sh_type = make_u32(&buf, 4);
        self.sh_flags = make_u64(&buf, 8);
        self.sh_addr = make_u64(&buf, 16);
        self.sh_offset = make_u64(&buf, 24);
        self.sh_size = make_u64(&buf, 32);
        self.sh_link = make_u32(&buf, 40);
        self.sh_info = make_u32(&buf, 44);
        self.sh_addralign = make_u64(&buf, 48);
        self.sh_entsize = make_u64(&buf, 56);
    }

    pub fn headers(file: &mut File, ehdr: &ELF64_Ehdr) -> Vec<ELF64_Shdr> {
        let shnum: usize = ehdr.e_shnum as usize;
        let mut ret = vec![ELF64_Shdr::zero_init(); shnum];

        file.seek(SeekFrom::Start(ehdr.e_shoff)).unwrap();

        for i in 0..shnum {
            ret[i].load_from(file);
        }

        ret
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::from("shdr {");

        ret += "\"type\": ";
        ret += match self.sh_type {
            SHT_NULL => " null ",
            SHT_PROGBITS => " prog ",
            SHT_STRTAB => "strtab",
            SHT_SYMTAB => "symtab",
            SHT_RELA => " rela ",
            _ => "other ",
        };
        ret += ",\"size\": ";
        ret += self.sh_size.to_string().as_str();
        ret += ",\"offset\": ";
        ret += self.sh_offset.to_string().as_str();
        ret += "}";

        ret
    }

    pub fn data(&self, file: &mut File) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![0; self.sh_size as usize];

        file.seek(SeekFrom::Start(self.sh_offset)).unwrap();
        file.read_exact(ret.as_mut_slice()).unwrap();

        ret
    }

    pub fn symbols(&self, file: &mut File) -> Vec<ELF64_Sym> {
        let size = (self.sh_size / 24) as usize;
        let mut ret: Vec<ELF64_Sym> = vec![ELF64_Sym::zero_init(); size];

        file.seek(SeekFrom::Start(self.sh_offset)).unwrap();

        for i in 0..size {
            ret[i].load_from(file);
        }

        ret
    }
}
