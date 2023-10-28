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

    pub fn run(&mut self) {
        while self.instruction_index < self.data.len() {
            let instr = self.data.get(self.instruction_index).unwrap().clone();
            self.execute_instr(&instr);
            self.instruction_index += 1;
        }
    }

    pub fn execute_instr(&mut self, instr: &u8) {
        let instr_char = *instr as char;
        match instr_char {
            '+' => self.add(),
            '-' => self.sub(),
            '>' => self.incr_mem_ptr(),
            '<' => self.dec_mem_ptr(),
            '[' => self.loop_start_indicator(),
            ']' => self.loop_end_indicator(),
            '.' => self.print_char(),
            _ => ()
        }
    }

    pub fn add(&mut self) {
        let cell = self.reel.get_mut(self.memory_index);
        match cell {
            None => println!("you really messed up to get here..."),
            Some(valid_cell) => *valid_cell += 1,
        }
    }

    pub fn sub(&mut self) {
        let cell = self.reel.get_mut(self.memory_index);
        match cell {
            None => println!("rip"),
            Some(valid_cell) => *valid_cell -= 1,
        }
    }

    pub fn incr_mem_ptr(&mut self) {
        self.memory_index += 1;
    }

    pub fn dec_mem_ptr(&mut self) {
        if self.memory_index > 0 {
            self.memory_index -= 1;
        }
    }

    pub fn loop_start_indicator(&mut self) {
        let cell = self.reel.get_mut(self.memory_index);
        match cell {
            None => println!("error"),
            Some(valid_cell) => {
                if *valid_cell == 0 {
                    let mut found_end = false;
                    while !found_end {
                        let instr = self.data.get(self.instruction_index).unwrap().clone() as char;
                        match instr {
                            ']' => found_end = true,
                            _ => self.instruction_index += 1,
                        }
                    }
                }
            }
        }
    }

    pub fn loop_end_indicator(&mut self) {
        let cell = self.reel.get_mut(self.memory_index);
        match cell {
            None => println!("error"),
            Some(valid_cell) =>
                if *valid_cell != 0 {
                    let mut loops_to_ignore = 0;
                    let mut found_start = false;
                    while !found_start {
                        self.instruction_index -= 1;
                        let instr = self.data.get(self.instruction_index).unwrap().clone() as char;
                        match instr {
                            ']' => loops_to_ignore += 1,
                            '[' => {
                                if loops_to_ignore > 0 {
                                    loops_to_ignore -= 1;
                                } else {
                                    found_start = true;
                                }
                            },
                            _ => continue
                        }
                    }
                }
        }
    }

    pub fn print_char(&mut self) {
        let cell = self.reel.get(self.memory_index);
        match cell {
            None => println!("error"),
            Some(valid_cell) => {
                let char_rep = *valid_cell as u8;
                let ascii_char = char_rep as char;
                print!("{}", ascii_char);
            }
        }
    }
}