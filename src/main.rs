mod assembler;
mod binary;

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
    let mut file_out_name: String;

    // work through command line args
    println!("Running rivas Rust-Version at {}.", argv[0]);

    let mut argb: bool = false; // If true this will pause the incrementation once
    for i in 1..argc {
        if argb {
            argb = false;
        } else {
            match argv[i].as_str() {
                "-o" => {
                    file_out_name = String::from("") + &argv[i + 1] + ".vhdl";
                    file_out = file_out_name.as_str();
                    argb = true;
                }
                "-h" => {
                    println!("rivas - command line options");
                    println!("-o <filename>\t\tSet filename for file output.");
                    println!("-h\t\t\t Print this help message.");
                    println!("-vhdl\t\t\t Print VHDL compatible binary values and stores them in a file.");
                }
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
