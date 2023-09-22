use std::env;
use std::process;
use std::fs;

use crate::program::Program;

pub fn read_program() -> Program {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ERROR: File path argument missing");
        process::exit(1);
    } else if args.len() > 2 {
        println!("ERROR: Too many arguments provided, only path needed");
        process::exit(1);
    } else {}

    let mut iter = args.iter();
    iter.next();
    return Program::new(iter.next().unwrap()); //not very safe
}