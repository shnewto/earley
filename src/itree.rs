use crate::istate::FlippedIState;
use serde::{Deserialize, Serialize};
use std::fmt;
use bnf::{Production, Expression};
use crate::tree::{Tree, Branch};

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
    pub fn to_tree(&self) -> Tree {
        Tree {
            production: Production::from_parts(self.root.prod.lhs.clone(), vec![Expression::from_parts(self.root.prod.rhs.clone())]),
            branches: self.branches.iter().map(|b| b.to_branch()).collect(),
        }
    }

    fn fmt(&self, depth: usize, bars: Vec<usize>, ppchar: PPChar) -> String {
        let mut value: String;
        let mut next_bars;
        match ppchar {
            PPChar::Last => {
                next_bars = bars.clone()
            },
            PPChar::Mid => {
                next_bars = bars.clone();
                next_bars.push(depth*4-4);
            },
            PPChar::First => {
                next_bars = vec![];
            },
        }

        value = format!("{:>padding$} {}\n", ppchar.get(), self.root.prod, padding=depth*4);
        let mut val_chars = value.chars().collect::<Vec<char>>();
        for (i, bar) in bars.iter().enumerate() {
                val_chars.insert(bar+i+2, '|');
        }

        value = val_chars.iter().collect();

        for (i, branch) in self.branches.iter().enumerate() {
            if i == self.branches.len() - 1{
                value += &*branch.fmt(depth+1, next_bars.clone(), PPChar::Last);
            } else {
                value += &*branch.fmt(depth+1, next_bars.clone(), PPChar::Mid);
            }
        }

        value
    }
}

impl IBranch {
    pub fn to_branch(&self) -> Branch {
        match self {
            IBranch::Nonterminal(_, itree) => Branch::Nonterminal(itree.to_tree()),
            IBranch::Terminal(_, s) => Branch::Terminal(s.to_string()),
        }
    }

    fn fmt(&self, depth: usize, bars: Vec<usize>, ppchar: PPChar) -> String {
        match self {
            IBranch::Nonterminal(_, t) => t.fmt(depth, bars, ppchar),
            IBranch::Terminal(_, s) =>  {
                let value = format!("{:>padding$} {}\n", ppchar.get(), s, padding=depth*4);
                let mut val_chars = value.chars().collect::<Vec<char>>();
                for (i, bar) in bars.iter().enumerate() {
                    val_chars.insert(bar+i+2, '|');
                }

                val_chars.iter().collect()
            },
        }
    }
}

impl fmt::Display for IBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(0, vec![], PPChar::First))
    }
}

impl fmt::Display for ITree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(0, vec![], PPChar::First))
    }
}


pub enum PPChar {
    Last,
    Mid,
    First,
}

impl PPChar {
    pub fn get(&self) -> String {
        match self {
            PPChar::Last => "└─".to_string(),
            PPChar::Mid => "├─".to_string(),
            PPChar::First => "└─".to_string(),
        }
    }
}
