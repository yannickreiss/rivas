mod binary;
mod assembler;

use binary::*;
use core::panic;
use std::env;

fn main() {
    // load command line arguments
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    // check if number of supplied arguments is alright
    if argc < 1 {
        panic!("Missing Arguments!");
    }

    if argc % 2 == 1 {
        panic!("Wrong number of arguments given!");
    }

    // work through command line args
    println!("Running rivas Rust-Version at {}.", argv[0]);
    for i in 1..argc - 1 {
        match argv[i].as_str() {
            "-o" => println!("Set output name, Not yet implemented!"),
            "-h" => println!("Help: NYI!"),
            "-vhdl" => println!("Format Code for usage in VHDL: NYI!"),
            _ => println!("Input file translation: NYI!"),
        }
    }
}
