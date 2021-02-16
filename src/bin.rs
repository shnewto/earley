extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::error::Error;
use earley::outcome::EarleyOutcome;
use earley::tree::Tree;
use std::fs::File;
use std::io::Write;

fn wiki(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(());
    }

    let grammar_str = "
    <P> ::= <S>
    <S> ::= <S> \"+\" <M> | <M>
    <M> ::= <M> \"*\" <T> | <T>
    <T> ::= \"1\" | \"2\" | \"3\" | \"4\"
    ";

    let sentence = "2+3*4";
    let outcome = EarleyChart::eval(grammar_str, sentence, None)?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            serialize(
                do_save,
                "./tests/res/wiki_pf.json",
                &accepted.parse_forest()?,
            );
        }

        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}

fn lv(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(());
    }

    let grammar_str = "
    <Sum> ::= <Sum> '+' <Product> | <Sum> '-' <Product> | <Product>
    <Product> ::= <Product> '*' <Factor> | <Product> '/' <Factor> | <Factor>
    <Factor> ::= '(' <Sum> ')' | <Number>
    <Number> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ";

    let sentence = "1+(2*3-4)";
    let outcome = EarleyChart::eval(grammar_str, sentence, None)?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            serialize(
                do_save,
                "./tests/res/loup_pf.json",
                &accepted.parse_forest()?,
            );
        }

        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}

fn if_else_single_chars(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(());
    }

    let grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{' '}'
    <If>         ::=  'i' 'f' <Block>
    <If>         ::=  'i' 'f' <Block> 'e' 'l' 's' 'e' <Block>
    ";

    let sentence = "ifif{}else{}";
    let outcome = EarleyChart::eval(grammar_str, sentence, None)?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            serialize(
                do_save,
                "./tests/res/ambiguous_pf.json",
                &accepted.parse_forest()?,
            );
        }

        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}

fn if_else_multi_chars(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(());
    }

    let grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{}'
    <If>         ::=  'if' <Block>
    <If>         ::=  'if' <Block> 'else' <Block>
    ";

    let sentence = "if if {} else {}";
    let outcome = EarleyChart::eval(grammar_str, sentence, Some(' '))?;

    if let EarleyOutcome::Accepted(accepted) = outcome {
        if do_save {
            serialize(
                do_save,
                "./tests/res/ambiguous_pf.json",
                &accepted.parse_forest()?,
            );
        }

        for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
            println!("=== PT ({}) ===\n{}", i, pf);
        }
    }

    Ok(())
}

fn constituency(do_save: bool, do_run: bool) -> Result<(), Error> {
    if !do_run {
        return Ok(());
    }

    let grammar_str = "
    <S> ::= <N> <VP>
    <VP> ::= <V> <NP>
    <V> ::= 'joined' | 'followed' | 'lost' | 'caught'
    <N> ::= 'Amethyst' | 'Perl' | 'Garnet' | 'Peridot' | 'Stevonnie' | 'Lapis' | 'friend'
    <NP> ::= <D> <N>
    <D> ::= 'their' | 'a'
    ";

    let sentence = "Amethyst joined a friend";

    let outcome = EarleyChart::eval(grammar_str, sentence, Some(' '))?;

    match outcome {
        EarleyOutcome::Accepted(accepted) => {
            if do_save {
                serialize(
                    do_save,
                    "./tests/res/ambiguous_multichar_pf.json",
                    &accepted.parse_forest()?,
                );
            }

            for (i, pf) in accepted.parse_forest()?.iter().enumerate() {
                println!("=== PT ({}) ===\n{}", i, pf);
            }
        }
        EarleyOutcome::Rejected => println!("Input rejected"),
    }

    Ok(())
}

fn serialize(do_save: bool, fname: &str, forest: &[Tree]) {
    if do_save {
        let json_pf = serde_json::to_string(forest).unwrap();
        let mut fout = File::create(fname).unwrap();
        let _ = fout.write(json_pf.as_bytes());
    }
}

fn main() -> Result<(), Error> {
    let do_save = false;
    let _ = wiki(do_save, true);
    let _ = lv(do_save, true);
    let _ = if_else_single_chars(do_save, true);
    let _ = if_else_multi_chars(do_save, true);
    let _ = constituency(do_save, false);
    Ok(())
}
