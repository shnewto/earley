use crate::error::Error;
use bnf::{Term, Grammar, Production, Expression};
use linked_hash_set::LinkedHashSet;
use serde::{Deserialize, Serialize};

pub type EarlyChart = Vec<LinkedHashSet<State>>;


#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    origin: usize,
    prod: EarlyProd
}

impl State {
    pub fn new(prod: EarlyProd, origin: usize) -> State {
        State {
            prod,
            origin,
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct EarlyProd {
    lhs: Term,
    rhs: Vec<Term>,
    dot: usize,
}

impl EarlyProd {
    pub fn new(lhs: Term, rhs: Vec<Term>, dot: usize) -> EarlyProd {
        EarlyProd {
            lhs,
            rhs,
            dot,
        }
    }

    pub fn get_next(&self) -> Option<&Term> {
        self.rhs.iter().nth(self.dot)
    }
}
pub struct EarlyParser {
    state_sets: EarlyChart,
    input: String,
    grammar: Grammar,
}

impl EarlyParser {
    pub fn new(grammar: &str, input: &str) -> Result<EarlyParser, Error> {
        Ok(EarlyParser {
            state_sets: vec![],
            input: input.to_string(),
            grammar: grammar.parse()?,
        })
    }


    pub fn get_start_states(&self) -> Result<LinkedHashSet<State>, Error> {
        match self.grammar.productions_iter().peekable().peek() {
            Some(p) => {
                let mut state_set: LinkedHashSet<State> = LinkedHashSet::new();
                for expr in p.rhs_iter() {
                    let terms = expr.terms_iter().map(|t| t.clone()).collect::<Vec<Term>>();
                    let prod = EarlyProd::new((p.lhs).clone(), terms, 0);
                    let state = State::new(prod, 0);
                    state_set.insert(state);
                }

                Ok(state_set)
            },
            None => {
                Err(Error::GrammarError(
                    format!("No start state candidate found in grammar: {}", self.grammar)
                ))
            },
        }
    }

    pub fn earley_parse(&self) -> Result<EarlyChart, Error> {
        let start_states = self.get_start_states()?;

        let mut k:usize = 0;


        Ok(vec![])
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
    fn earley_predict(&self, k: usize, state_set: LinkedHashSet<State>) -> LinkedHashSet<State> {

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

        let mut ret_state_set : LinkedHashSet<State> = LinkedHashSet::new();

        for state in state_set.iter() {
            if let Some(term) = state.prod.get_next() {
                if let Term::Nonterminal(_) = term {
                    // let prods = self.find_productions_in_grammar(term);
                    find_productions_in_grammar(term)
                        .iter().for_each(|p|{
                        let exprs = p.rhs_iter().map(|e| e.clone()).collect::<Vec<Expression>>();
                        exprs.iter().for_each(|e| {
                            let rhs = e.terms_iter().map(|t| t.clone()).collect::<Vec<Term>>();
                            let earley_prod = EarlyProd::new(p.lhs.clone(), rhs, 0);
                            ret_state_set.insert(State::new(earley_prod, k));
                        });
                    });
                }
            }
        }

        ret_state_set
    }

    // fn find_productions_in_grammar(&self, term: &Term) -> Vec<&Production> {
    //     let mut ret: Vec<&Production> = vec![];
    //     for p in self.grammar.productions_iter() {
    //         if (*p).lhs == *term {
    //             // ret.push(p.clone())
    //             ret.push(p)
    //         }
    //     }
    //
    //     ret
    // }

    /// Scanning:
    /// If a is the next symbol in the input stream,
    /// for every state in S(k) of the form (X → α • a β, j),
    /// add (X → α a • β, j) to S(k+1).
    ///
    /// For every `curr_state: State` in `state_set` where `symbol` is the
    /// char being evaluated from the input string,
    /// `state.prod.get_next() == Some(bnf::Term::Terminal::from_str(symbol)`,
    /// add `new_state: State` to the returned state set, where:
    /// new_state = curr_state.clone()
    /// new_state.prod.dot = curr_state.prod.dot + 1
    fn earley_scan(&self, term: &Term, symbol: String, state_set: LinkedHashSet<State>) -> LinkedHashSet<State> {
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
}