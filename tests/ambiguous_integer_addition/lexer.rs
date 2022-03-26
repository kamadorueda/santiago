use santiago::lexer::LexerBuilder;
use santiago::lexer::LexerRule;

pub fn lexer() -> Vec<LexerRule> {
    LexerBuilder::new()
        // One more sequential digits from 0 to 9 will be mapped to an "INT"
        .pattern(&["INITIAL"], "INT", r"[0-9]+", |lexer| lexer.take())
        // A literal "+" will be mapped to "PLUS"
        .string(&["INITIAL"], "PLUS", "+", |lexer| lexer.take())
        // Whitespace " " will be skipped
        .pattern(&["INITIAL"], "WS", r"\s", |lexer| lexer.skip())
        .finish()
}
