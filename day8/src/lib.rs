use displaydoc::Display;
use itertools::Itertools as _;
use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Errors {
    /// Parse error {0}
    ParseError(#[from] std::num::ParseIntError),
    /// Bad regex {0}
    BadRegex(#[from] regex::Error),
    /// "{0}" is not a valid instruction
    InvalidInstruction(String),
    /// Out of bounds, "{0}" is not in program memory
    InstructionOutOfBounds(usize),
    /// Out of bounds, could not make negative program memory
    InstructionUnderflow,
    /// Infinite loop
    InfiniteLoop(ProgramState),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    /// Do nothing (nop)
    NoOp(isize),
    /// Increment accumulator (acc)
    Accumulator(isize),
    /// Relative instruction jump
    Jump(isize),
}

#[derive(Debug, Default)]
pub struct ProgramState {
    accumulator: isize,
    instruction_ptr: usize,
    visited: HashSet<usize>,
}

impl ProgramState {
    fn run_one_instruction(mut self, program: &Vec<Instruction>) -> Result<Self, Errors> {
        let instruction = program
            .get(self.instruction_ptr)
            .ok_or(Errors::InstructionOutOfBounds(self.instruction_ptr))?;

        if !self.visited.insert(self.instruction_ptr) {
            return Err(Errors::InfiniteLoop(self));
        }

        match instruction {
            Instruction::NoOp(..) => self.instruction_ptr += 1,
            Instruction::Accumulator(arg) => {
                self.instruction_ptr += 1;
                self.accumulator += arg
            }
            Instruction::Jump(arg) => {
                if *arg < 0 {
                    if -arg as usize > self.instruction_ptr {
                        return Err(Errors::InstructionUnderflow);
                    }
                    self.instruction_ptr -= -arg as usize
                } else {
                    self.instruction_ptr += *arg as usize
                }
            }
        }

        Ok(self)
    }

    pub fn run(mut self, program: &Vec<Instruction>) -> Result<Self, Errors> {
        loop {
            self = self.run_one_instruction(program)?;
            if self.instruction_ptr == program.len() {
                break;
            }
        }
        Ok(self)
    }
}

impl FromStr for Instruction {
    type Err = Errors;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() < 5 {
            return Err(Errors::InvalidInstruction(value.to_string()));
        }
        let operation = &value[0..3];
        let argument = value[4..].parse()?;

        Ok(match operation {
            "acc" => Instruction::Accumulator(argument),
            "jmp" => Instruction::Jump(argument),
            "nop" => Instruction::NoOp(argument),
            _ => return Err(Errors::InvalidInstruction(value.to_string())),
        })
    }
}

pub fn challenge1(input: &str) -> Result<isize, Errors> {
    let program = input.lines().map(str::parse).try_collect()?;

    let state = match ProgramState::default().run(&program) {
        Err(Errors::InfiniteLoop(state)) => state,
        Err(e) => return Err(e),
        Ok(state) => state,
    };

    Ok(state.accumulator)
}

pub fn challenge2(input: &str) -> Result<isize, Errors> {
    let mut program: Vec<Instruction> = input.lines().map(str::parse).try_collect()?;

    for i in 0..program.len() {
        let flip = match &program[i] {
            Instruction::Jump(arg) => Instruction::NoOp(*arg),
            Instruction::NoOp(arg) => Instruction::Jump(*arg),
            _ => continue,
        };

        let previous_op = std::mem::replace(&mut program[i], flip);
        if let Ok(state) = ProgramState::default().run(&program) {
            return Ok(state.accumulator);
        }
        program[i] = previous_op;
    }

    return Ok(0);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn test_challenge1() -> Result<(), super::Errors> {
        assert_eq!(super::challenge1(INPUT)?, 5);
        Ok(())
    }

    #[test]
    fn test_challenge2() -> Result<(), super::Errors> {
        assert_eq!(super::challenge2(INPUT)?, 8);
        Ok(())
    }
}
