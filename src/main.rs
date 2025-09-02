use std::env;
use std::fs::File;

use rld::elf;

use rld::binutils::nm;

fn main() {
    let args: Vec<String> = env::args().collect();

    nm(&args);
}
