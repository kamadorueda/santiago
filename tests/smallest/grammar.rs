use santiago::grammar::Grammar;
use santiago::grammar::GrammarBuilder;

pub fn grammar() -> Grammar {
    GrammarBuilder::new()
        // A rule for 0 characters
        .rule_to_rules("chars", &[])
        // A rule that maps to itself plus one character (recursion)
        .rule_to_rules("chars", &["chars", "char"])
        // A char comes from the lexeme "CHAR"
        .rule_to_lexemes("char", &["CHAR"])
        .finish()
}
