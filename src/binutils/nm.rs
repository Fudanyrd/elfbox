pub use crate::elf::{file::ELF64, strtab::StrTab, sym::ELF64_Sym};

pub fn nm(args: &Vec<String>) {
    let len = args.len();
    for i in 2..len {
        let f = &args[i];
        println!("{}: ", f);

        let mut elf: ELF64 = ELF64::open(f.as_str());

        // for sh in &elf.shdrs {
        //     println!("{} {}", elf.section_strtab.name(sh.sh_name), sh.to_string());
        // }

        for sh in &elf.shdrs {
            if sh.sh_type == crate::elf::shdr::SHT_SYMTAB {
                let symbols = sh.symbols(&mut elf.file);

                for symbol in symbols {
                    let symbol_name = elf.symbol_strtab.name(symbol.st_name);
                    if symbol_name.len() > 0 {
                        println!("{} ({})", symbol_name, symbol.precedence().to_string());
                    }
                }
            }
        }
    }
}
