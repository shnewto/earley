use crate::error::Error;
use crate::istate::{FlippedIState, IState};
use crate::itree::{IBranch, ITree};
use crate::tree::Tree;
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
    pub chart: Vec<LinkedHashSet<IState>>,
    pub accepted_states: Vec<IState>,
    pub input: Vec<String>,
}

impl EarleyAccepted {
    pub fn new(
        chart: Vec<LinkedHashSet<IState>>,
        accepted_states: Vec<IState>,
        input: Vec<String>,
    ) -> EarleyAccepted {
        EarleyAccepted {
            chart,
            accepted_states,
            input,
        }
    }

    fn find_in_chart(
        &self,
        parent_state: &FlippedIState,
        term: &Term,
        i: usize,
        limit: Option<usize>,
        chart: &[LinkedHashSet<FlippedIState>],
    ) -> Vec<FlippedIState> {
        let mut candidates = vec![];
        if let Some(state_set) = chart.get(i) {
            for state in state_set {
                if (parent_state != state) && term == &state.prod.lhs {
                    if let Some(l) = limit {
                        if state.end < l {
                            candidates.push(state.clone())
                        }
                    } else {
                        candidates.push(state.clone())
                    }
                }
            }
        }

        candidates
    }

    fn construct(
        &self,
        x: usize,
        state: &FlippedIState,
        chart: &[LinkedHashSet<FlippedIState>],
    ) -> ITree {
        let mut tree = ITree {
            root: state.clone(),
            branches: vec![],
        };

        let mut idxs: Vec<usize> = vec![x];

        let mut terms = state.prod.rhs.iter();
        let mut term_opt = terms.next();

        let mut candidates: Vec<(usize, IBranch)> = vec![];
        let mut success_indexes: Vec<usize> = vec![];
        let mut next_idxs: Vec<usize> = vec![];

        while let Some(term) = term_opt {
            let mut limit: Option<usize> = None;
            if let Some(n) = terms.clone().peekable().peek() {
                match n {
                    Term::Terminal(_) => limit = Some(self.input.len()),
                    Term::Nonterminal(_) => limit = Some(chart.len()),
                }
            }

            term_opt = terms.next();
            for idx in idxs {
                match term {
                    Term::Nonterminal(_) => {
                        let res = self.find_in_chart(state, term, idx, limit, chart);
                        for s in res {
                            let new_branch =
                                IBranch::Nonterminal(idx, self.construct(idx, &s, chart));
                            let insert_at = if term_opt.is_none() {
                                idx
                            } else {
                                next_idxs.push(s.end);
                                s.end
                            };
                            let _ = candidates.push((insert_at, new_branch));
                            success_indexes.push(idx);
                        }
                    }
                    Term::Terminal(symbol) => {
                        if let Some(found) = self.input.get(idx) {
                            if found == symbol {
                                let new_branch = IBranch::Terminal(idx, symbol.to_string());
                                tree.branches.push(new_branch);
                                success_indexes.push(idx);
                                next_idxs.push(idx + 1);
                            }
                        }
                    }
                }
            }
            idxs = next_idxs.clone();
        }

        for idx in success_indexes {
            tree.branches.append(
                &mut candidates
                    .iter()
                    .filter(|(i, _)| *i == idx)
                    .map(|(_, b)| b)
                    .cloned()
                    .collect(),
            );
        }

        tree.branches.dedup();
        tree.order();
        tree
    }

    pub fn parse_forest(&self) -> Result<Vec<Tree>, Error> {
        let flipped_chart = self.flip_completed();
        let flipped_start_states: Vec<FlippedIState>;
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

        let mut itrees: Vec<ITree> = vec![];
        for state in flipped_start_states {
            itrees.push(self.construct(0, &state, &flipped_chart));
        }

        let trees: Vec<Tree> = itrees.iter().map(|t| t.to_tree()).collect();

        Ok(trees)
    }

    pub fn flip_completed(&self) -> Vec<LinkedHashSet<FlippedIState>> {
        let mut flipped = vec![LinkedHashSet::new(); self.chart.len()];

        for (i, state_set) in self.get_completed().iter().enumerate() {
            for state in state_set {
                let flipped_state = FlippedIState::new(state.prod.clone(), i);
                flipped[state.origin].insert(flipped_state);
            }
        }

        flipped
    }

    pub fn get_completed(&self) -> Vec<LinkedHashSet<IState>> {
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

    pub fn get_completed_as_vecs(&self) -> Vec<Vec<IState>> {
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

    fn state_sets_to_vec(chart: Vec<LinkedHashSet<IState>>) -> Vec<Vec<IState>> {
        chart
            .iter()
            .map(|state_set| state_set.iter().cloned().collect::<Vec<IState>>())
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
