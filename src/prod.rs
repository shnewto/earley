use bnf::Term;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyProd {
    pub lhs: Term,
    pub rhs: Vec<Term>,
    pub dot: usize,
}

impl EarleyProd {
    pub fn new(lhs: Term, rhs: Vec<Term>, dot: usize) -> EarleyProd {
        EarleyProd { lhs, rhs, dot }
    }
    pub fn get_next(&self) -> Option<&Term> {
        self.rhs.get(self.dot)
    }
}

impl fmt::Display for EarleyProd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} := {}",
            self.lhs,
            self.rhs.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" ")
        )
    }
}
