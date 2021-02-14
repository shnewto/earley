use crate::state::FlippedIntermediateState;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Tree {
    pub root: FlippedIntermediateState,
    pub leaves: Vec<Leaf>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Leaf {
    Nonterminal(usize, Tree),
    Terminal(usize, String),
}

impl Tree {
    pub fn new(root: FlippedIntermediateState) -> Tree {
        Tree {
            root,
            leaves: vec![],
        }
    }

    fn fmt(&self, prefix: String) -> String {
        let mut value: String = format!("{} [{}]\n{} |", prefix, self.root.prod, prefix);
        for leaf in &self.leaves {
            value += &*leaf.fmt(prefix.clone() + "");
        }

        value
    }

}

impl Leaf {
    pub fn index(&self) -> usize {
        match self {
            Leaf::Nonterminal(index, _) => *index,
            Leaf::Terminal(index, _) => *index,
        }
    }
    fn fmt(&self, prefix: String) -> String {
        match self {
            Leaf::Nonterminal(_, t) => t.fmt(prefix),
            Leaf::Terminal(_, s) => format!("{} [{}]\n", prefix, s),
        }
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt( "".to_string()))
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt( "".to_string()))
    }
}
