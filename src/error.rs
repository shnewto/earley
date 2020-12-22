use std::error;
use std::fmt;
use std::str;

use bnf::{
    Error::ParseError,
};

#[derive(PartialEq, Debug, Clone)]
pub enum Error {
    GrammarError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::GrammarError(ref s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Early error"
    }
}

impl<'a> From<bnf::Error> for Error {
    fn from(err: bnf::Error) -> Self {
        Error::GrammarError(format!("Error parsing input grammar: {:?}", err))
    }
}