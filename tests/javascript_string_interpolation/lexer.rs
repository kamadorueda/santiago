use santiago::lexer::LexerBuilder;
use santiago::lexer::LexerRule;

pub fn lexer() -> Vec<LexerRule> {
    LexerBuilder::new()
        // If the current state is "INITIAL",
        // associate a "`" with the beginning of the string,
        // and make the current state be "INSIDE_STRING".
        .string(&["INITIAL"], "STRING_START", "`", |lexer| {
            lexer.push_state("INSIDE_STRING");
            lexer.take()
        })
        // If the current state is "INSIDE_STRING"
        // associate "${" with nothing,
        // make the current state be "INSIDE_STRING_INTERPOLATION"
        // and skip the current match.
        .string(&["INSIDE_STRING"], "", "${", |lexer| {
            lexer.push_state("INSIDE_STRING_INTERPOLATION");
            lexer.skip()
        })
        // If the current state is "INSIDE_STRING_INTERPOLATION"
        // associate one or more latin letters to a variable.
        .pattern(&["INSIDE_STRING_INTERPOLATION"], "VAR", "[a-z]+", |lexer| {
            lexer.take()
        })
        // If the current state is "INSIDE_STRING_INTERPOLATION"
        // associate a "}" with nothing,
        // and skip the current match.
        .string(&["INSIDE_STRING_INTERPOLATION"], "STR", "}", |lexer| {
            lexer.pop_state();
            lexer.skip()
        })
        // If the current state is "INSIDE_STRING",
        // associate a "`" with the end of the string
        // and go back to the previous state.
        .string(&["INSIDE_STRING"], "STRING_END", "`", |lexer| {
            lexer.pop_state();
            lexer.take()
        })
        // If the current state is "INSIDE_STRING"
        // associate anything with a "STR".
        //
        // Note how the "`" in the previous rule takes precedence over this one.
        .pattern(&["INSIDE_STRING"], "STR", ".", |lexer| lexer.take())
        // If the current state is "INITIAL" or "INSIDE_STRING_INTERPOLATION"
        // associate a " " with whitespace, and skip it.
        .string(
            &["INITIAL", "INSIDE_STRING_INTERPOLATION"],
            "WS",
            " ",
            |lexer| lexer.skip(),
        )
        .finish()
}
