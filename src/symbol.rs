use std::hash::Hasher;

#[derive(Clone, Eq, PartialEq)]
pub enum Symbol {
    Lexeme(String),
    Rule(String),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Lexeme(raw) => write!(f, "{raw:?}"),
            Symbol::Rule(rule) => write!(f, "{rule}"),
        }
    }
}

impl std::hash::Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Symbol::Lexeme(raw) => raw.hash(state),
            Symbol::Rule(rule) => rule.hash(state),
        }
    }
}
