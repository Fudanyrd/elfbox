pub use std::fs::File;

pub use super::{
    ehdr::ELF64_Ehdr,
    phdr::ELF64_Phdr,
    shdr::{ELF64_Shdr, SHT_STRTAB},
    strtab::StrTab,
    sym::ELF64_Sym,
};

pub struct ELF64 {
    pub ehdr: ELF64_Ehdr,
    pub phdrs: Vec<ELF64_Phdr>,
    pub shdrs: Vec<ELF64_Shdr>,

    pub section_strtab: StrTab,
    pub symbol_strtab: StrTab,

    pub file: File,
}

impl ELF64 {
    pub fn open(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut ehdr: ELF64_Ehdr = ELF64_Ehdr::zero_init();
        ehdr.load_from(&mut file);

        let phdrs = ELF64_Phdr::headers(&mut file, &ehdr);
        let shdrs = ELF64_Shdr::headers(&mut file, &ehdr);
        let shstrndx = ehdr.e_shstrndx;

        let mut symstrndx: u16 = 0;

        for i in 0..ehdr.e_shnum {
            let sh = &shdrs[i as usize];

            if sh.sh_type == SHT_STRTAB && i != shstrndx {
                symstrndx = i;
                break;
            }
        }

        let section_strtab_sh = shdrs[shstrndx as usize].clone();
        let symbol_strtab_sh = shdrs[symstrndx as usize].clone();

        Self {
            ehdr: ehdr,
            phdrs: phdrs,
            shdrs: shdrs,

            section_strtab: StrTab::new(&mut file, &section_strtab_sh),
            symbol_strtab: StrTab::new(&mut file, &symbol_strtab_sh),

            file: file,
        }
    }
}
