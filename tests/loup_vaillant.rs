extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use bnf::{Grammar, Term};
use earley::earley::{EarleyParser, State};
use linked_hash_set::LinkedHashSet;

#[test]
fn loup_vaillant_example() {
    // Validate against example documented here https://loup-vaillant.fr/tutorials/earley-parsing/parser

    let grammar_str = "
        <Sum>     ::= <Sum> \"+\" <Product> | <Sum> \"-\" <Product> | <Product>
        <Product> ::= <Product> \"*\" <Factor> | <Product> \"/\" <Factor> | <Factor>
        <Factor>  ::= \"(\" <Sum> \")\" | <Number>
        <Number>  ::= \"0\" <Number> | \"1\" <Number> | \"2\" <Number> | \"3\" <Number> |
                      \"4\" <Number> | \"5\" <Number> | \"6\" <Number> | \"7\" <Number> |
                      \"8\" <Number> | \"9\" <Number>
        <Number>  ::= \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\"
        ";

    let sentence: String = "1+(2*3-4)".to_string();

    let expect = loup_vaillant_example_states();

    let grammar: Grammar = grammar_str.parse().unwrap();
    let mut eparser = EarleyParser::new(grammar);
    let actual = eparser.earley_parse(sentence).unwrap();

    assert_eq!(expect, actual);
}

fn loup_vaillant_example_states() -> Vec<LinkedHashSet<State>> {
    let state_00 = loup_vaillant_example_state_00();
    let mut state_00_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_00 {
        state_00_hs.insert(s);
    }

    let state_01 = loup_vaillant_example_state_01();
    let mut state_01_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_01 {
        state_01_hs.insert(s);
    }

    let state_02 = loup_vaillant_example_state_02();
    let mut state_02_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_02 {
        state_02_hs.insert(s);
    }

    let state_03 = loup_vaillant_example_state_03();
    let mut state_03_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_03 {
        state_03_hs.insert(s);
    }

    let state_04 = loup_vaillant_example_state_04();
    let mut state_04_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_04 {
        state_04_hs.insert(s);
    }

    let state_05 = loup_vaillant_example_state_05();
    let mut state_05_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_05 {
        state_05_hs.insert(s);
    }

    let state_06 = loup_vaillant_example_state_06();
    let mut state_06_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_06 {
        state_06_hs.insert(s);
    }

    let state_07 = loup_vaillant_example_state_07();
    let mut state_07_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_07 {
        state_07_hs.insert(s);
    }

    let state_08 = loup_vaillant_example_state_08();
    let mut state_08_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_08 {
        state_08_hs.insert(s);
    }

    let state_09 = loup_vaillant_example_state_09();
    let mut state_09_hs: LinkedHashSet<State> = LinkedHashSet::new();

    for s in state_09 {
        state_09_hs.insert(s);
    }

    vec![
        state_00_hs,
        state_01_hs,
        state_02_hs,
        state_03_hs,
        state_04_hs,
        state_05_hs,
        state_06_hs,
        state_07_hs,
        state_08_hs,
        state_09_hs,
    ]
}

fn sum_to_sum_plus_prod(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Sum".to_string())),
        terms: vec![
            Term::Nonterminal("Sum".to_string()),
            Term::Terminal("+".to_string()),
            Term::Nonterminal("Product".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn sum_to_sum_sub_prod(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Sum".to_string())),
        terms: vec![
            Term::Nonterminal("Sum".to_string()),
            Term::Terminal("-".to_string()),
            Term::Nonterminal("Product".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn sum_to_sum_plus_sub_prod(origin: usize, dot: usize) -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut sum_to_sum_sub_prod(origin, dot));
    states.append(&mut sum_to_sum_plus_prod(origin, dot));

    states
}

fn sum_to_prod(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Sum".to_string())),
        terms: vec![Term::Nonterminal("Product".to_string())],
        dot: Some(dot),
    }]
}

fn prod_to_prod_mul_factor(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Product".to_string())),
        terms: vec![
            Term::Nonterminal("Product".to_string()),
            Term::Terminal("*".to_string()),
            Term::Nonterminal("Factor".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn prod_to_prod_div_factor(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Product".to_string())),
        terms: vec![
            Term::Nonterminal("Product".to_string()),
            Term::Terminal("/".to_string()),
            Term::Nonterminal("Factor".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn prod_to_prod_mul_div_factor(origin: usize, dot: usize) -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut prod_to_prod_div_factor(origin, dot));
    states.append(&mut prod_to_prod_mul_factor(origin, dot));

    states
}

fn prod_to_factor(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Product".to_string())),
        terms: vec![Term::Nonterminal("Factor".to_string())],
        dot: Some(dot),
    }]
}

fn factor_to_lp_sum_rp(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Factor".to_string())),
        terms: vec![
            Term::Terminal("(".to_string()),
            Term::Nonterminal("Sum".to_string()),
            Term::Terminal(")".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_0(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("0".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_1(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("1".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_2(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("2".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_3(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("3".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_4(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("4".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_5(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("5".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_6(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("6".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_7(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("7".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_8(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("8".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_9(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![Term::Terminal("9".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_0_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("0".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_1_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("1".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_2_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("2".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_3_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("3".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_4_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("4".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_5_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("5".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_6_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("6".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_7_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("7".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_8_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("8".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn number_to_9_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Number".to_string())),
        terms: vec![
            Term::Terminal("9".to_string()),
            Term::Nonterminal("Number".to_string()),
        ],
        dot: Some(dot),
    }]
}

fn factor_to_number(origin: usize, dot: usize) -> Vec<State> {
    vec![State {
        origin: Some(origin),
        lhs: Some(Term::Nonterminal("Factor".to_string())),
        terms: vec![Term::Nonterminal("Number".to_string())],
        dot: Some(dot),
    }]
}

fn number_to_digit(origin: usize, dot: usize) -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut number_to_9(origin, dot));
    states.append(&mut number_to_8(origin, dot));
    states.append(&mut number_to_7(origin, dot));
    states.append(&mut number_to_6(origin, dot));
    states.append(&mut number_to_5(origin, dot));
    states.append(&mut number_to_4(origin, dot));
    states.append(&mut number_to_3(origin, dot));
    states.append(&mut number_to_2(origin, dot));
    states.append(&mut number_to_1(origin, dot));
    states.append(&mut number_to_0(origin, dot));

    states
}

fn number_to_digit_number(origin: usize, dot: usize) -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut number_to_9_number(origin, dot));
    states.append(&mut number_to_8_number(origin, dot));
    states.append(&mut number_to_7_number(origin, dot));
    states.append(&mut number_to_6_number(origin, dot));
    states.append(&mut number_to_5_number(origin, dot));
    states.append(&mut number_to_4_number(origin, dot));
    states.append(&mut number_to_3_number(origin, dot));
    states.append(&mut number_to_2_number(origin, dot));
    states.append(&mut number_to_1_number(origin, dot));
    states.append(&mut number_to_0_number(origin, dot));

    states
}

fn loup_vaillant_example_state_00() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut sum_to_prod(0, 0));
    states.append(&mut prod_to_factor(0, 0));
    states.append(&mut factor_to_number(0, 0));
    states.append(&mut number_to_digit_number(0, 0));
    states.append(&mut number_to_digit(0, 0));
    states.append(&mut factor_to_lp_sum_rp(0, 0));
    states.append(&mut prod_to_prod_mul_div_factor(0, 0));
    states.append(&mut sum_to_sum_plus_sub_prod(0, 0));

    states
}

fn loup_vaillant_example_state_01() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut number_to_1(0, 1));
    states.append(&mut number_to_1_number(0, 1));
    states.append(&mut number_to_digit(1, 0));
    states.append(&mut number_to_digit_number(1, 0));
    states.append(&mut factor_to_number(0, 1));
    states.append(&mut prod_to_factor(0, 1));
    states.append(&mut sum_to_prod(0, 1));
    states.append(&mut prod_to_prod_mul_div_factor(0, 1));
    states.append(&mut sum_to_sum_plus_sub_prod(0, 1));

    states
}

fn loup_vaillant_example_state_02() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut sum_to_sum_plus_prod(0, 2));
    states.append(&mut prod_to_prod_mul_div_factor(2, 0));
    states.append(&mut prod_to_factor(2, 0));
    states.append(&mut factor_to_lp_sum_rp(2, 0));
    states.append(&mut factor_to_number(2, 0));
    states.append(&mut number_to_digit_number(2, 0));
    states.append(&mut number_to_digit(2, 0));

    states
}

fn loup_vaillant_example_state_03() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut factor_to_lp_sum_rp(2, 1));
    states.append(&mut sum_to_sum_plus_sub_prod(3, 0));
    states.append(&mut sum_to_prod(3, 0));
    states.append(&mut prod_to_prod_mul_div_factor(3, 0));
    states.append(&mut prod_to_factor(3, 0));
    states.append(&mut factor_to_lp_sum_rp(3, 0));
    states.append(&mut factor_to_number(3, 0));
    states.append(&mut number_to_digit_number(3, 0));
    states.append(&mut number_to_digit(3, 0));

    states
}

fn loup_vaillant_example_state_04() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut number_to_2_number(3, 1));
    states.append(&mut number_to_2(3, 1));
    states.append(&mut number_to_digit_number(4, 0));
    states.append(&mut number_to_digit(4, 0));
    states.append(&mut factor_to_number(3, 1));
    states.append(&mut prod_to_factor(3, 1));
    states.append(&mut sum_to_prod(3, 1));
    states.append(&mut prod_to_prod_mul_div_factor(3, 1));
    states.append(&mut factor_to_lp_sum_rp(2, 2));
    states.append(&mut sum_to_sum_plus_sub_prod(3, 1));

    states
}

fn loup_vaillant_example_state_05() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut prod_to_prod_mul_factor(3, 2));
    states.append(&mut factor_to_lp_sum_rp(5, 0));
    states.append(&mut factor_to_number(5, 0));
    states.append(&mut number_to_digit_number(5, 0));
    states.append(&mut number_to_digit(5, 0));

    states
}

fn loup_vaillant_example_state_06() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut number_to_3_number(5, 1));
    states.append(&mut number_to_3(5, 1));
    states.append(&mut number_to_digit_number(6, 0));
    states.append(&mut number_to_digit(6, 0));
    states.append(&mut factor_to_number(5, 1));
    states.append(&mut prod_to_prod_mul_factor(3, 3));
    states.append(&mut sum_to_prod(3, 1));
    states.append(&mut prod_to_prod_mul_div_factor(3, 1));
    states.append(&mut factor_to_lp_sum_rp(2, 2));
    states.append(&mut sum_to_sum_plus_sub_prod(3, 1));

    states
}

fn loup_vaillant_example_state_07() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut sum_to_sum_sub_prod(3, 2));
    states.append(&mut prod_to_prod_mul_div_factor(7, 0));
    states.append(&mut prod_to_factor(7, 0));
    states.append(&mut factor_to_lp_sum_rp(7, 0));
    states.append(&mut factor_to_number(7, 0));
    states.append(&mut number_to_digit_number(7, 0));
    states.append(&mut number_to_digit(7, 0));

    states
}

fn loup_vaillant_example_state_08() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut number_to_4_number(7, 1));
    states.append(&mut number_to_4(7, 1));
    states.append(&mut number_to_digit_number(8, 0));
    states.append(&mut number_to_digit(8, 0));
    states.append(&mut factor_to_number(7, 1));
    states.append(&mut prod_to_factor(7, 1));
    states.append(&mut sum_to_sum_sub_prod(3, 3));
    states.append(&mut prod_to_prod_mul_div_factor(7, 1));
    states.append(&mut factor_to_lp_sum_rp(2, 2));
    states.append(&mut sum_to_sum_plus_sub_prod(3, 1));

    states
}

fn loup_vaillant_example_state_09() -> Vec<State> {
    let mut states: Vec<State> = vec![];

    states.append(&mut factor_to_lp_sum_rp(2, 3));
    states.append(&mut prod_to_factor(2, 1));
    states.append(&mut sum_to_sum_plus_prod(0, 3));
    states.append(&mut prod_to_prod_mul_div_factor(2, 1));
    states.append(&mut sum_to_sum_plus_sub_prod(0, 1));

    states
}
