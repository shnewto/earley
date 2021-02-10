extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use bnf::Grammar;
use earley::earley::{EarleyChart, EarleyOutcome};
use earley::error::Error;

fn _print_chart(grammar: Grammar, outcome: EarleyOutcome) {
    println!("{}", grammar);

    if let EarleyOutcome::Accepted(accepted) = outcome {
        for (i, states) in accepted.chart.iter().enumerate() {
            println!("\n=== {} ===", i);
            for state in states.iter() {
                println!("{}", state);
            }
        }
    } else {
        println!("input was rejected");
    }
}

fn _print_parse_forest(outcome: &EarleyOutcome) {
    if let EarleyOutcome::Accepted(accepted) = outcome {
       accepted.parse_forest();
    } else {
        println!("input was rejected");
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
    _print_chart(grammar_str.parse().unwrap(), chart);
    Ok(())
}

fn _level_two() -> Result<(), Error> {
    let grammar_str = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let sentence = "1+(2*3-4)";
    let outcome = EarleyChart::eval(grammar_str, sentence)?;

    _print_parse_forest(&outcome);
    // _print_chart(grammar_str.parse().unwrap(), outcome);

    Ok(())
}

fn main() -> Result<(), Error> {
    // _level_one()
    _level_two()
}
