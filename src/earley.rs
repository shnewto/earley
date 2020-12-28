use bnf::{Grammar, Term};
use crate::error::Error;
use std::str::FromStr;
use std::fmt;
use std::fmt::Write;

use linked_hash_set::LinkedHashSet;
use std::iter::FromIterator;

pub struct EarleyParser {
    grammar: Grammar,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    origin: Option<usize>,
    lhs: Option<Term>,
    terms: Vec<Term>,
    dot: Option<usize>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    term: Term,
    leaves: Vec<Node>,
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

fn earley_next_element(production: &State) -> Option<&Term> {
    if let Some(dot) = production.dot {
        return production.terms.iter().nth(dot);
    }

    None
}

fn hashset(data: &[State]) -> LinkedHashSet<State> {
    LinkedHashSet::from_iter(data.iter().cloned())
}

fn term_str(term: &Term) -> &String {
    match term {
        Term::Terminal(t) => t,
        Term::Nonterminal(n) => n,
    }
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

            while let Some(mut production) = productions.pop() {
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
                            productions = hashset(&productions).union(&predicted).cloned().collect();
                        }
                        Term::Terminal(ref t) => {
                            if let Some(state) = states.iter_mut().nth(k + 1) {
                                let scanned = earley_scanner(&term, k, &input, &grammar, &production);

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
                            productions = hashset(&productions).union(&completed).cloned().collect();
                        }
                    }
                }
            }
        }

        Ok(states)
    }

    pub fn accept(&mut self, sentence: String) -> Result<(), Error> {
        let states = self.earley_parse(sentence)?;
        let mut chart = String::new();
        for (i, state) in states.iter().enumerate() {
            println!("\nState({})", i);
            for (_, production) in state.iter().enumerate() {
                println!("{:?}", production);
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

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let finished: String;
        if let None = earley_next_element(self) {
            finished = String::from("(complete)");
        } else {
            finished = String::from("");
        }
        let mut terms = String::new();
        for (i, term) in self.terms.iter().enumerate() {
            write!(terms, "{}", term_str(term));
        }

        terms.insert(self.dot.unwrap(), '•');

        write!(f,
            "{} | {} -> {} {}",
                 self.origin.unwrap(), term_str(&self.clone().lhs.unwrap()), terms, finished
        );
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use bnf::{Term, Grammar};
    use crate::earley::{State, EarleyParser};
    use linked_hash_set::LinkedHashSet;
    use std::fs::File;
    use std::io::prelude::*;

    fn wikipedia_example_state_00() -> Vec<State> {

        let mut ret: Vec<State> = vec![];

        let s00_1 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("P".to_string())),
            terms: vec![Term::Nonterminal("S".to_string())],
            dot: Some(0),
        };

        ret.push(s00_1);

        let s00_2 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()), Term::Nonterminal("M".to_string())],
            dot: Some(0),
        };

        ret.push(s00_2);


        let s00_3 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("M".to_string())],
            dot: Some(0),
        };

        ret.push(s00_3);

        let s00_4 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()), Term::Nonterminal("T".to_string())],
            dot: Some(0),
        };

        ret.push(s00_4);

        let s00_5 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("T".to_string())],
            dot: Some(0),
        };

        ret.push(s00_5);

        let s00_6 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("1".to_string())],
            dot: Some(0),
        };

        ret.push(s00_6);

        let s00_7 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("2".to_string())],
            dot: Some(0),
        };

        ret.push(s00_7);

        let s00_8 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("3".to_string())],
            dot: Some(0),
        };

        ret.push(s00_8);

        let s00_9 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("4".to_string())],
            dot: Some(0),
        };

        ret.push(s00_9);

        ret
    }

    fn wikipedia_example_state_01() -> Vec<State> {
        let mut ret: Vec<State> = vec![];

        let s01_1 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("2".to_string())],
            dot: Some(1),
        };

        ret.push(s01_1);

        let s01_2 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("T".to_string())],
            dot: Some(1),
        };

        ret.push(s01_2);

        let s01_3 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()),  Term::Nonterminal("T".to_string())],
            dot: Some(1),
        };

        ret.push(s01_3);

        let s01_4 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("M".to_string())],
            dot: Some(1),
        };

        ret.push(s01_4);

        let s01_5 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()),  Term::Nonterminal("M".to_string())],
            dot: Some(1),
        };

        ret.push(s01_5);

        let s01_6 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("P".to_string())),
            terms: vec![Term::Nonterminal("S".to_string())],
            dot: Some(1),
        };

        ret.push(s01_6);

        ret
    }

    fn wikipedia_example_state_02() -> Vec<State> {
        let mut ret: Vec<State> = vec![];

        let s02_01 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()), Term::Nonterminal("M".to_string())],
            dot: Some(2),
        };

        ret.push(s02_01);

        let s02_02 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()), Term::Nonterminal("T".to_string())],
            dot: Some(0),
        };

        ret.push(s02_02);

        let s02_03 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("T".to_string())],
            dot: Some(0),
        };

        ret.push(s02_03);

        let s02_04 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("1".to_string())],
            dot: Some(0),
        };

        ret.push(s02_04);

        let s02_05 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("2".to_string())],
            dot: Some(0),
        };

        ret.push(s02_05);

        let s02_06 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("3".to_string())],
            dot: Some(0),
        };

        ret.push(s02_06);

        let s02_07 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("4".to_string())],
            dot: Some(0),
        };

        ret.push(s02_07);

        ret
    }

    fn wikipedia_example_state_03() -> Vec<State> {
        let mut ret: Vec<State> = vec![];

        let s03_01 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("3".to_string())],
            dot: Some(1),
        };

        ret.push(s03_01);

        let s03_02 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("T".to_string())],
            dot: Some(1),
        };

        ret.push(s03_02);

        let s03_03 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()), Term::Nonterminal("T".to_string())],
            dot: Some(1),
        };

        ret.push(s03_03);

        let s03_04 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()), Term::Nonterminal("M".to_string())],
            dot: Some(3),
        };

        ret.push(s03_04);

        let s03_05 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()), Term::Nonterminal("M".to_string())],
            dot: Some(1),
        };

        ret.push(s03_05);

        let s03_06 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("P".to_string())),
            terms: vec![Term::Nonterminal("S".to_string())],
            dot: Some(1),
        };

        ret.push(s03_06);

        ret
    }

    fn wikipedia_example_state_04() -> Vec<State> {
        let mut ret: Vec<State> = vec![];

        let s04_01 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()), Term::Nonterminal("T".to_string())],
            dot: Some(2),
        };

        ret.push(s04_01);

        let s04_02 = State {
            origin: Some(4),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("1".to_string())],
            dot: Some(0),
        };

        ret.push(s04_02);

        let s04_03 = State {
            origin: Some(4),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("2".to_string())],
            dot: Some(0),
        };

        ret.push(s04_03);

        let s04_04 = State {
            origin: Some(4),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("3".to_string())],
            dot: Some(0),
        };

        ret.push(s04_04);

        let s04_05 = State {
            origin: Some(4),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("4".to_string())],
            dot: Some(0),
        };

        ret.push(s04_05);

        ret
    }

    fn wikipedia_example_state_05() -> Vec<State> {
        let mut ret: Vec<State> = vec![];

        let s05_01 = State {
            origin: Some(4),
            lhs: Some(Term::Nonterminal("T".to_string())),
            terms: vec![Term::Terminal("4".to_string())],
            dot: Some(1),
        };

        ret.push(s05_01);

        let s05_02 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()), Term::Nonterminal("T".to_string())],
            dot: Some(3),
        };

        ret.push(s05_02);

        let s05_03 = State {
            origin: Some(2),
            lhs: Some(Term::Nonterminal("M".to_string())),
            terms: vec![Term::Nonterminal("M".to_string()), Term::Terminal("*".to_string()), Term::Nonterminal("T".to_string())],
            dot: Some(1),
        };

        ret.push(s05_03);

        let s05_04 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()), Term::Nonterminal("M".to_string())],
            dot: Some(3),
        };

        ret.push(s05_04);

        let s05_05 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("S".to_string())),
            terms: vec![Term::Nonterminal("S".to_string()), Term::Terminal("+".to_string()), Term::Nonterminal("M".to_string())],
            dot: Some(1),
        };

        ret.push(s05_05);

        let s05_06 = State {
            origin: Some(0),
            lhs: Some(Term::Nonterminal("P".to_string())),
            terms: vec![Term::Nonterminal("S".to_string())],
            dot: Some(1),
        };

        ret.push(s05_06);

        ret
    }

    fn wikipedia_example_states() -> Vec<LinkedHashSet<State>> {

        let state_00 = wikipedia_example_state_00();
        let mut state_00_hs: LinkedHashSet<State> = LinkedHashSet::new();

        for s in state_00 {
            state_00_hs.insert(s);
        }

        let state_01 = wikipedia_example_state_01();
        let mut state_01_hs: LinkedHashSet<State> = LinkedHashSet::new();

        for s in state_01 {
            state_01_hs.insert(s);
        }


        let state_02 = wikipedia_example_state_02();
        let mut state_02_hs: LinkedHashSet<State> = LinkedHashSet::new();

        for s in state_02 {
            state_02_hs.insert(s);
        }

        let state_03 = wikipedia_example_state_03();
        let mut state_03_hs: LinkedHashSet<State> = LinkedHashSet::new();

        for s in state_03 {
            state_03_hs.insert(s);
        }

        let state_04 = wikipedia_example_state_04();
        let mut state_04_hs: LinkedHashSet<State> = LinkedHashSet::new();

        for s in state_04 {
            state_04_hs.insert(s);
        }

        let state_05 = wikipedia_example_state_05();
        let mut state_05_hs: LinkedHashSet<State> = LinkedHashSet::new();

        for s in state_05 {
            state_05_hs.insert(s);
        }

        vec![state_00_hs, state_01_hs, state_02_hs, state_03_hs, state_04_hs, state_05_hs]
    }

    #[test]
    fn wikipedia_example(){
    // https://en.wikipedia.org/wiki/Earley_parser#Example
        let grammar_str = "
        <P> ::= <S>
        <S> ::= <S> \"+\" <M> | <M>
        <M> ::= <M> \"*\" <T> | <T>
        <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
        ";
        let sentence: String = "2+3*4".to_string();

        let expect = wikipedia_example_states();

        let grammar: Grammar = grammar_str.parse().unwrap();
        let mut eparser = EarleyParser::new(grammar);
        let actual = eparser.earley_parse(sentence).unwrap();

        assert_eq!(expect, actual);
    }
}