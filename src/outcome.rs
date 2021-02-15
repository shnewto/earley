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
    pub input: String,
}

impl EarleyAccepted {
    pub fn new(
        chart: Vec<LinkedHashSet<IState>>,
        accepted_states: Vec<IState>,
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

    fn check_next(
        &self,
        parent_state: &FlippedIState,
        term: &Term,
        idx: usize,
        limit: Option<usize>,
        chart: &[LinkedHashSet<FlippedIState>],
    ) -> bool {
        match term {
            Term::Nonterminal(_) => {
                if !self
                    .find_in_chart(parent_state, term, idx, limit, chart)
                    .is_empty()
                {
                    return true;
                }

                return false;
            }
            Term::Terminal(symbol) => {
                if let Some(found) = self.input.chars().nth(idx) {
                    if &found.to_string() == symbol {
                        return true;
                    }
                }

                return false;
            }
        }
    }

    fn construct(
        &self,
        x: usize,
        state: &FlippedIState,
        chart: &[LinkedHashSet<FlippedIState>],
        prefix: String,
    ) -> ITree {
        let mut tree = ITree {
            root: state.clone(),
            branches: vec![],
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
                        // let res: Vec<FlippedIState> =
                        // if let Some(res) = self.eval(term_opt, idx, chart) {
                        let res = self.find_in_chart(state, term, idx, limit, chart);
                            for s in res {
                                if let Some(t) = term_opt {
                                    if self.check_next(state, t, s.end, limit, chart) {
                                        tree.branches.push(IBranch::Nonterminal(
                                            idx,
                                            self.construct(idx, &s, chart, prefix.clone() + "\t"),
                                        ));

                                        if term_opt.is_some() {
                                            next_idxs.push(s.end);
                                        }
                                    }
                                } else {
                                    tree.branches.push(IBranch::Nonterminal(
                                        idx,
                                        self.construct(idx, &s, chart, prefix.clone() + "\t"),
                                    ));
                                }
                            }
                    }
                    Term::Terminal(symbol) => {
                        if let Some(found) = self.input.chars().nth(idx) {
                            if &found.to_string() == symbol {
                                tree.branches
                                    .push(IBranch::Terminal(idx, symbol.to_string()));
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

        tree.branches.dedup();

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
            itrees.push(self.construct(0, &state, &flipped_chart, "".to_string()));
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
