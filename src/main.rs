mod earley;
mod error;

extern crate bnf;
extern crate linked_hash_set;

use bnf::Grammar;
use crate::earley::EarleyParser;
use crate::error::Error;

fn level_one() -> Result<(), Error> {
    let grammar_str = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";
    let sentence: String = "2+3*4".to_string();

    let grammar: Grammar = grammar_str.parse().unwrap();
    let mut eparser = EarleyParser::new(grammar);
    eparser.accept(sentence)?;

    Ok(())
}

fn level_two() -> Result<(), Error> {
    let grammar_str = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let sentence = "1+(2*3-4)".to_string();

    let grammar: Grammar = grammar_str.parse().unwrap();
    let mut eparser = EarleyParser::new(grammar);
    eparser.accept(sentence)?;

    Ok(())
}

fn main() -> Result<(), Error>{
    level_one();
    // level_two();

    Ok(())
}
