mod earley;
mod error;

extern crate bnf;

use bnf::Grammar;
use crate::earley::Parser;

fn main() {
let bnf_str = "<P> ::= <S>
               <S> ::= <S> \"+\" <M> | <M>
               <M> ::= <M> \"*\" <T> | <T>
               <T> ::= \"1\" | \"2\" | \"3\" | \"4\"";
    let grammar: Grammar = bnf_str.parse().unwrap();
    let eparser = Parser::new(grammar);
    println!("{:#}", eparser);
}
