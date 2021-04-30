use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum BrokenReason {
    BadLeftString,
    NoRightString,
    BadRightString {
        right_string: String,
    },
    Duplicated {
        another_left_string: String,
    },
}

#[derive(Debug)]
pub enum ReadError {
    IOError(io::Error),
    Broken {
        line: usize,
        left_string: String,
        reason: BrokenReason,
    },
}

impl From<io::Error> for ReadError {
    #[inline]
    fn from(error: io::Error) -> Self {
        ReadError::IOError(error)
    }
}

impl Display for ReadError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ReadError::IOError(err) => Display::fmt(&err, f),
            ReadError::Broken {
                line,
                left_string,
                reason,
            } => {
                f.write_fmt(format_args!("broken at line {}, ", line))?;

                match reason {
                    BrokenReason::BadLeftString => {
                        f.write_fmt(format_args!(
                            "the left string {:?} is not correct",
                            left_string
                        ))
                    }
                    BrokenReason::NoRightString => {
                        f.write_fmt(format_args!(
                        "expected a \"=\" after the left string {:?} to concatenate a right string",
                        left_string
                    ))
                    }
                    BrokenReason::BadRightString {
                        right_string,
                    } => {
                        f.write_fmt(format_args!(
                            "the right string {:?} is not correct",
                            right_string
                        ))
                    }
                    BrokenReason::Duplicated {
                        another_left_string,
                    } => {
                        if left_string == another_left_string {
                            f.write_fmt(format_args!(
                                "the left string {:#?} is duplicated",
                                left_string
                            ))
                        } else {
                            f.write_fmt(format_args!(
                                "the left string {:#?} and {:#?} are duplicated",
                                left_string, another_left_string
                            ))
                        }
                    }
                }
            }
        }
    }
}

impl Error for ReadError {}

#[derive(Debug)]
pub enum WriteError {
    IOError(io::Error),
    BadLeftString,
    BadRightString,
    Duplicated,
    Same,
}

impl From<io::Error> for WriteError {
    #[inline]
    fn from(error: io::Error) -> Self {
        WriteError::IOError(error)
    }
}

impl Display for WriteError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            WriteError::IOError(err) => Display::fmt(&err, f),
            WriteError::BadLeftString => f.write_str("the left word is not correct"),
            WriteError::BadRightString => f.write_str("the right word is not correct"),
            WriteError::Duplicated => {
                f.write_str("the pair of the left word and the right word is duplicated")
            }
            WriteError::Same => f.write_str("the left word is equal to the right word"),
        }
    }
}

impl Error for WriteError {}
