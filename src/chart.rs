use crate::earley::EarleyParser;
use crate::error::Error;
use crate::outcome::EarleyOutcome;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyChart;

impl EarleyChart {
    pub fn eval(grammar: &str, input: &str) -> Result<EarleyOutcome, Error> {
        let parser = EarleyParser::new(grammar, input)?;
        let outcome = parser.earley_parse()?;
        Ok(outcome)
    }

    pub fn accept(grammar: &str, input: &str) -> Result<bool, Error> {
        let parser = EarleyParser::new(grammar, input)?;
        let res = parser.earley_parse()?;
        if let EarleyOutcome::Accepted(_) = res {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
