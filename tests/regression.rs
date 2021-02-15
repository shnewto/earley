extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::outcome::EarleyOutcome;
use earley::tree::Tree;
use std::fs;

#[test]
fn wikipedia_parse_forst() {
    let grammar_str = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";

    let sentence = "2+3*4";
    let outcome = EarleyChart::eval(grammar_str, sentence).unwrap();

    if let EarleyOutcome::Accepted(accepted) = outcome {
        let fjson: String = fs::read_to_string("tests/res/wiki_pf.json").unwrap().parse().unwrap();
        let pf: Vec<Tree> = serde_json::from_str(&fjson).unwrap();

        assert_eq!(pf, accepted.parse_forest().unwrap());
    } else {
        assert_eq!("EarleyOutcome::Accepted", "EarleyOutcome::Rejected");
    }

}

#[test]
fn loup_parse_forst() {
    let grammar_str = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let sentence = "1+(2*3-4)";
    let outcome = EarleyChart::eval(grammar_str, sentence).unwrap();

    if let EarleyOutcome::Accepted(accepted) = outcome {
        let fjson: String = fs::read_to_string("tests/res/loup_pf.json").unwrap().parse().unwrap();
        let pf: Vec<Tree> = serde_json::from_str(&fjson).unwrap();

        assert_eq!(pf, accepted.parse_forest().unwrap());
    } else {
        assert_eq!("EarleyOutcome::Accepted", "EarleyOutcome::Rejected");
    }

}

#[test]
fn ambiguous_parse_forst() {
    let grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{' '}'
    <If>         ::=  'i' 'f' <Block>
    <If>         ::=  'i' 'f' <Block> 'e' 'l' 's' 'e' <Block>
    ";

    let sentence = "ifif{}else{}";
    let outcome = EarleyChart::eval(grammar_str, sentence).unwrap();

    if let EarleyOutcome::Accepted(accepted) = outcome {
        let fjson: String = fs::read_to_string("tests/res/ambiguous_pf.json").unwrap().parse().unwrap();
        let pf: Vec<Tree> = serde_json::from_str(&fjson).unwrap();

        assert_eq!(pf, accepted.parse_forest().unwrap());
    } else {
        assert_eq!("EarleyOutcome::Accepted", "EarleyOutcome::Rejected");
    }

}