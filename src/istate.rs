use crate::prod::EarleyProd;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct IState {
    pub origin: usize,
    pub prod: EarleyProd,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct FlippedIState {
    pub end: usize,
    pub prod: EarleyProd,
}

impl FlippedIState {
    pub fn new(prod: EarleyProd, end: usize) -> FlippedIState {
        FlippedIState { prod, end }
    }
}

impl IState {
    pub fn new(prod: EarleyProd, origin: usize) -> IState {
        IState { prod, origin }
    }
}

impl fmt::Display for IState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let terms: String = self
            .prod
            .rhs
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if i == self.prod.dot {
                    format!("{}{:#}", "•", t)
                } else if i + 1 == self.prod.rhs.len() && self.prod.dot == self.prod.rhs.len() {
                    format!("{:#}{} ", t, "•")
                } else {
                    format!("{:#} ", t)
                }
            })
            .collect();

        write!(f, "[{}] {} := {}", self.origin, self.prod.lhs, terms)
    }
}

impl fmt::Display for FlippedIState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let terms: String = self
            .prod
            .rhs
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if i == self.prod.dot {
                    format!("{}{:#}", "•", t)
                } else if i + 1 == self.prod.rhs.len() && self.prod.dot == self.prod.rhs.len() {
                    format!("{:#}{} ", t, "•")
                } else {
                    format!("{:#} ", t)
                }
            })
            .collect();

        write!(f, "{} := {} ({})", self.prod.lhs, terms, self.end)
    }
}
