use std::collections::VecDeque;
use super::instruction::*;
use std::{io, fmt, error};

// enum for error handling
#[derive(Debug)]
pub enum AheuiError {
    StorageSizeError, // error when insufficient num of elements in storage for operation
    TerminatedError, // error when executing after termination
    InstructionNotFoundError, // error when 
    EmptyInputError,
    ArithmeticError(isize, isize), // error when arithmetic operation leads to (over/under)flow
    InputError(io::Error),
    OutputError(io::Error),
    InvalidCharError(u32),
    InvalidNumberError(String),
}

impl fmt::Display for AheuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            AheuiError::ArithmeticError(a,b) => format!("fatal: overflow/underflow occurred with arithmetic operation between {} and {}", a, b),
            AheuiError::InputError(err) => err.to_string(),
            AheuiError::OutputError(err) => err.to_string(),
            AheuiError::InvalidCharError(num) => format!("fatal: attempted to pop invalid UTF-8 value {} to output", num),
            AheuiError::InvalidNumberError(num) => format!("non-number input: {}", num),
            AheuiError::EmptyInputError => "no input provided".to_string(),
            _ => "".to_string(),
        };
        f.write_str(&error)
    }
}

impl error::Error for AheuiError {}

// struct for individual stack/queue
#[derive(Debug)]
pub struct AheuiStorage {
    storage: VecDeque<isize>,
    queue: bool,
}

impl AheuiStorage {
    // create a new storage unit
    pub fn new(queue: bool) -> Self {
        Self {
            storage: VecDeque::new(),
            queue,
        }
    }

    // push a value to the storage
    // push_back for stack,
    // push_front for queue
    pub fn push(&mut self, data: isize) {
        if self.queue {
            self.storage.push_back(data);
        } else {
            self.storage.push_front(data);
        }
    }

    // pop a value from storage
    pub fn pop(&mut self) -> Option<isize> {
        self.storage.pop_front()
    }

    // swap the top two values of storage
    // TODO: make use of AheuiError dropped in lib
    pub fn swap(&mut self) -> Result<(), AheuiError> {
        if self.len() < 2 {
            return Err(AheuiError::StorageSizeError);
        }

        let val1 = self.pop().unwrap();
        let val2 = self.pop().unwrap();
        self.storage.push_front(val1);
        self.storage.push_front(val2);

        Ok(())
    }

    // duplicate the first element in storage
    // TODO: make use of AheuiError dropped in lib
    pub fn duplicate(&mut self) -> Result<(), AheuiError> {
        let num = self.peek().ok_or(AheuiError::StorageSizeError)?;
        self.storage.push_front(*num);

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
    
    pub fn peek(&self) -> Option<&isize> {
        self.storage.get(0)
    }
}

// storage struct for aheui program coordinates
#[derive(Debug)]
pub struct AheuiCoordinates {
    pub x: usize,
    pub y: usize,
}

impl AheuiCoordinates {
    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

// struct for holding 2D table of AheuiInstructions
#[derive(Debug)]
pub struct AheuiProgram {
    pub program: Vec<Vec<AheuiInstruction>>,
    pub size: AheuiCoordinates
}

impl AheuiProgram {
    pub fn from_str(s: &str) -> Self {
        let mut program: Vec<Vec<AheuiInstruction>> = Vec::new();

        // track rows and columns of programs
        let mut rows = 0;
        let mut max_col_length = 0;

        // convert each line's characters into AheuiInstructions
        // and keep track of which line has the most characters
        for line in s.lines() {
            program.push(Vec::new());
            let current_row = program.last_mut().unwrap();

            // create an AheuiInstruction for each character in line
            for c in line.trim().chars() {
                let aheui_cmd = AheuiInstruction::from_char(c);
                current_row.push(aheui_cmd);
            }

            // update max_col_length accordingly
            let num_chars = line.trim().chars().count();
            if num_chars > max_col_length {
                max_col_length = num_chars;
            }

            rows += 1;
        }

        // iterate through each row of AheuiInstructions and
        // add empty AheuiInstructions until each row has max_col_length items
        for row in program.iter_mut() {
            while row.len() < max_col_length {
                row.push(AheuiInstruction::null());
            }
        }

        Self {
            program,
            size: AheuiCoordinates {
                x: max_col_length,
                y: rows,
            },
        }
    }

    pub fn get_instruction(&self, coords: &AheuiCoordinates) -> Option<&AheuiInstruction> {
        // attempt to get row
        let row = self.program.get(coords.y)?;

        // attempt to return instruction in row
        row.get(coords.x)
    }
}
