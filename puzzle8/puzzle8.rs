use bitvec::prelude::*;
use std::convert;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    NOP(i16),
    ACC(i16),
    JMP(i16),
}

#[derive(Debug)]
enum VMError {
    InvalidOpcode(String),
    InvalidParameter(String),
    InvalidJump,
    PastTheEnd,
    InfiniteLoop,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            VMError::InvalidOpcode(opcode) => write!(f, "Unknown opcode {}", opcode),
            VMError::InvalidParameter(param) => {
                write!(f, "Parameter {} not a 16-bit integer", param)
            }
            VMError::InvalidJump => write!(f, "Negative jump overflow"),
            VMError::PastTheEnd => write!(f, "Positive jump overflow"),
            VMError::InfiniteLoop => write!(f, "Infinite loop detected"),
        }
    }
}

impl Error for VMError {}

impl convert::From<VMError> for io::Error {
    fn from(err: VMError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

struct VM {
    acc: i32,
    pc: usize,
    code: Vec<Instruction>,
    visited: BitVec,
}

impl VM {
    fn new() -> Self {
        VM {
            acc: 0,
            pc: 0,
            code: vec![],
            visited: BitVec::new(),
        }
    }

    fn assemble_line(&mut self, line: &str) -> Result<(), VMError> {
        let opcode_string = &line[..3];
        let parameter_string = &line[4..];
        let parameter = parameter_string
            .parse::<i16>()
            .map_err(|_| VMError::InvalidParameter(parameter_string.to_string()))?;
        let instruction = match opcode_string {
            "nop" => Instruction::NOP(parameter),
            "acc" => Instruction::ACC(parameter),
            "jmp" => Instruction::JMP(parameter),
            _ => return Err(VMError::InvalidOpcode(opcode_string.to_string())),
        };
        self.code.push(instruction);
        self.visited.push(false);
        Ok(())
    }

    fn run(&mut self) -> Result<(), VMError> {
        self.pc = 0;
        self.acc = 0;
        self.visited.set_all(false);
        let code_length = self.code.len();
        while self.pc < code_length {
            if self.visited[self.pc] {
                return Err(VMError::InfiniteLoop);
            }
            self.visited.set(self.pc, true);
            match self.code[self.pc] {
                Instruction::NOP(_) => self.pc += 1,
                Instruction::ACC(value) => {
                    self.acc += value as i32;
                    self.pc += 1;
                }
                Instruction::JMP(distance) => {
                    self.pc = checked_jump(self.pc, distance).ok_or(VMError::InvalidJump)?
                }
            };
        }
        if self.pc != code_length {
            Err(VMError::PastTheEnd)
        } else {
            Ok(())
        }
    }

    fn repair_instruction(&mut self, pc: usize) -> bool {
        match self.code[pc] {
            Instruction::NOP(param) => {
                self.code[pc] = Instruction::JMP(param);
                true
            }
            Instruction::ACC(_) => false,
            Instruction::JMP(param) => {
                self.code[pc] = Instruction::NOP(param);
                true
            }
        }
    }
}

// https://stackoverflow.com/a/54035801/172999
fn checked_jump(pc: usize, jump: i16) -> Option<usize> {
    if jump.is_negative() {
        pc.checked_sub(jump.wrapping_abs() as u16 as usize)
    } else {
        pc.checked_add(jump as usize)
    }
}

fn main() -> Result<(), io::Error> {
    let file = fs::File::open("input")?;
    let mut vm = VM::new();
    for line in read_lines(file) {
        vm.assemble_line(&line)?;
    }

    if is_part2() {
        for pc in 0..vm.code.len() {
            if !vm.repair_instruction(pc) {
                continue;
            }
            if vm.run().is_ok() {
                break;
            }
            vm.repair_instruction(pc);
        }
        println!("{}", vm.acc);
    } else {
        let err = vm.run().unwrap_err();
        match err {
            VMError::InfiniteLoop => println!("{}", vm.acc),
            _ => return Err(err.into()),
        }
    }

    Ok(())
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

fn read_lines(file: fs::File) -> impl Iterator<Item = String> {
    io::BufReader::new(file).lines().map(|res| res.unwrap())
}
