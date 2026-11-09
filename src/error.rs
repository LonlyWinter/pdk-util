//! 错误处理

use std::{
    fmt::{Debug, Display, Error as FmtError},
    io::Error as IOError,
    num::{ParseFloatError, ParseIntError},
};

pub enum PDKError {
    UnImplemented(String),
    IO(String),
    Fmt(String),
    Empty(&'static str),
    Parse(String),
}

pub type PDKResult<T> = Result<T, PDKError>;

impl std::error::Error for PDKError {}

impl From<IOError> for PDKError {
    fn from(value: IOError) -> Self {
        Self::IO(value.to_string())
    }
}

impl From<FmtError> for PDKError {
    fn from(value: FmtError) -> Self {
        Self::Fmt(value.to_string())
    }
}

impl From<ParseIntError> for PDKError {
    fn from(value: ParseIntError) -> Self {
        Self::Parse(value.to_string())
    }
}

impl From<ParseFloatError> for PDKError {
    fn from(value: ParseFloatError) -> Self {
        Self::Parse(value.to_string())
    }
}

impl Display for PDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(v) => write!(f, "IO error, {}", v),
            Self::Fmt(v) => write!(f, "Fmt error, {}", v),
            Self::Parse(v) => write!(f, "Parse error, {}", v),
            Self::Empty(v) => write!(f, "{} empty", v),
            Self::UnImplemented(info) => write!(f, "UnImplemented {info:?}"),
        }
    }
}

impl Debug for PDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
