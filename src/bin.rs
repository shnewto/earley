extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use bnf::Grammar;
use earley::earley::{EarleyParser, State};
use earley::error::Error;
use earley::tree::parse_forest;
use linked_hash_set::LinkedHashSet;

use serde_json;

fn _print_parse_forest(states: Vec<State>) {
    for _f in states {
        // println!("Successful Parse");
        // let mut curr: Option<State> = Some(f);
        //
        // while let Some(s) = curr {
        //     println!("{:#}", s);
        //     curr = *s.parse_parent;
        // }
    }
}

fn _print_states(grammar: Grammar, states: Vec<LinkedHashSet<State>>) {
    println!("{}", grammar);
    for (i, state) in states.iter().enumerate() {
        println!("\n=== {} ===", i);
        for s in state.iter() {
            println!("{}", s);
        }
    }
}

fn _level_one() -> Result<(), Error> {
    let grammar_str = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";
    let sentence: String = "2+3*4".to_string();

    let grammar: Grammar = grammar_str.parse().unwrap();
    let mut eparser = EarleyParser::new(grammar.clone());
    let states = eparser.earley_parse(sentence)?;
    // _print_states(grammar.clone(), states.clone());
    for parse in parse_forest(&grammar, states) {
        println!("{:#?}", parse);
    }

    Ok(())
}

fn _level_two() -> Result<(), Error> {
    let grammar_str = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let sentence = "1+(2*3-4)".to_string();

    let grammar: Grammar = grammar_str.parse().unwrap();
    let mut eparser = EarleyParser::new(grammar.clone());
    let states = eparser.earley_parse(sentence)?;

    // _print_states(grammar, states);

    // parse_forest(&grammar, states);
    let forest = parse_forest(&grammar, states)?;
    for tree in forest {
        println!("{:#}", serde_json::to_string(&tree).unwrap());
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    // _level_one()
    _level_two()
}
