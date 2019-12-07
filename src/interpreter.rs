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
            Parameter::Position(_) => Ok(p),
            Parameter::Immediate(_) => Err(Error::InvalidParameterMode {
                mode,
                parameter: n,
                opcode,
                position: *i,
            }),
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

    fn arithmetic(
        code: &[isize],
        i: &mut usize,
        opcode: isize,
        modes_and_opcode: isize,
    ) -> Result<(Self, Self, Self), Error> {
        let modes = (
            modes_and_opcode / 100 % 10,
            modes_and_opcode / 1000 % 10,
            modes_and_opcode / 10000 % 10,
        );

        let n1 = Self::from_code(code, i, modes.0, 0, opcode)?;
        let n2 = Self::from_code(code, i, modes.1, 1, opcode)?;
        let to = Self::positional_from_code(code, i, modes.2, 2, opcode)?;

        Ok((n1, n2, to))
    }

    fn jump(
        code: &[isize],
        i: &mut usize,
        opcode: isize,
        modes_and_opcode: isize,
    ) -> Result<(Self, Self), Error> {
        let modes = (modes_and_opcode / 100 % 10, modes_and_opcode / 1000 % 10);

        let test = Self::from_code(code, i, modes.0, 0, opcode)?;
        let goto = Self::from_code(code, i, modes.1, 1, opcode)?;

        Ok((test, goto))
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
    Input {
        to: Parameter,
    },
    Output {
        from: Parameter,
    },
    JumpIfTrue {
        test: Parameter,
        goto: Parameter,
    },
    JumpIfFalse {
        test: Parameter,
        goto: Parameter,
    },
    LessThan {
        n1: Parameter,
        n2: Parameter,
        to: Parameter,
    },
    Equals {
        n1: Parameter,
        n2: Parameter,
        to: Parameter,
    },
    Halt,
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
        match opcode {
            1 => {
                let (n1, n2, to) = Parameter::arithmetic(code, i, opcode, modes_and_opcode)?;
                Ok(Instruction::Add { n1, n2, to })
            }
            2 => {
                let (n1, n2, to) = Parameter::arithmetic(code, i, opcode, modes_and_opcode)?;
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
            5 => {
                let (test, goto) = Parameter::jump(code, i, opcode, modes_and_opcode)?;
                Ok(Instruction::JumpIfTrue { test, goto })
            }
            6 => {
                let (test, goto) = Parameter::jump(code, i, opcode, modes_and_opcode)?;
                Ok(Instruction::JumpIfFalse { test, goto })
            }
            7 => {
                let (n1, n2, to) = Parameter::arithmetic(code, i, opcode, modes_and_opcode)?;
                Ok(Instruction::LessThan { n1, n2, to })
            }
            8 => {
                let (n1, n2, to) = Parameter::arithmetic(code, i, opcode, modes_and_opcode)?;
                Ok(Instruction::Equals { n1, n2, to })
            }
            99 => Ok(Instruction::Halt),
            _ => Err(Error::InvalidOpcode {
                opcode,
                position: *i,
            }),
        }
    }
}

#[derive(Debug)]
pub struct RunResults {
    pub output: Vec<isize>,
    pub run_code: usize,
    pub used_input: usize,
}

fn add(code: &mut [isize], n1: Parameter, n2: Parameter, to: Parameter) {
    let n1 = n1.value(&code);
    let n2 = n2.value(&code);
    let to = to.index().unwrap();
    code[to] = n1 + n2;
}

fn multiply(code: &mut [isize], n1: Parameter, n2: Parameter, to: Parameter) {
    let n1 = n1.value(&code);
    let n2 = n2.value(&code);
    let to = to.index().unwrap();
    code[to] = n1 * n2;
}

fn jump_if_true(
    code: &mut [isize],
    i: &mut usize,
    test: Parameter,
    goto: Parameter,
) -> Result<(), Error> {
    let test = test.value(&code);
    if test != 0 {
        let goto = goto.value(&code);
        let goto = goto
            .try_into()
            .map_err(|_| Error::NegativePositionalParameter {
                value: goto,
                parameter: 1,
                opcode: 5,
                position: *i,
            })?;
        *i = goto;
    }
    Ok(())
}

fn jump_if_false(
    code: &mut [isize],
    i: &mut usize,
    test: Parameter,
    goto: Parameter,
) -> Result<(), Error> {
    let test = test.value(&code);
    if test == 0 {
        let goto = goto.value(&code);
        let goto = goto
            .try_into()
            .map_err(|_| Error::NegativePositionalParameter {
                value: goto,
                parameter: 1,
                opcode: 5,
                position: *i,
            })?;
        *i = goto;
    }
    Ok(())
}

fn less_than(code: &mut [isize], n1: Parameter, n2: Parameter, to: Parameter) {
    let n1 = n1.value(&code);
    let n2 = n2.value(&code);
    let to = to.index().unwrap();
    if n1 < n2 {
        code[to] = 1;
    } else {
        code[to] = 0;
    }
}

fn euqals(code: &mut [isize], n1: Parameter, n2: Parameter, to: Parameter) {
    let n1 = n1.value(&code);
    let n2 = n2.value(&code);
    let to = to.index().unwrap();
    if n1 == n2 {
        code[to] = 1;
    } else {
        code[to] = 0;
    }
}

pub fn eval(mut code: Vec<isize>, input: Vec<isize>) -> Result<RunResults, Error> {
    let mut output = Vec::new();

    let mut i = 0;
    let mut j = 0;
    loop {
        let instruction = Instruction::from_code(&code, &mut i)?;
        match instruction {
            Instruction::Add { n1, n2, to } => add(&mut code, n1, n2, to),
            Instruction::Multiply { n1, n2, to } => multiply(&mut code, n1, n2, to),
            Instruction::Input { to } => match input.get(j) {
                None => break,
                Some(i) => {
                    j += 1;
                    let to = to.index().unwrap();
                    code[to] = *i
                }
            },
            Instruction::Output { from } => {
                let from = from.value(&code);
                output.push(from);
            }
            Instruction::JumpIfTrue { test, goto } => jump_if_true(&mut code, &mut i, test, goto)?,
            Instruction::JumpIfFalse { test, goto } => {
                jump_if_false(&mut code, &mut i, test, goto)?
            }
            Instruction::LessThan { n1, n2, to } => less_than(&mut code, n1, n2, to),
            Instruction::Equals { n1, n2, to } => euqals(&mut code, n1, n2, to),
            Instruction::Halt => break,
            Instruction::End => break,
        }
    }

    Ok(RunResults {
        output,
        run_code: i,
        used_input: j,
    })
}
