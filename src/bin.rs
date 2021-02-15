extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::error::Error;
use earley::outcome::EarleyOutcome;
use std::fs::File;
use std::io::Write;

fn wiki(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(())
    }

    let grammar_str = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";

    let sentence = "2+3*4";
    let outcome = EarleyChart::eval(grammar_str, sentence)?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            let json_pf = serde_json::to_string(&accepted.parse_forest()?).unwrap();
            let mut fout = File::create("./tests/res/wiki_pf.json").unwrap();
            let _ = fout.write(json_pf.as_bytes());
        }

        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}

fn lv(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(())
    }

    let grammar_str = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let sentence = "1+(2*3-4)";
    let outcome = EarleyChart::eval(grammar_str, sentence)?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            let json_pf = serde_json::to_string(&accepted.parse_forest()?).unwrap();
            let mut fout = File::create("./tests/res/loup_pf.json").unwrap();
            let _ = fout.write(json_pf.as_bytes());
        }


        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}

fn if_else_chars(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(())
    }

    let grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{' '}'
    <If>         ::=  'i' 'f' <Block>
    <If>         ::=  'i' 'f' <Block> 'e' 'l' 's' 'e' <Block>
    ";

    let sentence = "ifif{}else{}";
    let outcome = EarleyChart::eval(grammar_str, sentence)?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            let json_pf = serde_json::to_string(&accepted.parse_forest()?).unwrap();
            let mut fout = File::create("./tests/res/ambiguous_pf.json").unwrap();
            let _ = fout.write(json_pf.as_bytes());
        }

        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}


fn if_else_words(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(())
    }

    let grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{}'
    <If>         ::=  'if' <Block>
    <If>         ::=  'if' <Block> 'else' <Block>
    ";

    let sentence = "ifif{}else{}";
    let outcome = EarleyChart::eval(grammar_str, sentence)?;

    match outcome {
        EarleyOutcome::Accepted(accepted) => {
            if do_save {
                let json_pf = serde_json::to_string(&accepted.parse_forest()?).unwrap();
                let mut fout = File::create("./tests/res/ambiguous_pf.json").unwrap();
                let _ = fout.write(json_pf.as_bytes());
            }

            for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
                println!("=== PT ({}) ===\n{}", i, pf);
            }
        },
        EarleyOutcome::Rejected => println!("Input rejected"),
    }

    Ok(())
}


fn main() -> Result<(), Error> {
    let do_save = false;
    let _ = wiki(do_save, false);
    let _ = lv(do_save, false);
    let _ = if_else_chars(do_save, false);
    let _ = if_else_words(do_save, true);
    Ok(())
}
