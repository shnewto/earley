use crate::error::Error;
use bnf::{Grammar, Production, Term};
use std::fmt;
use std::str::FromStr;

use linked_hash_set::LinkedHashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub struct EarleyParser {
    grammar: Grammar,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub origin: Option<usize>,
    pub lhs: Option<Term>,
    pub terms: Vec<Term>,
    pub dot: Option<usize>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Node {
    pub term: Term,
    pub leaves: Vec<Node>,
}

fn earley_predictor(term: &Term, k: usize, grammar: &Grammar) -> LinkedHashSet<State> {
    let mut productions: LinkedHashSet<State> = LinkedHashSet::new();

    for prod in grammar.productions_iter() {
        if prod.lhs == *term {
            for expr in prod.rhs_iter() {
                productions.insert(State {
                    origin: Some(k),
                    lhs: Some(prod.lhs.clone()),
                    terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                    dot: Some(0),
                });
            }
        }
    }

    productions
}

fn earley_scanner(
    term: &Term,
    k: usize,
    words: &String,
    grammar: &Grammar,
    production: &State,
) -> LinkedHashSet<State> {
    let mut pattern: String = String::new();
    let mut matches: LinkedHashSet<State> = LinkedHashSet::new();
    for (_, c) in words[k..].chars().enumerate() {
        pattern.push(c);
        for prod in grammar.productions_iter() {
            for expr in prod.rhs_iter() {
                for t in expr.terms_iter() {
                    if let Term::Terminal(ref s) = *t {
                        if t == term {
                            if pattern == *s {
                                let mut update = production.clone();
                                if let Some(dot) = update.dot {
                                    update.dot = Some(dot + 1);
                                }
                                matches.insert(update);
                            }
                        }
                    }
                }
            }
        }
    }

    matches
}

fn earley_completer(productions: &LinkedHashSet<State>, finished: &State) -> LinkedHashSet<State> {
    let mut updates: LinkedHashSet<State> = LinkedHashSet::new();
    for production in productions {
        if let Some(term) = earley_next_element(production) {
            if let &Some(ref lhs) = &finished.lhs {
                if lhs == term {
                    let mut update = production.clone();
                    if let Some(dot) = update.dot {
                        update.dot = Some(dot + 1);
                    }
                    updates.insert(update);
                }
            }
        }
    }

    updates
}

fn earley_init(grammar: &Grammar) -> Option<LinkedHashSet<State>> {
    if let Some(prod) = grammar.productions_iter().nth(0) {
        let mut productions: LinkedHashSet<State> = LinkedHashSet::new();
        for expr in prod.rhs_iter() {
            productions.insert(State {
                origin: Some(0),
                lhs: Some(prod.lhs.clone()),
                terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                dot: Some(0),
            });
        }

        return Some(productions);
    }

    return None;
}

pub fn earley_next_element(production: &State) -> Option<&Term> {
    if let Some(dot) = production.dot {
        return production.terms.iter().nth(dot);
    }

    None
}

fn hashset(data: &[State]) -> LinkedHashSet<State> {
    LinkedHashSet::from_iter(data.iter().cloned())
}

pub fn final_state_candidates(grammar: &Grammar) -> Option<Vec<State>> {
    let start_prod: &Production = grammar.productions_iter().next()?;

    Some(
        start_prod
            .rhs_iter()
            .map(|e| State {
                origin: Some(0),
                dot: Some(e.terms_iter().count()),
                lhs: Some(start_prod.lhs.clone()),
                terms: e.terms_iter().into_iter().map(|t| t.clone()).collect(),
            })
            .collect::<Vec<State>>(),
    )
}

impl EarleyParser {
    pub fn new(grammar: Grammar) -> EarleyParser {
        EarleyParser { grammar }
    }

    pub fn grammar(&self) -> Grammar {
        self.grammar.clone()
    }

    pub fn earley_parse(&mut self, input: String) -> Result<Vec<LinkedHashSet<State>>, Error> {
        let grammar = self.grammar.clone();

        let mut states: Vec<LinkedHashSet<State>> = vec![LinkedHashSet::new(); input.len() + 1];
        let mut productions: Vec<State> = vec![];

        let mut tokens: Vec<String> = vec![];

        if let Some(intial) = earley_init(&grammar) {
            states[0] = intial;
        } else {
            println!("Something in init went wrong!");
        }

        for k in 0..input.len() + 1 {
            if let Some(state) = states.iter_mut().nth(k) {
                productions = state.iter().cloned().collect::<Vec<_>>();
                state.clear();
            }
            while let Some(production) = productions.pop() {
                if let Some(state) = states.iter_mut().nth(k) {
                    if state.contains(&production) {
                        continue;
                    }

                    state.insert(production.clone());
                }

                if let Some(term) = earley_next_element(&production) {
                    match *term {
                        Term::Nonterminal(_) => {
                            let predicted = earley_predictor(&term, k, &grammar);
                            productions =
                                hashset(&productions).union(&predicted).cloned().collect();
                        }
                        Term::Terminal(ref t) => {
                            if let Some(state) = states.iter_mut().nth(k + 1) {
                                let scanned =
                                    earley_scanner(&term, k, &input, &grammar, &production);

                                if scanned.len() > 0 {
                                    tokens.push(t.clone());
                                }

                                *state = scanned.union(&state).cloned().collect();
                            }
                        }
                    }
                } else {
                    if let Some(origin) = production.origin {
                        if let Some(state) = states.iter_mut().nth(origin) {
                            let completed = earley_completer(&state, &production);
                            productions =
                                hashset(&productions).union(&completed).cloned().collect();
                        }
                    }
                }
            }
        }
        let final_states_actual = states.iter().last().unwrap();
        if let Some(candidates) = final_state_candidates(&self.grammar) {
            for candidate in candidates {
                if final_states_actual.contains(&candidate) {
                    return Ok(states);
                }
            }
        }

        Err(Error::InputRejected(format!(
            "Input '{}' rejected by grammar. Expected: Final States: {}",
            input,
            final_states_actual
                .iter()
                .map(|s| format!("\n{:?}", s))
                .collect::<String>()
        )))
    }
}

impl FromStr for EarleyParser {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Grammar::from_str(s) {
            Result::Ok(g) => Ok(EarleyParser::new(g)),
            Result::Err(e) => Err(Error::from(e)),
        }
    }
}

impl fmt::Display for EarleyParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.grammar().to_string())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let terms: String = self
            .terms
            .iter()
            .enumerate()
            .map(|(i, t)| {
                if Some(i) == self.dot && i == 0 {
                    format!("{}{:#} ", "•", t)
                } else if Some(i) == self.dot {
                    format!("{:#}{} ", t, "•")
                } else if i + 1 == self.terms.len() && self.dot == Some(self.terms.len()) {
                    format!("{:#}{} ", t, "•")
                } else {
                    format!("{:#} ", t)
                }
            })
            .collect();

        write!(
            f,
            "[{}] {} := {}",
            self.origin.unwrap_or(0),
            self.lhs
                .as_ref()
                .unwrap_or(&Term::Nonterminal("??".to_string())),
            terms
        )
    }
}

//
// impl PartialEq for State {
//     fn eq(&self, other: &Self) -> bool {
//         self.dot == other.dot &&
//             self.terms == other.terms &&
//             self.origin == other.origin &&
//             self.lhs == other.lhs
//     }
// }
//
// impl Eq for State {}
//
// impl Hash for State {
//     fn hash<H: Hasher>(&self, hasher: &mut H) {
//         self.dot.hash(hasher);
//         self.terms.hash(hasher);
//         self.origin.hash(hasher);
//         self.lhs.hash(hasher);
//     }
// }
