use santiago::grammar::Associativity;
use santiago::grammar::Grammar;
use santiago::grammar::GrammarBuilder;

pub fn grammar() -> Grammar {
    GrammarBuilder::new()
        // Map the rule "sum" to the sequence of rules "sum", "plus", and "sum"
        .rule_to_rules("sum", &["sum", "plus", "sum"])
        // Map the rule "sum" to the lexeme "INT"
        .rule_to_lexemes("sum", &["INT"])
        // Map the rule "plus" to the lexeme "PLUS"
        .rule_to_lexemes("plus", &["PLUS"])
        // Specify that we want the "plus" rule to be left-associative.
        // In other words: `a+b+c` should group as `(a+b)+c`.
        .disambiguate(Associativity::Left, &["plus"])
        .finish()
}
