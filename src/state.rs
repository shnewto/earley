use crate::prod::EarleyProd;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntermediateState {
    pub origin: usize,
    pub prod: EarleyProd,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct FlippedIntermediateState {
    pub end: usize,
    pub prod: EarleyProd,
}

impl FlippedIntermediateState {
    pub fn new(prod: EarleyProd, end: usize) -> FlippedIntermediateState {
        FlippedIntermediateState { prod, end }
    }
}

impl IntermediateState {
    pub fn new(prod: EarleyProd, origin: usize) -> IntermediateState {
        IntermediateState { prod, origin }
    }
}

impl fmt::Display for IntermediateState {
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

impl fmt::Display for FlippedIntermediateState {
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
