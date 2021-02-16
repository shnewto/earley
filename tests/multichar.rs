extern crate bnf;
extern crate earley;
extern crate linked_hash_set;

use earley::chart::EarleyChart;
use earley::outcome::EarleyOutcome;

#[test]
fn multichar_terminals_ok() {
    let grammar_str = "
    <S> ::= <N> <VP>
    <VP> ::= <V> <NP>
    <V> ::= 'joined' | 'followed' | 'lost' | 'caught'
    <N> ::= 'Amethyst' | 'Perl' | 'Garnet' | 'Peridot' | 'Stevonnie' | 'Lapis' | 'friend'
    <NP> ::= <D> <N>
    <D> ::= 'their' | 'a'
    ";

    let sentence = "Amethyst joined a friend";

    let outcome = EarleyChart::eval(grammar_str, sentence, Some(' '));

    assert!(outcome.is_ok());

    if let EarleyOutcome::Accepted(accepted) = outcome.unwrap() {
        assert!(accepted.parse_forest().is_ok());
    } else {
        assert_eq!("EarleyOutcome::Accepted", "EarleyOutcome::Rejected");
    }
}
