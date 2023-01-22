use assembler::translate;
// imports for file operations
use std::fs;

use crate::assembler;

pub // file
// store information about the input and output file
struct file {
    name_in: String,
    name_out: String,
    cont_in: Vec<String>,
    cont_out: Vec<String>,
}

// Implementation of file
// new, set_out, translate, write
impl file {
    pub fn new(input_filename: &str) -> file {
        
        let content: String = fs::read_to_string(input_filename).expect("Could not open File!");

        let mut lines: Vec<String> = Vec::new();

        for instruction in content.lines() {
            lines.push(String::from(instruction));
        }

        file {
            name_in: input_filename.to_owned(),
            name_out: String::from("out.rvb"),
            cont_in: lines,
            cont_out: Vec::new(),
        }

    }

    pub fn set_out(&mut self, output_filename: &str) {
        self.name_out = output_filename.to_owned();
    }

    pub fn translate(&mut self) {
        
    }

    pub fn write(&mut self) {}
}