extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use bnf::Grammar;
use earley::earley::EarleyParser;
use earley::error::Error;

fn _level_one() -> Result<(), Error> {
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

fn _level_two() -> Result<(), Error> {
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

fn main() -> Result<(), Error> {
    _level_one()
    // level_two();
}
