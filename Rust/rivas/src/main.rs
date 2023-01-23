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
    if argc < 1 {
        panic!("Missing Arguments!");
    }

    if argc % 2 == 1 {
        panic!("Wrong number of arguments given!");
    }

    let mut filename: &str = "";
    let mut file_out: &str = "";

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
                "-vhdl" => println!("Format Code for usage in VHDL: NYI!"),
                _ => filename = &argv[i],
            }
        }
    }
    let mut file = binary::File::new(filename);

    if file_out != "" {
        file.set_out(file_out);
    }

    file.translate();

    file.write();
}
