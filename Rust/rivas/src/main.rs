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

    
}
