use crate::istate::FlippedIState;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ITree {
    pub root: FlippedIState,
    pub branches: Vec<IBranch>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum IBranch {
    Nonterminal(usize, ITree),
    Terminal(usize, String),
}

impl ITree {
    pub fn new(root: FlippedIState) -> ITree {
        ITree {
            root,
            branches: vec![],
        }
    }

    fn fmt(&self, prefix: String, depth: usize) -> String {
        let mut value: String;
        if depth == 0 {
            value = format!("{:>padding$}└─ {}\n", prefix, self.root.prod, padding=0);
        } else if self.branches.len() == 1 {
            value = format!("{:>padding$}└─ {}\n", prefix, self.root.prod, padding=depth);
        } else {
            value = format!("{:>padding$}├─ {}\n", prefix, self.root.prod, padding=depth);
        }

        for branch in &self.branches {
            value += &*branch.fmt(format!("\t{}", prefix), depth);
            // value = format!("{}{}", value, branch.fmt(format!("{}", value), depth + 1));
        }
        value
    }
}

impl IBranch {
    pub fn index(&self) -> usize {
        match self {
            IBranch::Nonterminal(index, _) => *index,
            IBranch::Terminal(index, _) => *index,
        }
    }
    fn fmt(&self, prefix: String, depth: usize) -> String {
        match self {
            IBranch::Nonterminal(_, t) => t.fmt(prefix, depth),
            IBranch::Terminal(_, s) =>  format!("{:>padding$}└─ {}\n", prefix, s, padding=depth),
        }
    }
}

impl fmt::Display for IBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt("".to_string(), 0))
    }
}

impl fmt::Display for ITree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt("".to_string(), 0))
    }
}
