use crate::prod::EarleyProd;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;

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
        let mut terms = self
            .prod
            .rhs
            .iter()
            .map(|t| format!("{:#}", t))
            .collect::<Vec<String>>();
        terms.insert(self.prod.dot, "•".to_string());
        write!(
            f,
            "[{}] {} := {} ({})",
            self.origin,
            self.prod.lhs,
            terms.join(""),
            self.prod.dot
        )
    }
}

impl fmt::Display for FlippedIState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut terms = self
            .prod
            .rhs
            .iter()
            .map(|t| format!("{:#}", t))
            .collect::<Vec<String>>();
        terms.insert(self.prod.dot, "•".to_string());
        write!(f, "{} := {} ({})", self.prod.lhs, terms.join(""), self.end)
    }
}
