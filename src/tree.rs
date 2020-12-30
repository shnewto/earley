use crate::earley::{earley_next_element, final_state_candidates, Node, State};
use bnf::{Grammar, Term};
use linked_hash_set::LinkedHashSet;

pub fn parse_forest(grammar: &Grammar, states: Vec<LinkedHashSet<State>>) -> Vec<Node> {
    get_root_nodes(grammar, &states)
}

pub fn get_root_nodes(grammar: &Grammar, states: &Vec<LinkedHashSet<State>>) -> Vec<Node> {
    let completed_states = get_complete(states);
    let mut roots: Vec<Node> = vec![];

    if let Some(final_states) = final_state_candidates(grammar) {
        for state in final_states {
            let mut record: LinkedHashSet<State> = LinkedHashSet::new();
            if let Some(lhs) = state.lhs {
                roots.push(Node {
                    term: lhs,
                    leaves: traverse_parse(&completed_states, state.terms, &mut record),
                });
            }
        }
    }

    roots
}

fn get_complete(states: &Vec<LinkedHashSet<State>>) -> Vec<Vec<State>> {
    let mut completed_states: Vec<Vec<State>> = vec![];

    for state in states {
        let mut completes: Vec<State> = vec![];
        for prod in state {
            if let None = earley_next_element(&prod) {
                completes.push(prod.clone());
            }
        }

        if !completes.is_empty() {
            completed_states.push(completes);
        }
    }

    completed_states
}

fn traverse_parse(
    state_vecs: &Vec<Vec<State>>,
    terms: Vec<Term>,
    record: &mut LinkedHashSet<State>,
) -> Vec<Node> {
    let mut parse: Vec<(Option<Term>, Vec<Term>)> = vec![];
    let mut dot: Option<usize> = None;

    for term in terms.clone() {
        if let Term::Nonterminal(_) = term {
            for state_vec in state_vecs.iter().rev() {
                for state in state_vec {
                    if let Some(ref s) = state.lhs {
                        if *s == term {
                            if let Some(_) = state.dot {
                                if !record.contains(state) {
                                    dot = state.dot;
                                    record.insert(state.clone());
                                    parse.push((state.lhs.clone(), state.terms.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for term in terms.clone() {
        match term {
            Term::Nonterminal(_) => {
                if let Some(s) = match_final_state_term(&state_vecs, &term, dot, record) {
                    dot = s.dot;
                    parse.push((s.lhs.clone(), s.terms.clone()));
                }
            }
            Term::Terminal(_) => {
                parse.push((None, vec![term.clone()]));
            }
        }
    }

    let mut ret: Vec<Node> = vec![];
    for (lhs, leaves) in parse.iter().rev() {
        if let Some(t) = lhs {
            ret.push(Node {
                term: t.clone(),
                leaves: traverse_parse(state_vecs, leaves.clone(), record),
            });
        } else {
            ret.push(Node {
                term: leaves.get(0).unwrap().clone(),
                leaves: vec![],
            });
        }
    }
    ret
}

fn match_final_state_term(
    state_vecs: &Vec<Vec<State>>,
    rule: &Term,
    dot: Option<usize>,
    record: &mut LinkedHashSet<State>,
) -> Option<State> {
    if let Some(d) = dot {
        if let Some(state_vec) = state_vecs.get(d) {
            for state in state_vec.iter() {
                if let Some(ref s) = state.lhs {
                    if s == rule {
                        if !record.contains(state) {
                            return Some(state.clone());
                        }
                    }
                }
            }
        }
    }

    None
}
