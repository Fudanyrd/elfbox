use std::env;
use std::fs::File;

use rld::elf;

use rld::binutils::elfbox_main;

fn main() {
    let args: Vec<String> = env::args().collect();

    elfbox_main(&args);
}
