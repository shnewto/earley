// extern crate bnf;
// extern crate earley;
// extern crate linked_hash_set;
//
// use bnf::{Grammar, Term};
// use earley::earley::{EarleyParser, State};
// use linked_hash_set::LinkedHashSet;
//
// // Begin Wikipedia Example Test
// #[test]
// fn wikipedia_example() {
//     // https://en.wikipedia.org/wiki/Earley_parser#Example
//     let grammar_str = "
//         <P> ::= <S>
//         <S> ::= <S> \"+\" <M> | <M>
//         <M> ::= <M> \"*\" <T> | <T>
//         <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
//         ";
//     let sentence: String = "2+3*4".to_string();
//
//     let expect = wikipedia_example_states();
//
//     let grammar: Grammar = grammar_str.parse().unwrap();
//     let mut eparser = EarleyParser::new(grammar);
//     let actual = eparser.earley_parse(sentence).unwrap();
//
//     assert_eq!(expect, actual);
// }
//
// fn wikipedia_example_states() -> Vec<LinkedHashSet<State>> {
//     let state_00 = wikipedia_example_state_00();
//     let mut state_00_hs: LinkedHashSet<State> = LinkedHashSet::new();
//
//     for s in state_00 {
//         state_00_hs.insert(s);
//     }
//
//     let state_01 = wikipedia_example_state_01();
//     let mut state_01_hs: LinkedHashSet<State> = LinkedHashSet::new();
//
//     for s in state_01 {
//         state_01_hs.insert(s);
//     }
//
//     let state_02 = wikipedia_example_state_02();
//     let mut state_02_hs: LinkedHashSet<State> = LinkedHashSet::new();
//
//     for s in state_02 {
//         state_02_hs.insert(s);
//     }
//
//     let state_03 = wikipedia_example_state_03();
//     let mut state_03_hs: LinkedHashSet<State> = LinkedHashSet::new();
//
//     for s in state_03 {
//         state_03_hs.insert(s);
//     }
//
//     let state_04 = wikipedia_example_state_04();
//     let mut state_04_hs: LinkedHashSet<State> = LinkedHashSet::new();
//
//     for s in state_04 {
//         state_04_hs.insert(s);
//     }
//
//     let state_05 = wikipedia_example_state_05();
//     let mut state_05_hs: LinkedHashSet<State> = LinkedHashSet::new();
//
//     for s in state_05 {
//         state_05_hs.insert(s);
//     }
//
//     vec![
//         state_00_hs,
//         state_01_hs,
//         state_02_hs,
//         state_03_hs,
//         state_04_hs,
//         state_05_hs,
//     ]
// }
//
// fn p_to_s(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("P".to_string())),
//         terms: vec![Term::Nonterminal("S".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn s_to_s_plus_m(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("S".to_string())),
//         terms: vec![
//             Term::Nonterminal("S".to_string()),
//             Term::Terminal("+".to_string()),
//             Term::Nonterminal("M".to_string()),
//         ],
//         dot: Some(dot),
//     }]
// }
//
// fn s_to_m(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("S".to_string())),
//         terms: vec![Term::Nonterminal("M".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn m_to_m_mul_t(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("M".to_string())),
//         terms: vec![
//             Term::Nonterminal("M".to_string()),
//             Term::Terminal("*".to_string()),
//             Term::Nonterminal("T".to_string()),
//         ],
//         dot: Some(dot),
//     }]
// }
//
// fn m_to_t(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("M".to_string())),
//         terms: vec![Term::Nonterminal("T".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn t_to_number(origin: usize, dot: usize) -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut t_to_4(origin, dot));
//     states.append(&mut t_to_3(origin, dot));
//     states.append(&mut t_to_2(origin, dot));
//     states.append(&mut t_to_1(origin, dot));
//
//     states
// }
//
// fn t_to_1(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("T".to_string())),
//         terms: vec![Term::Terminal("1".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn t_to_2(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("T".to_string())),
//         terms: vec![Term::Terminal("2".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn t_to_3(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("T".to_string())),
//         terms: vec![Term::Terminal("3".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn t_to_4(origin: usize, dot: usize) -> Vec<State> {
//     vec![State {
//         origin: Some(origin),
//         lhs: Some(Term::Nonterminal("T".to_string())),
//         terms: vec![Term::Terminal("4".to_string())],
//         dot: Some(dot),
//     }]
// }
//
// fn wikipedia_example_state_00() -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut p_to_s(0, 0));
//     states.append(&mut s_to_m(0, 0));
//     states.append(&mut m_to_t(0, 0));
//     states.append(&mut t_to_number(0, 0));
//     states.append(&mut m_to_m_mul_t(0, 0));
//     states.append(&mut s_to_s_plus_m(0, 0));
//
//     states
// }
//
// fn wikipedia_example_state_01() -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut t_to_2(0, 1));
//     states.append(&mut m_to_t(0, 1));
//     states.append(&mut m_to_m_mul_t(0, 1));
//     states.append(&mut s_to_m(0, 1));
//     states.append(&mut s_to_s_plus_m(0, 1));
//     states.append(&mut p_to_s(0, 1));
//
//     states
// }
//
// fn wikipedia_example_state_02() -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut s_to_s_plus_m(0, 2));
//     states.append(&mut m_to_t(2, 0));
//     states.append(&mut t_to_number(2, 0));
//     states.append(&mut m_to_m_mul_t(2, 0));
//
//     states
// }
//
// fn wikipedia_example_state_03() -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut t_to_3(2, 1));
//     states.append(&mut m_to_t(2, 1));
//     states.append(&mut m_to_m_mul_t(2, 1));
//     states.append(&mut s_to_s_plus_m(0, 3));
//     states.append(&mut s_to_s_plus_m(0, 1));
//     states.append(&mut p_to_s(0, 1));
//
//     states
// }
//
// fn wikipedia_example_state_04() -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut m_to_m_mul_t(2, 2));
//     states.append(&mut t_to_number(4, 0));
//
//     states
// }
//
// fn wikipedia_example_state_05() -> Vec<State> {
//     let mut states: Vec<State> = vec![];
//
//     states.append(&mut t_to_4(4, 1));
//     states.append(&mut m_to_m_mul_t(2, 3));
//     states.append(&mut m_to_m_mul_t(2, 1));
//     states.append(&mut s_to_s_plus_m(0, 3));
//     states.append(&mut s_to_s_plus_m(0, 1));
//     states.append(&mut p_to_s(0, 1));
//
//     states
// }
