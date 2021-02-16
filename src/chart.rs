use crate::earley::EarleyParser;
use crate::error::Error;
use crate::outcome::EarleyOutcome;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyChart;

impl EarleyChart {
    pub fn eval(
        grammar: &str,
        input: &str,
        split_on: Option<char>,
    ) -> Result<EarleyOutcome, Error> {
        let parser = EarleyParser::new(grammar, input)?;
        let outcome = parser.earley_parse(split_on)?;
        Ok(outcome)
    }

    pub fn accept(grammar: &str, input: &str, split_on: Option<char>) -> Result<bool, Error> {
        let parser = EarleyParser::new(grammar, input)?;
        let res = parser.earley_parse(split_on)?;
        if let EarleyOutcome::Accepted(_) = res {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
