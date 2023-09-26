use std::{fs, process};

#[derive(Debug)]
pub struct Program {
    pub data: Vec<u8>,
    reel: [i8; 10],
    memory_index: usize,
    instruction_index: usize,
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
            memory_index: 0,
            instruction_index: 0,
            reel: [Default::default(); 10],
        }
    }

    pub fn add(&mut self) {
        let cell = self.reel.get_mut(self.memory_index);
        match cell {
            None => println!("you really messed up to get here..."),
            Some(valid_cell) => *valid_cell += 1,
        }
    }

    pub fn execute_instr(&mut self, instr: &u8) {
        let instr_char = *instr as char;
        match instr_char {
            '+' => self.add(),
            _ => println!("The current character is not a valid instruction"),
        }
    }


    pub fn run(&mut self) {
        while self.instruction_index < self.data.len() {
            let instr = self.data.get(self.instruction_index).unwrap().clone();
            self.execute_instr(&instr);
            self.instruction_index += 1;
        }
    }

    
}