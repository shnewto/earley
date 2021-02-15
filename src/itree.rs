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

    fn fmt(&self, padding: usize, ppchar: PPChar) -> String {
        let mut value: String;
        value = format!("{:>padding$} {}\n", ppchar.get(), self.root.prod, padding=padding);

        for (i, branch) in self.branches.iter().enumerate() {
            if i == self.branches.len() - 1{
                value += &*branch.fmt(padding+4, PPChar::Last);
            } else {
                value += &*branch.fmt(padding+4, PPChar::Mid);
            }
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
    fn fmt(&self, padding: usize, ppchar: PPChar) -> String {
        match self {
            IBranch::Nonterminal(_, t) => t.fmt(padding, ppchar),
            IBranch::Terminal(_, s) =>  {
                format!("{:>padding$} {}\n", ppchar.get(), s, padding=padding)
            },
        }
    }
}

impl fmt::Display for IBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(0, PPChar::First))
    }
}

impl fmt::Display for ITree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(0, PPChar::First))
    }
}


enum PPChar {
    Last,
    Mid,
    First,
}

impl PPChar {
    fn get(&self) -> String {
        match self {
            PPChar::Last => "└─".to_string(),
            PPChar::Mid => "├─".to_string(),
            PPChar::First => "└─".to_string(),
        }
    }
}
