extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::error::Error;
use earley::outcome::EarleyOutcome;
use std::fs::File;
use std::io::Write;

// fn _print_flipped_completed_chart(grammar: Grammar, outcome: EarleyOutcome) {
//     println!("{}", grammar);
//
//     if let EarleyOutcome::Accepted(accepted) = outcome {
//         for (i, states) in accepted.flip_completed().iter().enumerate() {
//             println!("\n=== {} ===", i);
//             for state in states.iter() {
//                 println!("{}", state);
//             }
//         }
//     } else {
//         println!("input was rejected");
//     }
// }

fn _print_parse_forest(outcome: &EarleyOutcome) {
    if let EarleyOutcome::Accepted(accepted) = outcome {
        let tj = serde_json::to_string(&accepted.parse_forest().unwrap()).unwrap();
        let mut file = File::create("parse_forest.json").unwrap();
        let _ = file.write(tj.as_bytes());
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
    let outcome = EarleyChart::eval(grammar_str, sentence)?;
    println!("{}", outcome);

    if let EarleyOutcome::Accepted(accepted) = outcome {
        println!("{}", accepted.parse_forest()?[0]);
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

    let sentence = "1+(2*3-4)";
    let outcome = EarleyChart::eval(grammar_str, sentence)?;
    // println!("{}", outcome);

    if let EarleyOutcome::Accepted(accepted) = outcome {
        println!("{}", accepted.parse_forest()?[0]);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    // _level_one()
    _level_two()
}
