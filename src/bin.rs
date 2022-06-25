extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::error::Error;

fn dna() -> Result<(), Error> {
    let grammar_str = "
    <dna> ::= <base> | <dna> <base>
    <base> ::= \"A\" | \"C\" | \"G\" | \"T\"
    ";

    let sentence = "GATTACA";
    let outcome = EarleyChart::eval(grammar_str, sentence, None)?;

    println!("{}", outcome);
    
    Ok(())
}

fn main() -> Result<(), Error> {
    dna()
}
