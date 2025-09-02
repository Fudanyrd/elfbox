pub use crate::elf::{
    ehdr::ELF64_Ehdr, file::ELF64, phdr::ELF64_Phdr, shdr::ELF64_Shdr, strtab::StrTab,
    sym::ELF64_Sym,
};

pub mod nm;
pub use nm::nm;

pub mod size; 
pub use size::size;

pub fn elfbox_main(args: &Vec<String>) {
    match args[1].as_str() {
        "nm" => {
            nm(args);
        },
        "size" => {
            size(args);
        },
        _ => {
            println!("applet {} not implemented.", args[1]);
            panic!("");
        }
    }
}
