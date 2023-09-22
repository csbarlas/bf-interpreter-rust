use std::fs;

#[derive(Debug)]
pub struct Program {
    pub data: Vec<u8>,
}

impl Program {
    pub fn new(path: &String) -> Self {
        Self { 
            data: fs::read(path).expect("Could not open file"),
        }
    }
}