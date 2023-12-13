use std::env;
use std::io::prelude::*;
use std::fs::File;
use libaheui::AheuiState;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("error: supply a program file to run");
        return;
    }

    let filename = &args[1];
    let mut f = File::open(filename).expect("error opening file!");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    let mut program = AheuiState::init(&buffer);
    program.run();
}
