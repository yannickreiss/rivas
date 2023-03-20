mod assembler;
mod binary;

use binary::{File};
use core::panic;
use std::env;

fn main() {
    // load command line arguments
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    // check if number of supplied arguments is alright
    if argc < 2 {
        panic!("Missing Arguments!");
    }

    let mut filename: &str = "";
    let mut file_out: &str = "";
    let mut vhdl_mode: bool = false;

    // work through command line args
    println!("Running rivas Rust-Version at {}.", argv[0]);

    let mut argb: bool = false; // If true this will pause the incrementation once
    for i in 1..argc {
        if argb {
            argb = false;
        } else {
            match argv[i].as_str() {
                "-o" => {
                    file_out = &argv[i + 1];
                    argb = true;
                }
                "-h" => println!("Help: NYI!"),
                "-vhdl" => vhdl_mode = true,
                _ => filename = &argv[i],
            }
        }
    }
    let mut file = binary::File::new(filename);
    file.set_mode(vhdl_mode);

    if file_out != "" {
        file.set_out(file_out);
    }

    file.translate();

    file.write();
}
