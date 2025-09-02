//! Provides string table.
//!

use super::shdr::ELF64_Shdr;

use std::fs::File;

pub struct StrTab {
    buf: Vec<u8>,
}

impl StrTab {
    pub fn new(file: &mut File, shdr: &ELF64_Shdr) -> Self {
        Self {
            buf: shdr.data(file),
        }
    }

    pub fn no_strtab() -> Self {
        Self { buf: vec![0; 8] }
    }

    pub fn name(&self, offset: u32) -> String {
        let mut ret = String::from("");
        let len = self.buf.len();

        let mut i: usize = offset as usize;
        while self.buf[i] != 0 {
            ret.push(self.buf[i] as char);
            i += 1;
        }

        ret
    }
}
