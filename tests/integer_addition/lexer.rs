use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // One more sequential digits from 0 to 9 will be mapped to an "INT"
        "DEFAULT" | "INT" = pattern r"[0-9]+";
        // A literal "+" will be mapped to "PLUS"
        "DEFAULT" | "PLUS" = string "+";
        // Whitespace " " will be skipped
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}
