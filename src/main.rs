mod earley;
mod error;

extern crate bnf;

use bnf::Grammar;
use crate::earley::EarleyParser;
use crate::error::Error;

fn main() -> Result<(), Error>{
    let bnf_str = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"\
    ";
    let sentence: String = "2+3*4".to_string();

    let grammar: Grammar = bnf_str.parse().unwrap();
    let mut eparser = EarleyParser::new(grammar);
    eparser.accept(sentence)?;

    Ok(())
}
