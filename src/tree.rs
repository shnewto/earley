use crate::earley::{earley_next_element, State};
use crate::error::Error;
use bnf::{Expression, Grammar, Production, Term};
use linked_hash_set::LinkedHashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Root {
    State(State),
    Term(Term),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Tree {
    root: Root,
    branches: Vec<Tree>,
}

pub fn parse_forest(
    grammar: &Grammar,
    states: Vec<LinkedHashSet<State>>,
) -> Result<Vec<Tree>, Error> {
    let completes = get_complete(&states);
    let preprocessed = preprocess(completes);
    for (i, p) in preprocessed.iter().enumerate() {
        println!("---{}---", i);
        for s in p {
            println!("{:#}", s);
        }
    }

    // let start_rule = grammar
    //     .productions_iter()
    //     .nth(0)
    //     .ok_or(Error::GrammarError("bad start rule in grammar".to_string()))?;
    //
    // let mut trees: Vec<Tree> = vec![];
    //
    // for state in completes
    //     .last()
    //     .ok_or(Error::GrammarError("no complete states".to_string()))?
    //     .iter()
    // {
    //     if state.lhs == Some((start_rule.clone()).lhs) {
    //
    //         trees.push(Tree {
    //             root: Root::State((*state).clone()),
    //             branches: vec![],
    //         });
    //     }
    // }
    //
    // for tree in trees.iter_mut() {
    //     let mut branches = eval(
    //         &tree,
    //         &completes,
    //         completes
    //             .last()
    //             .ok_or(Error::GrammarError("no complete states".to_string()))?,
    //     )?;
    //     tree.branches.append(&mut branches);
    // }
    //
    // Ok(trees)
    Ok(vec![])
}

pub fn preprocess(early_items: Vec<Vec<State>>) -> Vec<Vec<State>> {
    let len = early_items.len();
    let mut ret: Vec<Vec<State>> = vec![vec![]; len];

    for (i, item) in early_items.iter().enumerate() {
        for state in item {
            let mut updated_state = state.clone();
            if let Some(idx) = state.origin {
                updated_state.origin = Some(i);
                if let Some(ret_mut) = ret.get_mut(idx) {
                    ret_mut.push(updated_state);
                }
            }
        }
    }

    ret
}
pub fn eval(
    tree: &Tree,
    states: &Vec<Vec<State>>,
    early_item: &Vec<State>,
) -> Result<Vec<Tree>, Error> {
    let mut ret = tree.clone();
    let items = early_item
        .iter()
        .filter(|i| match (tree.clone()).root {
            Root::State(s) => i.lhs == s.lhs,
            _ => false,
        })
        .collect::<Vec<&State>>();
    let early_items_to_search = early_item
        .iter()
        .filter(|i| match (tree.clone()).root {
            Root::State(s) => i.lhs != s.lhs && i.terms != s.terms,
            _ => true,
        })
        .collect::<Vec<&State>>();

    for production in items {
        for term in production.terms.iter() {
            match term {
                Term::Nonterminal(_) => {
                    for option in &early_items_to_search {
                        if option.lhs == Some((*term).clone()) {
                            if let Some(origin) = option.origin {
                                if origin == 0 {
                                    if let Some(nth) = states.iter().nth(0) {
                                        let mut branches = eval(
                                            &Tree {
                                                root: Root::State((*option).clone()),
                                                branches: vec![],
                                            },
                                            states,
                                            nth,
                                        )?
                                        .clone();

                                        ret.branches.append(&mut branches);
                                    }
                                } else {
                                    if let Some(nth) = states.iter().nth(origin - 1) {
                                        let mut branches = eval(
                                            &Tree {
                                                root: Root::State((*option).clone()),
                                                branches: vec![],
                                            },
                                            states,
                                            nth,
                                        )?
                                        .clone();

                                        ret.branches.append(&mut branches);
                                    }

                                    if let Some(nth) = states.iter().nth(origin) {
                                        let mut branches = eval(
                                            &Tree {
                                                root: Root::State((*option).clone()),
                                                branches: vec![],
                                            },
                                            states,
                                            nth,
                                        )?
                                        .clone();

                                        ret.branches.append(&mut branches);
                                    }
                                }
                            }
                        }
                    }
                }
                Term::Terminal(_) => {
                    ret.branches.push(Tree {
                        root: Root::Term(term.clone()),
                        branches: vec![],
                    });
                }
            }
        }
    }

    Ok(vec![ret])
}

pub fn exprs_in_prod(exprs: Vec<&Expression>, prod: &Production) -> bool {
    let expressions = prod.rhs_iter().collect::<Vec<&Expression>>();
    for expr in exprs.iter() {
        if !expressions.contains(&expr) {
            return false;
        }
    }

    return true;
}

pub fn partial_eq_prod(p1: &Production, p2: &Production) -> bool {
    let left_in_right = |leftprod: &Production, rightprod: &Production| -> bool {
        if leftprod.lhs == rightprod.lhs {
            let rightprod_exprs = rightprod.rhs_iter().collect::<Vec<&Expression>>();
            return exprs_in_prod(rightprod_exprs, leftprod);
        }

        return false;
    };

    left_in_right(p1, p2) || left_in_right(p2, p1)
}

pub fn prod_in_grammar(grammar: &Grammar, waldo: &Production, pos: Option<usize>) -> bool {
    if let Some(n) = pos {
        if let Some(p) = grammar.productions_iter().nth(n) {
            return partial_eq_prod(waldo, p);
        }

        return false;
    }

    for p in grammar.productions_iter() {
        if partial_eq_prod(p, waldo) {
            return true;
        }
    }

    false
}

//
// pub fn parse_forest(grammar: &Grammar, states: Vec<LinkedHashSet<State>>) -> Vec<Node> {
//     get_root_nodes(grammar, &states)
// }
//
// pub fn get_root_nodes(grammar: &Grammar, states: &Vec<LinkedHashSet<State>>) -> Vec<Node> {
//     let completed_states = get_complete(states);
//     let mut roots: Vec<Node> = vec![];
//
//     if let Some(final_states) = final_state_candidates(grammar) {
//         for state in final_states {
//             let mut record: LinkedHashSet<State> = LinkedHashSet::new();
//             if let Some(lhs) = state.lhs {
//                 roots.push(Node {
//                     term: lhs,
//                     leaves: traverse_parse(&completed_states, state.terms, &mut record),
//                 });
//             }
//         }
//     }
//
//     roots
// }
//
fn get_complete(states: &Vec<LinkedHashSet<State>>) -> Vec<Vec<State>> {
    let mut completed_states: Vec<Vec<State>> = vec![];

    for state in states {
        let mut completes: Vec<State> = vec![];
        for prod in state {
            if let None = earley_next_element(&prod) {
                completes.push(prod.clone());
            }
        }

        completed_states.push(completes);
    }

    completed_states
}
//     let mut dot: Option<usize> = None;
//
//     for term in terms.clone() {
//         if let Term::Nonterminal(_) = term {
//             for state_vec in state_vecs.iter().rev() {
//                 for state in state_vec {
//                     if let Some(ref s) = state.lhs {
//                         if *s == term {
//                             if let Some(_) = state.dot {
//                                 if !record.contains(state) {
//                                     dot = state.dot;
//                                     record.insert(state.clone());
//                                     parse.push((state.lhs.clone(), state.terms.clone()));
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         } else if let Term::Terminal(_) = term {
//
//         }
//
//
//     }
//
//     for term in terms.clone() {
//         match term {
//             Term::Nonterminal(_) => {
//                 if let Some(s) = match_final_state_term(&state_vecs, &term, dot, record) {
//                     dot = s.dot;
//                     parse.push((s.lhs.clone(), s.terms.clone()));
//                 }
//             }
//             Term::Terminal(_) => {
//                 parse.push((None, vec![term.clone()]));
//             }
//         }
//     }
//
//     let mut ret: Vec<Node> = vec![];
//     for (lhs, leaves) in parse.iter().rev() {
//         if let Some(t) = lhs {
//             ret.push(Node {
//                 term: t.clone(),
//                 leaves: traverse_parse(state_vecs, leaves.clone(), record),
//             });
//         } else {
//             ret.push(Node {
//                 term: leaves.get(0).unwrap().clone(),
//                 leaves: vec![],
//             });
//         }
//     }
//     ret
// }
//
// fn match_final_state_term(
//     state_vecs: &Vec<Vec<State>>,
//     rule: &Term,
//     dot: Option<usize>,
//     record: &mut LinkedHashSet<State>,
// ) -> Option<State> {
//     if let Some(d) = dot {
//         if let Some(state_vec) = state_vecs.get(d) {
//             for state in state_vec.iter() {
//                 if let Some(ref s) = state.lhs {
//                     if s == rule {
//                         if !record.contains(state) {
//                             return Some(state.clone());
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     None
// }
