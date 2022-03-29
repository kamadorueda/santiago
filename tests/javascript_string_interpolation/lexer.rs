use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // If the current state is "INITIAL",
        // associate a "`" with the beginning of the string,
        // and make the current state be "INSIDE_STRING".
        | "STRING_START" = string "`" => |lexer| {
            lexer.push_state("INSIDE_STRING");
            lexer.take()
        };
        // If the current state is "INSIDE_STRING"
        // associate "${" with nothing,
        // make the current state be "INSIDE_STRING_INTERPOLATION"
        // and skip the current match.
        "INSIDE_STRING" | "" = string "${" => |lexer| {
            lexer.push_state("INSIDE_STRING_INTERPOLATION");
            lexer.skip()
        };
        // If the current state is "INSIDE_STRING_INTERPOLATION"
        // associate one or more latin letters to a variable.
        "INSIDE_STRING_INTERPOLATION" | "VAR" = pattern "[a-z]+";
        // If the current state is "INSIDE_STRING_INTERPOLATION"
        // associate a "}" with nothing,
        // and skip the current match.
        "INSIDE_STRING_INTERPOLATION" | "STR" = string "}" => |lexer| {
            lexer.pop_state();
            lexer.skip()
        };
        // If the current state is "INSIDE_STRING",
        // associate a "`" with the end of the string
        // and go back to the previous state.
        "INSIDE_STRING" | "STRING_END" = string "`" => |lexer| {
            lexer.pop_state();
            lexer.take()
        };
        // If the current state is "INSIDE_STRING"
        // associate anything with a "STR".
        //
        // Note how the "`" in the previous rule takes precedence over this one.
        "INSIDE_STRING" | "STR" = pattern ".";
        // If the current state is "INITIAL" or "INSIDE_STRING_INTERPOLATION"
        // associate a " " with whitespace, and skip it.
        "INITIAL" "INSIDE_STRING_INTERPOLATION" | "WS" = string " " => |lexer| {
            lexer.skip()
        };
    )
}
