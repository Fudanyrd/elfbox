//! Provides definition of ELF Symbol.
//!

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::ehdr::ELF64_Ehdr;
use super::{make_u16, make_u32, make_u64};

#[derive(Clone)]
pub struct ELF64_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

pub enum Precedence {
    NULL,
    LOCAL,
    GLOBAL,
    WEAK,
}

impl Precedence {
    pub fn to_string(&self) -> String {
        match self {
            Self::LOCAL => "L".to_string(),
            Self::GLOBAL => "G".to_string(),
            Self::WEAK => "W".to_string(),
            _ => "U".to_string(),
        }
    }
}

pub const STT_NOTYPE: u32 = 0;
pub const STT_OBJECT: u32 = 1;
pub const STT_FUNC: u32 = 2;
pub const STT_SECTION: u32 = 3;
pub const STT_FILE: u32 = 4;
pub const STT_COMMON: u32 = 5;
pub const STT_TLS: u32 = 6;
pub const STT_NUM: u32 = 7;
pub const STT_LOOS: u32 = 10;
pub const STT_GNU_IFUNC: u32 = 10;
pub const STT_HIOS: u32 = 12;
pub const STT_LOPROC: u32 = 13;
pub const STT_HIPROC: u32 = 15;
pub const STT_SPARC_REGISTER: u32 = 13;

pub const STB_LOCAL : u8 =	0		/* Local symbol */;
pub const STB_GLOBAL : u8 =	1		/* Global symbol */;
pub const STB_WEAK : u8 =	2		/* Weak symbol */;
pub const STB_NUM : u8 =		3		/* Number of defined types.  */;
pub const STB_LOOS : u8 =	10		/* Start of OS-specific */;
pub const STB_GNU_UNIQUE : u8 =	10		/* Unique symbol.  */;
pub const STB_HIOS : u8 =	12		/* End of OS-specific */;
pub const STB_LOPROC : u8 =	13		/* Start of processor-specific */;
pub const STB_HIPROC : u8 =	15		/* End of processor-specific */;

impl ELF64_Sym {
    pub fn zero_init() -> Self {
        Self {
            st_name: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
        }
    }

    pub fn load_from(&mut self, file: &mut File) {
        let mut buf: [u8; 24] = [0; 24];
        file.read_exact(&mut buf).unwrap();

        self.st_name = make_u32(&buf, 0);
        self.st_info = buf[4];
        self.st_other = buf[5];
        self.st_shndx = make_u16(&buf, 6);
        self.st_value = make_u64(&buf, 8);
        self.st_size = make_u64(&buf, 16);
    }

    pub fn precedence(&self) -> Precedence {
        match self.st_info & 0xf {
            STB_LOCAL => Precedence::LOCAL,
            STB_GLOBAL => Precedence::GLOBAL,
            STB_WEAK => Precedence::WEAK,
            _ => {
                // panic!("unsupported symbol type.");
                Precedence::NULL
            }
        }
    }
}
