use std::hash::Hasher;

#[derive(Clone, Eq, PartialEq)]
pub enum Symbol {
    Char(char),
    Rule(String),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Char(char) => write!(f, "{char:?}"),
            Symbol::Rule(rule) => write!(f, "{rule}"),
        }
    }
}

impl std::hash::Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Symbol::Char(char) => char.hash(state),
            Symbol::Rule(rule) => rule.hash(state),
        }
    }
}
