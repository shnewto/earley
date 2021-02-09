use crate::error::Error;
use bnf::{Expression, Grammar, Production, Term};
use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyChart;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EarleyOutcome {
    Accepted(EarleyAccepted),
    Rejected,
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyAccepted {
    pub chart: Vec<LinkedHashSet<State>>,
    pub accepted_states: Vec<State>,
    pub input: String,
}

impl EarleyAccepted {
    pub fn new(chart: Vec<LinkedHashSet<State>>, accepted_states: Vec<State>, input: String) -> EarleyAccepted {
        EarleyAccepted {
            chart,
            accepted_states,
            input
        }
    }

    pub fn parse_forest(&self) {
        // println!("accepted states len in parse_forest {}", self.accepted_states.len());
        for accepted_state in &self.accepted_states {
            println!("accepted state {}", accepted_state);
        }
    }
}


impl EarleyChart {
    pub fn eval(grammar: &str, input: &str) -> Result<EarleyOutcome, Error> {
        let parser = EarleyParser::new(grammar, input)?;
        let outcome = parser.earley_parse()?;

        // if let EarleyOutcome::Accepted(accepted) = &outcome {
        //     accepted.parse_forest();
        // }

        Ok(outcome.clone())
    }

    pub fn accept(grammar: &str, input: &str) -> Result<bool, Error> {
        let parser = EarleyParser::new(grammar, input)?;
        let res = parser.earley_parse()?;
        if let EarleyOutcome::Accepted(_) = res {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub origin: usize,
    pub prod: EarleyProd,
}

impl State {
    pub fn new(prod: EarleyProd, origin: usize) -> State {
        State { prod, origin }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let terms: String = self
            .prod
            .rhs
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if i == self.prod.dot {
                    format!("{}{:#}", "•", t)
                } else if i + 1 == self.prod.rhs.len() && self.prod.dot == self.prod.rhs.len() {
                    format!("{:#}{} ", t, "•")
                } else {
                    format!("{:#} ", t)
                }
            })
            .collect();

        write!(
            f,
            "[{}] {} := {} ({})",
            self.origin, self.prod.lhs, terms, self.prod.dot
        )
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarleyProd {
    pub lhs: Term,
    pub rhs: Vec<Term>,
    pub dot: usize,
}

impl EarleyProd {
    pub fn new(lhs: Term, rhs: Vec<Term>, dot: usize) -> EarleyProd {
        EarleyProd { lhs, rhs, dot }
    }
    pub fn get_next(&self) -> Option<&Term> {
        self.rhs.get(self.dot)
    }
}

pub struct EarleyParser {
    input: String,
    grammar: Grammar,
    // completed_states: LinkedHashSet<State>,
    // chart
}

impl EarleyParser {
    pub fn new(grammar: &str, input: &str) -> Result<EarleyParser, Error> {
        Ok(EarleyParser {
            input: input.to_string(),
            grammar: grammar.parse()?,
            // start_states: LinkedHashSet::new(),
        })
    }

    pub fn earley_parse(self) -> Result<EarleyOutcome, Error> {
        let get_start_states = || match self.grammar.productions_iter().peekable().peek() {
            Some(p) => {
                let mut state_set: LinkedHashSet<State> = LinkedHashSet::new();
                for expr in p.rhs_iter() {
                    let terms = expr.terms_iter().cloned().collect::<Vec<Term>>();
                    let prod = EarleyProd::new((p.lhs).clone(), terms, 0);
                    let state = State::new(prod, 0);
                    state_set.insert(state);
                }

                Ok(state_set)
            }
            None => Err(Error::GrammarError(format!(
                "No start state candidate found in grammar: {}",
                self.grammar
            ))),
        };

        let start_states = get_start_states()?;
        let input_symbols = self.input.chars().collect::<Vec<char>>();

        let mut chart: Vec<LinkedHashSet<State>> =
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

        if let Some(accepted_states) = self.get_accepted(&start_states, chart.last(), input_symbols.len(), chart.len()) {
            // println!("accepted_states first len {}", accepted_states.len());
            Ok(EarleyOutcome::Accepted(EarleyAccepted::new(chart, accepted_states, self.input)))
        } else {
            Ok(EarleyOutcome::Rejected)
        }
    }

    fn get_accepted(&self, start_states: &LinkedHashSet<State>,
                    final_chart_states: Option<&LinkedHashSet<State>>, input_len: usize, chart_len: usize) -> Option<Vec<State>> {

        if input_len + 1 != chart_len {
            return None
        }

        let mut completed: Vec<State> = vec![];
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
                        // println!("adding {} to accepted states", found);
                        completed.push(found.clone());
                    }
                }
                if completed.is_empty() {
                    None
                } else {
                    Some(completed)
                }
            },
            _ => None
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
    /// For every `state: State` in `state_set` where
    /// `state.prod.get_next() == Some(bnf::Term::Nonterm(nt))`,
    /// find all productions `prod` in self.grammar where `prod.lhs == nt`
    /// and add a new state to the returned `state_set` for all
    /// bnf::Expression `expr` in `prod.rhs` where:
    /// state.lhs = prod.lhs
    /// state.rhs = expr collected into a Vec<Term>
    /// origin = `k` (the index of this state set in the EarleyChart)
    /// dot = 0
    fn earley_predict(&self, k: usize, state_set: &LinkedHashSet<State>) -> LinkedHashSet<State> {
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

        let mut ret_state_set: LinkedHashSet<State> = state_set.clone();

        for state in state_set.iter() {
            if let Some(term) = state.prod.get_next() {
                if let Term::Nonterminal(_) = term {
                    // let prods = self.find_productions_in_grammar(term);
                    find_productions_in_grammar(term).iter().for_each(|p| {
                        let exprs = p.rhs_iter().cloned().collect::<Vec<Expression>>();
                        exprs.iter().for_each(|e| {
                            let rhs = e.terms_iter().cloned().collect::<Vec<Term>>();
                            let earley_prod = EarleyProd::new(p.lhs.clone(), rhs, 0);
                            ret_state_set.insert(State::new(earley_prod, k));
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
    /// For every `curr_state: State` in `state_set` where `symbol` is the
    /// char being evaluated from the input string,
    /// `state.prod.get_next() == Some(bnf::Term::Terminal::from_str(symbol)`,
    /// add `new_state: State` to the returned state set, where:
    /// new_state = curr_state.clone()
    /// new_state.prod.dot = curr_state.prod.dot + 1
    fn earley_scan(
        &self,
        symbol: String,
        state_set: &LinkedHashSet<State>,
    ) -> LinkedHashSet<State> {
        let mut ret_state_set: LinkedHashSet<State> = LinkedHashSet::new();

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
    /// For every `curr_state: State` in `state_set` where
    /// curr_state.prod.get_next() == None, find all states in
    /// the Vec<LinkedHashSet<State>> where
    /// `chart_state[curr_state.origin].get_next() == Some(curr_state.prod.lhs)`
    /// and add a state to the returned state set where:
    /// new_state = chart_state.clone()
    /// new_state.dot = chart_state.prod.dot + 1
    fn earley_complete(
        &self,
        state_set: &LinkedHashSet<State>,
        chart: &[LinkedHashSet<State>],
    ) -> LinkedHashSet<State> {
        let find_in_chart = |lhs: &Term, pos: usize| {
            let mut ret_states: Vec<State> = vec![];
            if let Some(states_at_pos) = chart.get(pos) {
                for state in states_at_pos {
                    if state.prod.get_next() == Some(lhs) {
                        ret_states.push(state.clone());
                    }
                }
            }
            ret_states
        };

        let mut ret_state_set: LinkedHashSet<State> = state_set.clone();

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
