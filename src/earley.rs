use crate::error::Error;
use crate::outcome::{EarleyAccepted, EarleyOutcome};
use crate::prod::EarleyProd;
use crate::state::IntermediateState;
use bnf::{Expression, Grammar, Production, Term};
use linked_hash_set::LinkedHashSet;

pub struct EarleyParser {
    input: String,
    grammar: Grammar,
}

impl EarleyParser {
    pub fn new(grammar: &str, input: &str) -> Result<EarleyParser, Error> {
        Ok(EarleyParser {
            input: input.to_string(),
            grammar: grammar.parse()?,
        })
    }

    fn get_start_states(&self) -> Result<LinkedHashSet<IntermediateState>, Error> {
        match self.grammar.productions_iter().peekable().peek() {
            Some(p) => {
                let mut state_set: LinkedHashSet<IntermediateState> = LinkedHashSet::new();
                for expr in p.rhs_iter() {
                    let terms = expr.terms_iter().cloned().collect::<Vec<Term>>();
                    let prod = EarleyProd::new((p.lhs).clone(), terms, 0);
                    let state = IntermediateState::new(prod, 0);
                    state_set.insert(state);
                }
                Ok(state_set)
            }
            None => Err(Error::GrammarError(format!(
                "No start state candidate found in grammar: {}",
                self.grammar
            ))),
        }
    }

    pub fn earley_parse(self) -> Result<EarleyOutcome, Error> {
        let start_states = self.get_start_states()?;
        let input_symbols = self.input.chars().collect::<Vec<char>>();

        let mut chart: Vec<LinkedHashSet<IntermediateState>> =
            vec![LinkedHashSet::new(); input_symbols.len() + 1];
        chart[0] = start_states.clone();

        for k in 0..chart.len() {
            let mut unchanged = LinkedHashSet::new();

            while unchanged != chart[k] {
                unchanged = chart[k].clone();

                chart[k] = self.earley_predict(k, &chart[k]);
                if k + 1 < chart.len() && k < input_symbols.len() {
                    chart[k + 1] = self.earley_scan(input_symbols[k].to_string(), &chart[k]);
                }
                chart[k] = self.earley_complete(&chart[k], &chart);
            }
        }

        if let Some(accepted_states) = self.get_accepted(
            &start_states,
            chart.last(),
            input_symbols.len(),
            chart.len(),
        ) {
            Ok(EarleyOutcome::Accepted(EarleyAccepted::new(
                chart,
                accepted_states,
                self.input,
            )))
        } else {
            Ok(EarleyOutcome::Rejected)
        }
    }

    fn get_accepted(
        &self,
        start_states: &LinkedHashSet<IntermediateState>,
        final_chart_states: Option<&LinkedHashSet<IntermediateState>>,
        input_len: usize,
        chart_len: usize,
    ) -> Option<Vec<IntermediateState>> {
        if input_len + 1 != chart_len {
            return None;
        }

        let mut completed: Vec<IntermediateState> = vec![];
        match final_chart_states {
            Some(final_states) => {
                for start_state in start_states {
                    let find = final_states.iter().find(|final_state| {
                        final_state.origin == 0
                            && final_state.prod.dot == final_state.prod.rhs.len()
                            && start_state.prod.lhs == final_state.prod.lhs
                            && start_state.prod.rhs == final_state.prod.rhs
                    });

                    if let Some(found) = find {
                        completed.push(found.clone());
                    }
                }
                if completed.is_empty() {
                    None
                } else {
                    Some(completed)
                }
            }
            _ => None,
        }
    }

    /// Prediction:
    /// For every state in S(k) of the form (X → α • Y β, j)
    /// (where j is the origin position as above),
    /// add (Y → • γ, k) to S(k) for every production in the
    /// grammar with Y on the left-hand side (Y → γ).
    /// [https://en.wikipedia.org/wiki/Earley_parser]
    ///
    ///
    /// For every `state: IntermediateState` in `state_set` where
    /// `state.prod.get_next() == Some(bnf::Term::Nonterm(nt))`,
    /// find all productions `prod` in self.grammar where `prod.lhs == nt`
    /// and add a new state to the returned `state_set` for all
    /// bnf::Expression `expr` in `prod.rhs` where:
    /// state.lhs = prod.lhs
    /// state.rhs = expr collected into a Vec<Term>
    /// origin = `k` (the index of this state set in the EarleyChart)
    /// dot = 0
    fn earley_predict(
        &self,
        k: usize,
        state_set: &LinkedHashSet<IntermediateState>,
    ) -> LinkedHashSet<IntermediateState> {
        let find_productions_in_grammar = |term: &Term| {
            let mut ret: Vec<&Production> = vec![];
            for p in self.grammar.productions_iter() {
                if (*p).lhs == *term {
                    // ret.push(p.clone())
                    ret.push(p)
                }
            }

            ret
        };

        let mut ret_state_set: LinkedHashSet<IntermediateState> = state_set.clone();

        for state in state_set.iter() {
            if let Some(term) = state.prod.get_next() {
                if let Term::Nonterminal(_) = term {
                    // let prods = self.find_productions_in_grammar(term);
                    find_productions_in_grammar(term).iter().for_each(|p| {
                        let exprs = p.rhs_iter().cloned().collect::<Vec<Expression>>();
                        exprs.iter().for_each(|e| {
                            let rhs = e.terms_iter().cloned().collect::<Vec<Term>>();
                            let earley_prod = EarleyProd::new(p.lhs.clone(), rhs, 0);
                            ret_state_set.insert(IntermediateState::new(earley_prod, k));
                        });
                    });
                }
            }
        }

        ret_state_set
    }

    /// Scanning:
    /// If a is the next symbol in the input stream,
    /// for every state in S(k) of the form (X → α • a β, j),
    /// add (X → α a • β, j) to S(k+1).
    /// [https://en.wikipedia.org/wiki/Earley_parser]
    ///
    /// For every `curr_state: IntermediateState` in `state_set` where `symbol` is the
    /// char being evaluated from the input string,
    /// `state.prod.get_next() == Some(bnf::Term::Terminal::from_str(symbol)`,
    /// add `new_state: IntermediateState` to the returned state set, where:
    /// new_state = curr_state.clone()
    /// new_state.prod.dot = curr_state.prod.dot + 1
    fn earley_scan(
        &self,
        symbol: String,
        state_set: &LinkedHashSet<IntermediateState>,
    ) -> LinkedHashSet<IntermediateState> {
        let mut ret_state_set: LinkedHashSet<IntermediateState> = LinkedHashSet::new();

        for state in state_set.iter() {
            if let Some(term) = state.prod.get_next() {
                if let Term::Terminal(s) = term {
                    if *s == symbol {
                        let mut incremented = state.clone();
                        incremented.prod.dot = state.prod.dot + 1;
                        ret_state_set.insert(incremented);
                    }
                }
            }
        }

        ret_state_set
    }

    /// Completion:
    /// For every state in S(k) of the form (Y → γ •, j),
    /// find all states in S(j) of the form (X → α • Y β, i)
    /// and add (X → α Y • β, i) to S(k).
    /// [https://en.wikipedia.org/wiki/Earley_parser]
    ///
    /// For every `curr_state: IntermediateState` in `state_set` where
    /// curr_state.prod.get_next() == None, find all states in
    /// the Vec<LinkedHashSet<IntermediateState>> where
    /// `chart_state[curr_state.origin].get_next() == Some(curr_state.prod.lhs)`
    /// and add a state to the returned state set where:
    /// new_state = chart_state.clone()
    /// new_state.dot = chart_state.prod.dot + 1
    fn earley_complete(
        &self,
        state_set: &LinkedHashSet<IntermediateState>,
        chart: &[LinkedHashSet<IntermediateState>],
    ) -> LinkedHashSet<IntermediateState> {
        let find_in_chart = |lhs: &Term, index: usize| {
            let mut ret_states: Vec<IntermediateState> = vec![];
            if let Some(states_at_index) = chart.get(index) {
                for state in states_at_index {
                    if state.prod.get_next() == Some(lhs) {
                        ret_states.push(state.clone());
                    }
                }
            }
            ret_states
        };

        let mut ret_state_set: LinkedHashSet<IntermediateState> = state_set.clone();

        for state in state_set {
            if state.prod.get_next().is_none() {
                let next_states = find_in_chart(&state.prod.lhs, state.origin);
                for n in next_states {
                    let mut new_state = n.clone();
                    new_state.prod.dot = n.prod.dot + 1;
                    ret_state_set.insert(new_state);
                }
            }
        }

        ret_state_set
    }
}
