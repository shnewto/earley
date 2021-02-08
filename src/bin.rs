extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use bnf::Grammar;
use earley::earley::{EarleyChart, State};
use earley::error::Error;

fn print_chart(grammar: Grammar, chart: Vec<Vec<State>>) {
    println!("{}", grammar);
    for (i, states) in chart.iter().enumerate() {
        println!("\n=== {} ===", i);
        for state in states.iter() {
            println!("{}", state);
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
    let sentence = "2+3*4";

    let chart = EarleyChart::eval(grammar_str, sentence)?;

    print_chart(grammar_str.parse().unwrap(), chart);

    Ok(())
}

fn _level_two() -> Result<(), Error> {
    // let grammar_str = "
    // <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    // <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    // <Factor> ::= '(' <Sum> ')' | <Number>
    // <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    // ";
    //
    // let sentence = "1+(2*3-4)".to_string();
    //
    // let grammar: Grammar = grammar_str.parse().unwrap();
    // let mut eparser = EarleyParser::new(grammar.clone());
    // let states = eparser.earley_parse(sentence)?;
    //
    // print_states(grammar, states);

    Ok(())
}

fn main() -> Result<(), Error> {
    _level_one()
    // _level_two()
}
