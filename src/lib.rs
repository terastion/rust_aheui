pub mod hangul;
pub mod instruction;
pub mod component;

use std::io;
use std::io::{BufRead, BufWriter, Write};
use crate::instruction::*;
use crate::component::*;


#[derive(Debug)]
pub struct AheuiState {
    pub program: AheuiProgram,
    pub terminated: bool,
    pub storages: Vec<AheuiStorage>,
    pub storage_index: usize,
    pub position: AheuiCoordinates,
    pub direction: AheuiDirection,
    input: io::Stdin,
    output: BufWriter<io::Stdout>,
}

impl AheuiState {
    pub fn init(prog: &str) -> Self {
        // convert input string into AheuiProgram
        let program = AheuiProgram::from_str(prog);
        let terminated = false;

        // initialize the 28 storages
        // 21 is ㅇ and 27 is ㅎ
        // extension protocol to be treated as queue
        let mut storages: Vec<AheuiStorage> = Vec::new();
        for i in 0..28 {
            storages.push(AheuiStorage::new(i == 21 || i == 27));
        }

        // initialize storage index to 0 (null coda)
        let storage_index = 0;

        // initialize position to row 0 col 0
        let position = AheuiCoordinates::zero();

        // initialize direction to AheuiDirection::Down(false) (0,1)
        let direction = AheuiDirection::Down(false);

        // initialize buffered output to stdout
        let stdout = io::stdout();
        let output = BufWriter::new(stdout);

        Self {
            program,
            terminated,
            storages,
            storage_index,
            position,
            direction,
            input: io::stdin(),
            output,
        }
    }

    // Update current position based on current direction and new direction
    pub fn step_coordinate(&mut self, new_direction: AheuiDirection) {
        let mut final_direction = self.direction;

        // either reflect the program's current direction
        // or update it to the current instruction's direction
        // or do nothing (in case of null instruction)
        match new_direction {
            AheuiDirection::Null => {},
            AheuiDirection::ReflectX => final_direction.reflect_x(),
            AheuiDirection::ReflectY => final_direction.reflect_y(),
            AheuiDirection::ReflectXY => final_direction.reflect_xy(),
            _ => final_direction = new_direction,
        };

        // update position based on final_direction
        match final_direction {
            AheuiDirection::Left(b) => {
                let movement = 1 + b as usize;
                // in case movement goes beyond left boundary
                if self.position.x < movement {
                    // wrap around to right side of program
                    self.position.x = self.program.size.x - (movement - self.position.x);
                } else {
                    // otherwise just subtract movement
                    self.position.x -= movement;
                }
            },
            AheuiDirection::Right(b) => {
                let movement = 1 + b as usize;
                // in case movement goes beyond right boundary
                if self.position.x >= self.program.size.x - movement {
                    // wrap around to right side of program
                    self.position.x = movement - (self.program.size.x - self.position.x);
                } else {
                    // otherwise just add movement
                    self.position.x += movement;
                }
            },
            AheuiDirection::Up(b) => {
                let movement = 1 + b as usize;
                // in case movement goes beyond upper boundary
                if self.position.y < movement {
                    // wrap around to bottom side of program
                    self.position.y = self.program.size.y - (movement - self.position.y);
                } else {
                    // otherwise just subtract movement
                    self.position.y -= movement;
                }
            },
            AheuiDirection::Down(b) => {
                let movement = 1 + b as usize;
                // in case movement goes beyond bottom boundary
                if self.position.y >= self.program.size.y - movement {
                    // wrap around to upper side of program
                    self.position.y = movement - (self.program.size.y - self.position.y);
                } else {
                    // otherwise just add movement
                    self.position.y += movement;
                }
            }
            _ => {},
        };

        self.direction = final_direction;
    }

    pub fn step(&mut self) -> Result<(), AheuiError> {
        // return an error if program has terminated
        if self.terminated {
            return Err(AheuiError::TerminatedError);
        }
        // get current instruction, or terminate if failed
        let instruction = match self.program.get_instruction(&self.position) {
            Some(op) => op,
            None => {
                self.terminated = true;
                //self.output.flush().unwrap();
                return Err(AheuiError::InstructionNotFoundError);
            },
        };

        // keep track of success operation and current storage
        let mut success = false;
        match instruction.operation {
            AheuiOperation::Null => success = true,
            AheuiOperation::Terminate => {
                // terminate program, and flush output
                self.terminated = true;
                match write!(self.output, "\n") {
                    Ok(()) => {},
                    Err(_) => {},
                };
                //self.output.flush().unwrap();
                return Ok(());
            },
            AheuiOperation::Add |
            AheuiOperation::Multiply |
            AheuiOperation::Divide |
            AheuiOperation::Subtract |
            AheuiOperation::Modulo => {
                // check for at least two elements in current storage
                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                if current_storage.len() >= 2 {
                    // pop two values, perform arithmetic operation, and push result
                    let a = current_storage.pop().unwrap();
                    let b = current_storage.pop().unwrap();
                    let a_b = instruction.operation
                        .arithmetic_operation(b,a)
                        .ok_or_else(|| {
                            self.terminated = true;
                            //self.output.flush().unwrap();
                            AheuiError::ArithmeticError(a,b)
                        })?;
                    current_storage.push(a_b);
                    success = true;
                }
            },
            AheuiOperation::Pop => {
                // attempt to pop a value from the storage
                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                if let Some(num) = current_storage.pop() {
                    match instruction.argument {
                        AheuiArgument::AsInt => {
                            // convert number to string representation then output
                            let num_string = num.to_string();
                            match self.output.write_all(&num_string.into_bytes()) {
                                Ok(()) => success = true,
                                Err(e) => return Err(AheuiError::OutputError(e)),
                            };
                        },
                        AheuiArgument::AsChar => {
                            // attempt to convert number to char
                            let num_char = char::from_u32(num as u32)
                                .ok_or_else(|| {
                                    self.terminated = true;
                                    //self.output.flush().unwrap();
                                    AheuiError::InvalidCharError(num as u32)
                                })?;

                            // then convert char to byte array then output
                            let mut char_array = [0; 4];
                            num_char.encode_utf8(&mut char_array);
                            match self.output.write_all(&char_array) {
                                Ok(()) => success = true,
                                Err(e) => return Err(AheuiError::OutputError(e)),
                            };
                        },
                        _ => {},
                    };
                }
            },
            AheuiOperation::Push => {
                // push argument into storage
                match instruction.argument {
                    AheuiArgument::Number(n) => {
                        // push a number specified by argument/coda
                        let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                        current_storage.push(n as isize);
                        success = true;
                    },
                    AheuiArgument::AsInt => {
                        // read a number from stdin
                        let mut buffer = String::new();
                        let mut stdin_handle = self.input.lock();

                        // prompt user for input...
                        print!("\ninput number: ");
                        self.output.flush().expect("could not flush stdout");

                        match stdin_handle.read_line(&mut buffer) {
                            Ok(_) => {
                                // ... then attempt conversion into an isize and push
                                let num: isize = buffer
                                    .trim()
                                    .parse()
                                    .map_err(|_| AheuiError::InvalidNumberError(buffer.clone()))?;
                                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                                current_storage.push(num);
                                success = true;
                            },
                            Err(e) => return Err(AheuiError::InputError(e)),
                        };
                    },
                    AheuiArgument::AsChar => {
                        // read string from stdin
                        let mut buffer = String::new();
                        let mut stdin_handle = self.input.lock();

                        // prompt user for input
                        print!("\ninput character: ");
                        self.output.flush().expect("could not flush stdout");

                        match stdin_handle.read_line(&mut buffer) {
                            Ok(_) => {
                                // then get first char in input
                                let first_char = buffer
                                    .trim()
                                    .chars()
                                    .nth(0)
                                    .ok_or_else(|| AheuiError::EmptyInputError)?;
                                let num = first_char as isize;
                                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                                current_storage.push(num);
                                success = true;
                            },
                            Err(e) => return Err(AheuiError::InputError(e)),
                        }
                    },
                    _ => {},
                };
            },
            AheuiOperation::Duplicate => {
                // duplicate the first element in storage
                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                if let Ok(_) = current_storage.duplicate() {
                    success = true;
                };
            },
            AheuiOperation::Swap => {
                // swap the top two values in storage
                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                if let Ok(_) = current_storage.swap() {
                    success = true;
                };
            },
            AheuiOperation::StoreSelect => {
                // set storage_index to argument number
                if let AheuiArgument::Storage(n) = instruction.argument {
                    self.storage_index = n;
                    success = true;
                };
            },
            AheuiOperation::StoreTransfer => {
                // pop a value from current storage, then push to specified storage in arg
                if let AheuiArgument::Storage(dest) = instruction.argument {
                    // attempt to pop from current storage
                    let current_storage_val = {
                        let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                        current_storage.pop()
                    };
                    
                    // if pop successful, push to dest storage in argument
                    if let Some(current_num) = current_storage_val {
                        let dest_storage = self.storages.get_mut(dest).unwrap();
                        dest_storage.push(current_num);
                        success = true;
                    };
                };
            },
            AheuiOperation::Compare => {
                // pop two values from current storage, 
                // then push 1 or 0 to storage based on comparison
                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                if current_storage.len() >= 2 {
                    let value1 = current_storage.pop().unwrap();
                    let value2 = current_storage.pop().unwrap();
                    current_storage.push((value1 <= value2) as isize);
                }
            },

            AheuiOperation::Fork => {
                // pop a number from storage,
                // then mark op as successful if number is nonzero
                let current_storage = self.storages.get_mut(self.storage_index).unwrap();
                if let Some(num) = current_storage.pop() {
                    success = num != 0;
                };
            },
        };

        if success {
            // step based on instruction's direction if command successful
            self.step_coordinate(instruction.direction);
        } else {
            // otherwise, reflect both current and instruction's directions and step
            self.direction.reflect_xy();
            let mut inst_direction = instruction.direction;
            inst_direction.reflect_xy();
            self.step_coordinate(inst_direction);
        }

        Ok(())
    }

    pub fn run(&mut self) {
        // run until terminated, printing any errors encountered
        while !self.terminated {
            if let Err(err) = self.step() {
                self.output.flush().expect("Could not flush stdout");
                eprintln!("{err}");
            }
        }
    }
}
