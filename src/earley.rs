use bnf::Grammar;
use crate::error::Error;
use crate::error::Error::GrammarError;
use std::str::FromStr;
use std::fmt;

pub struct Parser {
    grammar: Grammar,
}

impl Parser {
    pub fn new(grammar: Grammar) -> Parser {
        Parser {
            grammar
        }
    }

    pub fn grammar(&self) -> Grammar {
        self.grammar.clone()
    }

    pub fn earley_parse(&mut self, sentence: String) {
        ()
    }

    fn predictor() {
        ()
    }

    fn scanner() {
        ()
    }

    fn completer() {
        ()
    }
}

impl FromStr for Parser {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Grammar::from_str(s) {
            Result::Ok(g) => Ok( Parser::new(g) ),
            Result::Err(e) => Err(Error::from(e)),
        }
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            self.grammar().to_string()
        )
    }
}