use serde::{Deserialize, Serialize};
use std::fmt;
use bnf::{Production};
use crate::itree::PPChar;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Tree {
    pub production: Production,
    pub branches: Vec<Branch>,
}

impl Tree {
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

        value = format!("{:>padding$} {}\n", ppchar.get(), self.production, padding=depth*4);
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

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Branch {
    Nonterminal(Tree),
    Terminal(String),
}

impl Branch {

    fn fmt(&self, depth: usize, bars: Vec<usize>, ppchar: PPChar) -> String {
        match self {
            Branch::Nonterminal(t) => t.fmt(depth, bars, ppchar),
            Branch::Terminal(s) =>  {
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

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(0, vec![], PPChar::First))
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.fmt(0, vec![], PPChar::First))
    }
}
