use santiago::lexer::LexerBuilder;
use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    LexerBuilder::new()
        .pattern(&["INITIAL"], "CHAR", ".", |lexer| lexer.take())
        .finish()
}
