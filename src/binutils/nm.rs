pub use crate::elf::{file::ELF64, strtab::StrTab, sym::ELF64_Sym};

pub fn nm(args: &Vec<String>) {
    for f in args {
        println!("{}: ", f);

        let mut elf: ELF64 = ELF64::open(f.as_str());

        for sh in &elf.shdrs {
            println!("{} {}", elf.section_strtab.name(sh.sh_name), sh.to_string());
        }

        for sh in &elf.shdrs {
            if sh.sh_type == crate::elf::shdr::SHT_SYMTAB {
                let symbols = sh.symbols(&mut elf.file);

                for symbol in symbols {
                    println!("{}", elf.symbol_strtab.name(symbol.st_name));
                }
            }
        }
    }
}
