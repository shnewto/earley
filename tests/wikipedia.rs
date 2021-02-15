extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use bnf::Term;
use earley::chart::EarleyChart;
use earley::istate::IState;
use earley::outcome::EarleyOutcome;
use earley::prod::EarleyProd;
use linked_hash_set::LinkedHashSet;

// Begin Wikipedia Example Test
#[test]
fn wikipedia_example() {
    // https://en.wikipedia.org/wiki/Earley_parser#Example
    let grammar_str = "
        <P> ::= <S>
        <S> ::= <S> '+' <M> | <M>
        <M> ::= <M> '*' <T> | <T>
        <T> ::= '1' | '2' | '3' | '4'
        ";

    let sentence = "2+3*4";
    let expected: Vec<LinkedHashSet<IState>> = wikipedia_example_states()
        .iter()
        .map(|state_set| state_set.iter().cloned().collect())
        // .cloned()
        .collect::<Vec<LinkedHashSet<IState>>>();

    let mut actual: Vec<LinkedHashSet<IState>> = vec![];
    if let Ok(EarleyOutcome::Accepted(res)) = EarleyChart::eval(grammar_str, sentence) {
        actual = res.chart;
    }

    assert_eq!(expected, actual);
}

fn wikipedia_example_states() -> Vec<Vec<IState>> {
    let state_00 = wikipedia_example_state_00();
    let mut state_00_hs: Vec<IState> = vec![];

    for s in state_00 {
        state_00_hs.push(s);
    }

    let state_01 = wikipedia_example_state_01();
    let mut state_01_hs: Vec<IState> = vec![];

    for s in state_01 {
        state_01_hs.push(s);
    }

    let state_02 = wikipedia_example_state_02();
    let mut state_02_hs: Vec<IState> = vec![];

    for s in state_02 {
        state_02_hs.push(s);
    }

    let state_03 = wikipedia_example_state_03();
    let mut state_03_hs: Vec<IState> = vec![];

    for s in state_03 {
        state_03_hs.push(s);
    }

    let state_04 = wikipedia_example_state_04();
    let mut state_04_hs: Vec<IState> = vec![];

    for s in state_04 {
        state_04_hs.push(s);
    }

    let state_05 = wikipedia_example_state_05();
    let mut state_05_hs: Vec<IState> = vec![];

    for s in state_05 {
        state_05_hs.push(s);
    }

    vec![
        state_00_hs,
        state_01_hs,
        state_02_hs,
        state_03_hs,
        state_04_hs,
        state_05_hs,
    ]
}

fn p_to_s(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("P".to_string()),
            rhs: vec![Term::Nonterminal("S".to_string())],
            dot,
        },
    }]
}

fn s_to_s_plus_m(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("S".to_string()),
            rhs: vec![
                Term::Nonterminal("S".to_string()),
                Term::Terminal("+".to_string()),
                Term::Nonterminal("M".to_string()),
            ],
            dot,
        },
    }]
}

fn s_to_m(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("S".to_string()),
            rhs: vec![Term::Nonterminal("M".to_string())],
            dot,
        },
    }]
}

fn m_to_m_mul_t(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("M".to_string()),
            rhs: vec![
                Term::Nonterminal("M".to_string()),
                Term::Terminal("*".to_string()),
                Term::Nonterminal("T".to_string()),
            ],
            dot,
        },
    }]
}

fn m_to_t(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("M".to_string()),
            rhs: vec![Term::Nonterminal("T".to_string())],
            dot,
        },
    }]
}

fn t_to_number(origin: usize, dot: usize) -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut t_to_4(origin, dot));
    states.append(&mut t_to_3(origin, dot));
    states.append(&mut t_to_2(origin, dot));
    states.append(&mut t_to_1(origin, dot));

    states
}

fn t_to_1(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("T".to_string()),
            rhs: vec![Term::Terminal("1".to_string())],
            dot,
        },
    }]
}

fn t_to_2(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("T".to_string()),
            rhs: vec![Term::Terminal("2".to_string())],
            dot,
        },
    }]
}

fn t_to_3(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("T".to_string()),
            rhs: vec![Term::Terminal("3".to_string())],
            dot,
        },
    }]
}

fn t_to_4(origin: usize, dot: usize) -> Vec<IState> {
    vec![IState {
        origin,
        prod: EarleyProd {
            lhs: Term::Nonterminal("T".to_string()),
            rhs: vec![Term::Terminal("4".to_string())],
            dot,
        },
    }]
}

fn wikipedia_example_state_00() -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut p_to_s(0, 0));
    states.append(&mut s_to_m(0, 0));
    states.append(&mut m_to_t(0, 0));
    states.append(&mut t_to_number(0, 0));
    states.append(&mut m_to_m_mul_t(0, 0));
    states.append(&mut s_to_s_plus_m(0, 0));

    states
}

fn wikipedia_example_state_01() -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut t_to_2(0, 1));
    states.append(&mut m_to_t(0, 1));
    states.append(&mut m_to_m_mul_t(0, 1));
    states.append(&mut s_to_m(0, 1));
    states.append(&mut s_to_s_plus_m(0, 1));
    states.append(&mut p_to_s(0, 1));

    states
}

fn wikipedia_example_state_02() -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut s_to_s_plus_m(0, 2));
    states.append(&mut m_to_t(2, 0));
    states.append(&mut t_to_number(2, 0));
    states.append(&mut m_to_m_mul_t(2, 0));

    states
}

fn wikipedia_example_state_03() -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut t_to_3(2, 1));
    states.append(&mut m_to_t(2, 1));
    states.append(&mut m_to_m_mul_t(2, 1));
    states.append(&mut s_to_s_plus_m(0, 3));
    states.append(&mut s_to_s_plus_m(0, 1));
    states.append(&mut p_to_s(0, 1));

    states
}

fn wikipedia_example_state_04() -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut m_to_m_mul_t(2, 2));
    states.append(&mut t_to_number(4, 0));

    states
}

fn wikipedia_example_state_05() -> Vec<IState> {
    let mut states: Vec<IState> = vec![];

    states.append(&mut t_to_4(4, 1));
    states.append(&mut m_to_m_mul_t(2, 3));
    states.append(&mut m_to_m_mul_t(2, 1));
    states.append(&mut s_to_s_plus_m(0, 3));
    states.append(&mut s_to_s_plus_m(0, 1));
    states.append(&mut p_to_s(0, 1));

    states
}
