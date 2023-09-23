use std::{fs, process};

#[derive(Debug)]
pub struct Program {
    pub data: Vec<u8>,
}

impl Program {
    pub fn new(path: &String) -> Self {
        Self { 
            data: {
                let result = fs::read(path);
                match result {
                    Ok(valid_path) => valid_path,
                    Err(_) => {
                        println!("The given path does not exist, exiting...");
                        process::exit(1)
                    },
                }  
            },
        }
    }
}