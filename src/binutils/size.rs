pub use crate::elf::{
    file::ELF64, 
    phdr::ELF64_Phdr
};

pub fn size(args: &Vec<String>) {
    let len = args.len();

    for i in 2..len {
        let f = &args[i];
        let elf: ELF64 = ELF64::open(f.as_str());

        let mut text: usize = 0;
        let mut data: usize = 0;
        let mut bss: usize = 0;

        for phdr in elf.phdrs {
            let w = phdr.is_writable();
            let x = phdr.is_executable();

            if w {
                bss += phdr.p_memsz as usize;
            } else if x {
                text += phdr.p_memsz as usize;
            } else {
                data += phdr.p_memsz as usize;
            }
        }

        println!("{}\t{}\t{}\t{}", text, data, bss, text + data + bss);
    }
}
