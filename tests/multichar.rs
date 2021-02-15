extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::outcome::EarleyOutcome;

#[test]
fn multichar_terminals_ok() {
    let singlechar_terminals_grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{' '}'
    <If>         ::=  'i' 'f' <Block>
    <If>         ::=  'i' 'f' <Block> 'e' 'l' 's' 'e' <Block>
    ";

    let multichar_terminals_grammar_str = "
    <Block>      ::=  <If>
    <Block>      ::= '{}'
    <If>         ::=  'if' <Block>
    <If>         ::=  'if' <Block> 'else' <Block>
    ";

    let sentence = "ifif{}else{}";

    let singlechar_outcome = EarleyChart::eval(singlechar_terminals_grammar_str, sentence).unwrap();
    let multicharchar_outcome = EarleyChart::eval(multichar_terminals_grammar_str, sentence).unwrap();

    assert_eq!(singlechar_outcome, multicharchar_outcome);

    if let (EarleyOutcome::Accepted(singlechar_accepted), EarleyOutcome::Accepted(multicharchar_accepted)) = (singlechar_outcome, multicharchar_outcome) {
        assert_eq!(multicharchar_accepted.parse_forest().unwrap(), singlechar_accepted.parse_forest().unwrap());
    } else {
        assert_eq!("EarleyOutcome::Accepted", "EarleyOutcome::Rejected");
    }
}