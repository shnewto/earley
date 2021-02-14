use crate::error::Error;
use crate::state::{FlippedIntermediateState, IntermediateState};
use crate::tree::{Leaf, Tree};
use bnf::Term;
use linked_hash_set::LinkedHashSet;
use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EarleyOutcome {
    Accepted(EarleyAccepted),
    Rejected,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyAccepted {
    pub chart: Vec<LinkedHashSet<IntermediateState>>,
    pub accepted_states: Vec<IntermediateState>,
    pub input: String,
}

impl EarleyAccepted {
    pub fn new(
        chart: Vec<LinkedHashSet<IntermediateState>>,
        accepted_states: Vec<IntermediateState>,
        input: String,
    ) -> EarleyAccepted {
        EarleyAccepted {
            chart,
            accepted_states,
            input,
        }
    }

    fn find_in_chart(
        &self,
        parent_state: &FlippedIntermediateState,
        term: &Term,
        i: usize,
        limit: Option<usize>,
        chart: &[LinkedHashSet<FlippedIntermediateState>],
    ) -> Vec<FlippedIntermediateState> {
        let mut ret = vec![];
        if let Some(state_set) = chart.get(i) {
            for state in state_set {
                if (parent_state != state) && term == &state.prod.lhs {
                    if let Some(l) = limit {
                        if state.end < l {
                            ret.push(state.clone());
                        }
                    } else {
                        ret.push(state.clone());
                    }
                }
            }
        }

        ret
    }

    fn construct(
        &self,
        x: usize,
        state: &FlippedIntermediateState,
        chart: &[LinkedHashSet<FlippedIntermediateState>],
        prefix: String,
    ) -> Tree {
        let mut tree = Tree {
            root: state.clone(),
            leaves: vec![],
        };

        let mut idxs: Vec<usize> = vec![x];

        let mut terms = state.prod.rhs.iter();
        let mut term_opt = terms.next();

        while let Some(term) = term_opt {
            let mut limit: Option<usize> = None;
            if let Some(n) = terms.clone().peekable().peek() {
                match n {
                    Term::Terminal(_) => limit = Some(self.input.len()),
                    Term::Nonterminal(_) => limit = Some(chart.len()),
                }
            }

            term_opt = terms.next();

            let mut next_idxs: Vec<usize> = vec![];

            for idx in idxs {
                match term {
                    Term::Nonterminal(_) => {
                        let res: Vec<FlippedIntermediateState> =
                            self.find_in_chart(state, term, idx, limit, chart);
                        for s in res {
                            tree.leaves.push(Leaf::Nonterminal(
                                idx,
                                self.construct(idx, &s, chart, prefix.clone() + "\t"),
                            ));
                            if term_opt.is_some() {
                                next_idxs.push(s.end);
                            }
                        }
                    }
                    Term::Terminal(symbol) => {
                        if let Some(found) = self.input.chars().nth(idx) {
                            if &found.to_string() == symbol {
                                tree.leaves.push(Leaf::Terminal(idx, symbol.to_string()));
                                if term_opt.is_some() {
                                    next_idxs.push(idx + 1);
                                }
                            }
                        }
                    }
                }
            }
            idxs = next_idxs;
        }

        tree
    }

    pub fn parse_forest(&self) -> Result<Vec<Tree>, Error> {
        let flipped_chart = self.flip_completed();
        let flipped_start_states: Vec<FlippedIntermediateState>;
        if let Some(inital) = flipped_chart.get(0) {
            flipped_start_states = inital
                .iter()
                .filter(|s| s.end == flipped_chart.len() - 1)
                .cloned()
                .collect();
        } else {
            return Err(Error::ParseForestError(
                "Couldn't a start state candidate!".to_string(),
            ));
        }

        let mut trees: Vec<Tree> = vec![];
        for state in flipped_start_states {
            trees.push(self.construct(0, &state, &flipped_chart, "".to_string()));
        }

        Ok(trees)
    }

    pub fn flip_completed(&self) -> Vec<LinkedHashSet<FlippedIntermediateState>> {
        let mut flipped = vec![LinkedHashSet::new(); self.chart.len()];

        for (i, state_set) in self.get_completed().iter().enumerate() {
            for state in state_set {
                let flipped_state = FlippedIntermediateState::new(state.prod.clone(), i);
                flipped[state.origin].insert(flipped_state);
            }
        }

        flipped
    }

    pub fn get_completed(&self) -> Vec<LinkedHashSet<IntermediateState>> {
        let mut only_completed = vec![];

        for state_sets in &self.chart {
            let mut reduced = LinkedHashSet::new();

            for state in state_sets {
                if state.prod.dot == (state.prod.rhs.len()) {
                    reduced.insert(state.clone());
                }
            }
            only_completed.push(reduced);
        }

        only_completed
    }

    pub fn get_completed_as_vecs(&self) -> Vec<Vec<IntermediateState>> {
        let mut only_completed = vec![];

        for state_sets in &self.chart {
            let mut reduced = LinkedHashSet::new();

            for state in state_sets {
                if state.prod.dot == (state.prod.rhs.len()) {
                    reduced.insert(state.clone());
                }
            }
            only_completed.push(reduced);
        }

        EarleyAccepted::state_sets_to_vec(only_completed)
    }

    fn state_sets_to_vec(
        chart: Vec<LinkedHashSet<IntermediateState>>,
    ) -> Vec<Vec<IntermediateState>> {
        chart
            .iter()
            .map(|state_set| {
                state_set
                    .iter()
                    .cloned()
                    .collect::<Vec<IntermediateState>>()
            })
            .collect()
    }
}

impl fmt::Display for EarleyAccepted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value: String = "".to_string();
        for (i, states) in self.chart.iter().enumerate() {
            value += &format!("\n=== {} ===\n", i);
            for state in states.iter() {
                value += &format!("{}\n", state);
            }
        }
        write!(f, "{}", value)
    }
}

impl fmt::Display for EarleyOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EarleyOutcome::Accepted(o) => write!(f, "{}", o),
            EarleyOutcome::Rejected => write!(f, "Rejected"),
        }
    }
}
