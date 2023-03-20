use assembler::translate;
// imports for file operations
use std::fs;

use crate::assembler;

pub // file
// store information about the input and output file
struct File {
    name_in: String,
    name_out: String,
    cont_in: Vec<String>,
    cont_out: Vec<String>,
    vhdl_mode: bool,
}

// Implementation of file
// new, set_out, translate, write
impl File {
    pub fn new(input_filename: &str) -> File {
        let content: String = fs::read_to_string(input_filename).expect("Could not open File {}!");

        let mut lines: Vec<String> = Vec::new();

        for instruction in content.lines() {
            lines.push(String::from(instruction));
        }

        File {
            name_in: input_filename.to_owned(),
            name_out: String::from("out.rvb"),
            cont_in: lines,
            cont_out: Vec::new(),
            vhdl_mode: false,
        }
    }

    pub fn set_out(&mut self, output_filename: &str) {
        self.name_out = output_filename.to_owned();
    }

    pub fn translate(&mut self) {
        for i in 0..self.cont_in.len() {
            let binary = translate(&self.cont_in[i]);

            // catch empty lines
            if binary != String::from("") {
                self.cont_out.push(binary);
            }
        }
    }

    pub fn write(&mut self) {
        println!("Assembling {} into {}", self.name_in, self.name_out);
        if self.vhdl_mode {
            println!("(");
            for line in 0..self.cont_out.len() {
                println!("b\"{}\", ", self.cont_out[line]);
            }
            println!("others => (others => '0')");
            println!(");");
        } else {
            // TODO: write to file in binary
        }
    }

    pub fn set_mode(&mut self, mode: bool) {
        self.vhdl_mode = mode;
    }
}

fn binary_to_character(word: &str) -> String {
    if word.len() != 32 {
        panic!("Word could not be converted to bytes: Length doesn't match!");
    }

    String::new()
}
