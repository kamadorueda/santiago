use santiago::grammar::Grammar;
use santiago::grammar::GrammarBuilder;

pub fn grammar() -> Grammar {
    GrammarBuilder::new()
        .rule_to_rules("string", &["string_start", "str_content", "string_end"])
        // Either empty
        .rule_to_rules("str_content", &[])
        // Or followed by a "str"
        .rule_to_rules("str_content", &["str_content", "str"])
        // Or followed by a "var"
        .rule_to_rules("str_content", &["str_content", "var"])
        // Map rules to their corresponding Lexemes
        .rule_to_lexemes("str", &["STR"])
        .rule_to_lexemes("string_start", &["STRING_START"])
        .rule_to_lexemes("string_end", &["STRING_END"])
        .rule_to_lexemes("var", &["VAR"])
        .finish()
}
