use std::{
    error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Error {
    InvalidInput {
        token: String,
        position: usize,
    },
    InvalidOpcode {
        opcode: isize,
        position: usize,
    },
    MissingParameter {
        parameter: u8,
        opcode: isize,
        position: usize,
    },
    NegativePositionalParameter {
        value: isize,
        parameter: u8,
        opcode: isize,
        position: usize,
    },
    InvalidParameterMode {
        mode: isize,
        parameter: u8,
        opcode: isize,
        position: usize,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput { token, position } => {
                write!(f, "Invalid token \"{}\" at position {}", token, position)
            }
            Error::InvalidOpcode { opcode, position } => {
                write!(f, "Invalid opcode \"{}\" at position {}", opcode, position)
            }
            Error::MissingParameter {
                parameter,
                opcode,
                position,
            } => write!(
                f,
                "Missing parameter {} for opcode \"{}\" at position {}",
                parameter, opcode, position
            ),
            Error::NegativePositionalParameter {
                value,
                parameter,
                opcode,
                position,
            } => write!(
                f,
                "Negative value {} for positional parameter {} for opcode \"{}\" at position {}",
                value, parameter, opcode, position
            ),
            Error::InvalidParameterMode {
                mode,
                parameter,
                opcode,
                position,
            } => write!(
                f,
                "Invalid parameter mode \"{}\" for parameter {} of opcode \"{}\" at position {}",
                mode, parameter, opcode, position
            ),
        }
    }
}

impl error::Error for Error {}
