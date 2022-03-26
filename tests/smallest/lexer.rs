use santiago::lexer::LexerBuilder;
use santiago::lexer::LexerRule;

pub fn lexer() -> Vec<LexerRule> {
    LexerBuilder::new()
        .pattern(&["INITIAL"], "CHAR", ".", |lexer| lexer.take())
        .finish()
}
