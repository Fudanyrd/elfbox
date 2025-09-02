use std::env;
use std::fs::File;

use rld::elf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file: File = File::open(&args[1]).unwrap();

    let mut ehdr: elf::ehdr::ELF64_Ehdr = elf::ehdr::ELF64_Ehdr::zero_init();
    ehdr.load_from(&mut file);

    let phdrs = elf::phdr::ELF64_Phdr::headers(&mut file, &ehdr);

    for phdr in phdrs {
        phdr.display();
    }

    println!("Hello, world!");
}
