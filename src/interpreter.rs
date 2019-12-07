use crate::error::Error;
use std::convert::TryInto;

enum Parameter {
    Position(usize),
    Immediate(isize),
}

impl Parameter {
    fn from_code(
        code: &[isize],
        i: &mut usize,
        mode: isize,
        n: u8,
        opcode: isize,
    ) -> Result<Self, Error> {
        match code.get(*i) {
            None => Err(Error::MissingParameter {
                parameter: n,
                opcode,
                position: *i,
            }),
            Some(p) => {
                *i += 1;
                match mode {
                    0 => Ok(Parameter::Position((*p).try_into().map_err(|_| {
                        Error::NegativePositionalParameter {
                            value: *p,
                            parameter: n,
                            opcode,
                            position: *i,
                        }
                    })?)),
                    1 => Ok(Parameter::Immediate(*p)),
                    _ => Err(Error::InvalidParameterMode {
                        mode,
                        parameter: n,
                        opcode,
                        position: *i,
                    }),
                }
            }
        }
    }

    fn positional_from_code(
        code: &[isize],
        i: &mut usize,
        mode: isize,
        n: u8,
        opcode: isize,
    ) -> Result<Self, Error> {
        let p = Self::from_code(code, i, mode, n, opcode)?;
        match p {
            Parameter::Position(_) => Err(Error::InvalidParameterMode {
                mode,
                parameter: n,
                opcode,
                position: *i,
            }),
            Parameter::Immediate(_) => Ok(p),
        }
    }

    fn value(&self, code: &[isize]) -> isize {
        match self {
            Parameter::Position(p) => code[*p],
            Parameter::Immediate(v) => *v,
        }
    }

    fn index(&self) -> Option<usize> {
        match self {
            Parameter::Position(p) => Some(*p),
            Parameter::Immediate(_) => None,
        }
    }
}

enum Instruction {
    Add {
        n1: Parameter,
        n2: Parameter,
        to: Parameter,
    },
    Multiply {
        n1: Parameter,
        n2: Parameter,
        to: Parameter,
    },
    Halt,
    Input {
        to: Parameter,
    },
    Output {
        from: Parameter,
    },
    End,
}

impl Instruction {
    fn from_code(code: &[isize], i: &mut usize) -> Result<Self, Error> {
        let modes_and_opcode = match code.get(*i) {
            None => return Ok(Instruction::End),
            Some(n) => {
                *i += 1;
                *n
            }
        };

        let opcode = modes_and_opcode % 100;

        let mut parse_math_parameters = || {
            let modes = (
                modes_and_opcode / 100 % 10,
                modes_and_opcode / 1000 % 10,
                modes_and_opcode / 10000 % 10,
            );

            let n1 = Parameter::from_code(code, i, modes.0, 0, opcode)?;
            let n2 = Parameter::from_code(code, i, modes.1, 1, opcode)?;
            let to = Parameter::positional_from_code(code, i, modes.2, 2, opcode)?;

            Ok((n1, n2, to))
        };

        match opcode {
            1 => {
                let (n1, n2, to) = parse_math_parameters()?;
                Ok(Instruction::Add { n1, n2, to })
            }
            2 => {
                let (n1, n2, to) = parse_math_parameters()?;
                Ok(Instruction::Multiply { n1, n2, to })
            }
            3 => {
                let mode = modes_and_opcode / 100 % 10;
                let to = Parameter::positional_from_code(code, i, mode, 0, opcode)?;
                Ok(Instruction::Input { to })
            }
            4 => {
                let mode = modes_and_opcode / 100 % 10;
                let from = Parameter::from_code(code, i, mode, 0, opcode)?;
                Ok(Instruction::Output { from })
            }
            99 => Ok(Instruction::Halt),
            _ => Err(Error::InvalidOpcode {
                opcode,
                position: *i,
            }),
        }
    }
}

pub struct RunResults {
    pub output: Vec<isize>,
    pub run_code: usize,
    pub used_input: usize,
}

pub fn run(mut code: Vec<isize>, input: Vec<isize>) -> Result<RunResults, Error> {
    let mut output = Vec::new();

    let mut i = 0;
    let mut j = 0;
    loop {
        let instruction = Instruction::from_code(&code, &mut i)?;
        match instruction {
            Instruction::Add { n1, n2, to } => {
                let n1 = n1.value(&code);
                let n2 = n2.value(&code);
                code[to.index().unwrap()] = n1 + n2;
            }
            Instruction::Multiply { n1, n2, to } => {
                let n1 = n1.value(&code);
                let n2 = n2.value(&code);
                code[to.index().unwrap()] = n1 * n2;
            }
            Instruction::Halt => break,
            Instruction::Input { to } => match input.get(j) {
                None => break,
                Some(i) => {
                    j += 1;
                    code[to.index().unwrap()] = *i
                }
            },
            Instruction::Output { from } => {
                let from = from.value(&code);
                output.push(from);
            }
            Instruction::End => break,
        }
    }

    Ok(RunResults {
        output,
        run_code: i,
        used_input: j,
    })
}
