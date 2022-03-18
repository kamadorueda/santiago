#[derive(Clone, Eq, Hash, PartialEq)]
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
