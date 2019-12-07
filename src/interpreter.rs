use crate::error::Error;
use std::{convert::TryFrom, iter::Enumerate, slice::Iter};

enum Parameter {
    Position(usize),
    Immediate(isize),
}

enum Instruction {
    Add {
        n1: Parameter,
        n2: Parameter,
        to: Parameter::Position,
    },
    Multiply {
        n1: Parameter,
        n2: Parameter,
        to: Parameter::Position,
    },
    Halt,
    Input {
        to: Parameter::Position,
    },
    Output {
        from: Parameter,
    },
    End,
}

impl TryFrom<&mut Enumerate<Iter<isize>>> for Instruction {
    type Error = Error;

    fn try_from(value: &mut Enumerate<Iter<isize>>) -> Result<Self, Self::Error> {
        let (position, modes_and_opcode) = match value.next() {
            None => return Ok(Instruction::End),
            Some(n) => n,
        };

        let opcode = *modes_and_opcode % 100;
        match opcode {
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            99 => {}
            _ => Err(Error::InvalidOpcode { opcode, position }),
        }
    }
}

pub struct RunResults {
    pub output: Vec<isize>,
    pub run_code: usize,
    pub used_input: usize,
}

pub fn run(mut code: Vec<isize>, input: Vec<isize>) -> Result<RunResults, Error> {
    let mut code_iter = code.iter().enumerate();
    let mut input = input.iter().enumerate();
    let mut output = Vec::new();

    loop {}
}
