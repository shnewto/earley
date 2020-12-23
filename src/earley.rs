use bnf::{Grammar, Term};
use crate::error::Error;
use std::str::FromStr;
use std::fmt;
use std::fmt::Write;

use std::collections::HashSet;
use std::iter::FromIterator;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EarleyParser {
    grammar: Grammar,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EarleyState {
    pub origin: Option<usize>,
    pub lhs: Option<Term>,
    pub terms: Vec<Term>,
    pub dot: Option<usize>,
}

struct Node {
    term: Term,
    edges: Vec<Node>
}

impl EarleyParser {
    pub fn new(grammar: Grammar) -> EarleyParser {
        EarleyParser {
            grammar,
        }
    }

    pub fn grammar(&self) -> Grammar {
        self.grammar.clone()
    }

    pub fn earley_parse(&mut self, sentence: String) -> Result<Vec<HashSet<EarleyState>>, Error> {
        let mut productions: Vec<EarleyState> = vec![];
        let mut states = vec![HashSet::new(); sentence.len() + 1];
        let mut tokens: Vec<String> = vec![];

        self.earley_init()?;

        for k in 0..sentence.len() + 1 {
            // TODO revisit why this was used in a previous implementation...
            if let Some(state) = states.iter_mut().nth(k) {
                productions = state.iter().cloned().collect::<Vec<_>>();
                state.drain();
            }

            while let Some(production) = productions.pop() {
                if let Some(state) = states.iter_mut().nth(k) {
                    if state.contains(&production) {
                        continue;
                    }

                    state.insert(production.clone());
                }

                if let Some(term) = self.earley_next_element(&production) {
                    match *term {
                        Term::Nonterminal(_) => {
                            let predicted = self.earley_predictor(&term, k, &self.grammar);
                            productions = self.hashset(&productions).union(&predicted).cloned().collect();
                        }
                        Term::Terminal(ref t) => {
                            if let Some(state) = states.iter_mut().nth(k + 1) {
                                let scanned = self.earley_scanner(&term, k, &sentence, &self.grammar, &production);

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
                            let completed = self.earley_completer(&state, &production);
                            productions = self.hashset(&productions).union(&completed).cloned().collect();
                        }
                    }
                }

            }
        }

        Ok(states)
        // return Err(Error::InputRejected("Input rejected by grammar".to_string()));
    }


    fn earley_init(&mut self) -> Result<HashSet<EarleyState>, Error> {
        if let Some(prod) = self.grammar.productions_iter().nth(0) {
            let mut productions: HashSet<EarleyState> = HashSet::new();
            for expr in prod.rhs_iter() {
                productions.insert(EarleyState {
                    origin: Some(0),
                    lhs: Some(prod.lhs.clone()),
                    terms: expr.terms_iter().cloned().collect::<Vec<_>>(),
                    dot: Some(0),
                });
            }

            return Ok(productions);
        }

        return Err(Error::InputRejected("Input rejected by grammar".to_string()));
    }


    fn earley_next_element<'a>(&self, production: &'a EarleyState) -> Option<&'a Term> {
        if let Some(dot) = production.dot {
            return production.terms.iter().nth(dot);
        }

        None
    }

    fn hashset(&self, data: &[EarleyState]) -> HashSet<EarleyState> {
        HashSet::from_iter(data.iter().cloned())
    }


    fn earley_predictor(&self, term: &Term, k: usize, grammar: &Grammar) -> HashSet<EarleyState> {
        let mut productions: HashSet<EarleyState> = HashSet::new();

        for prod in grammar.productions_iter() {
            if prod.lhs == *term {
                for expr in prod.rhs_iter() {
                    productions.insert(EarleyState {
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
        &self,
        term: &Term,
        k: usize,
        words: &String,
        grammar: &Grammar,
        production: &EarleyState,
    ) -> HashSet<EarleyState> {
        let mut pattern: String = String::new();
        let mut matches: HashSet<EarleyState> = HashSet::new();
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

    fn earley_completer(&self, productions: &HashSet<EarleyState>, finished: &EarleyState) -> HashSet<EarleyState> {
        let mut updates: HashSet<EarleyState> = HashSet::new();
        for production in productions {
            if let Some(term) = self.earley_next_element(&production) {
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

    pub fn accept(&mut self, sentence: String) -> Result<(), Error> {
        let states = self.earley_parse(sentence)?;
        let mut chart = String::new();
        for (i, state) in states.iter().enumerate() {
            println!("\n---S({})\n", i);
            for (_, production) in state.iter().enumerate() {
                let finished: String;
                if let None = self.earley_next_element(production) {
                    finished = String::from("(complete)");
                } else {
                    finished = String::from("");
                }
                println!(
                    "{} | {} -> {:?} - dot:{} {}",
                    production.origin.unwrap(), production.clone().lhs.unwrap(), production.terms, production.dot.unwrap(), finished
                );
            }
        }

        Ok(())
    }
}

impl FromStr for EarleyParser {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Grammar::from_str(s) {
            Result::Ok(g) => Ok(EarleyParser::new(g) ),
            Result::Err(e) => Err(Error::from(e)),
        }
    }
}

impl fmt::Display for EarleyParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}",
            self.grammar().to_string()
        )
    }
}