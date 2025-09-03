use crate::make_u64;

use std::{fs::File, io::Read};

#[derive(Clone)]
pub struct ELF64_Rela {
    /// Location to apply the reloc action.
    pub r_offset: u64,

    /// This member gives both the symbol table index
    /// with respect to which the relocation must be
    /// made and the type of relocation to apply.
    /// Relocation types are processor-specific.
    pub r_info: u64,

    /// This member specifies a constant addend used
    /// to compute the value to be stored into the relocatable field.
    pub r_addend: u64,
}

impl ELF64_Rela {
    pub fn load_rel(file: &mut File) -> Self {
        let mut buf: [u8; 24] = [0; 24];
        file.read_exact(&mut buf).unwrap();

        let offset: u64 = make_u64(&buf, 0);
        let info = make_u64(&buf, 8);

        Self {
            r_offset: offset,
            r_info: info,
            r_addend: 0,
        }
    }

    pub fn load_rela(file: &mut File) -> Self {
        let mut buf: [u8; 24] = [0; 24];
        file.read_exact(&mut buf).unwrap();

        let offset: u64 = make_u64(&buf, 0);
        let info = make_u64(&buf, 8);
        let addend = make_u64(&buf, 16);

        Self {
            r_offset: offset,
            r_info: info,
            r_addend: addend,
        }
    }

    pub fn type_(&self) -> u32 {
        (self.r_info & 0xffffffff) as u32
    }

    pub fn symbol_(&self) -> u32 {
        (self.r_info >> 32) as u32
    }
}
