use std::{
    error::Error,
    fs::File,
    io::{stdout, Read, Stdout, Write},
};

use super::util::get_u8_from_console;

pub static OPERATORS: [char; 8] = ['+', '-', '>', '<', '[', ']', '.', ','];

pub struct Rustfuck {
    pub path: String,
    pub program: Vec<char>,
    pub stdout: Stdout,
    pub state: InterpreterState,
}

pub struct InterpreterState {
    pub memory: [u8; 128],
    pub mem_ptr: usize,
    pub prog_ptr: usize,
    pub backhops: Vec<usize>,
    pub depth: usize,
}

impl Rustfuck {
    pub fn new(path: &String) -> Self {
        let state = InterpreterState {
            memory: [0; 128],
            mem_ptr: 0,
            prog_ptr: 0,
            backhops: Vec::new(),
            depth: 0,
        };

        return Self {
            path: path.clone(),
            program: Vec::new(),
            stdout: stdout(),
            state: state,
        };
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(&self.path)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        buf.chars().filter(|c| OPERATORS.contains(c)).for_each(|c| {
            self.program.push(c);
        });

        Ok(())
    }

    pub fn interp(&mut self) -> Result<(), Box<dyn Error>> {
        self.load()?;

        loop {
            let op = match self.program.get(self.state.prog_ptr) {
                Some(op) => op.clone(),
                None => {
                    break;
                }
            };

            match op {
                '+' => {
                    self.state.memory[self.state.mem_ptr] += 1;
                    self.state.prog_ptr += 1;
                }
                '-' => {
                    self.state.memory[self.state.mem_ptr] -= 1;
                    self.state.prog_ptr += 1;
                }
                '>' => {
                    self.state.mem_ptr += 1;
                    self.state.prog_ptr += 1;
                }
                '<' => {
                    self.state.mem_ptr -= 1;
                    self.state.prog_ptr += 1;
                }
                '[' => {
                    self.state.backhops.push(self.state.prog_ptr);
                    self.state.depth += 1;
                    self.state.prog_ptr += 1;
                }
                ']' => {
                    self.state.depth -= 1;

                    if self.state.depth != 0 {
                        self.state.prog_ptr += 1;
                        continue;
                    }

                    if self.state.memory[self.state.mem_ptr] == 0 {
                        self.state.prog_ptr += 1;
                        continue;
                    }

                    let Some(backhop) = self.state.backhops.pop() else {
                        continue;
                    };

                    self.state.prog_ptr = backhop;
                }
                ',' => {
                    let val = get_u8_from_console();
                    self.state.memory[self.state.mem_ptr] = val;
                    self.state.prog_ptr += 1;
                }
                '.' => {
                    let mut lock = self.stdout.lock();
                    let char = self.state.memory[self.state.mem_ptr] as char;
                    write!(lock, "{}", char).unwrap();

                    self.state.prog_ptr += 1;
                }
                _ => {
                    self.state.prog_ptr += 1;
                }
            }
        }

        Ok(())
    }
}
