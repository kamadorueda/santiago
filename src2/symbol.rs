use crate::lexeme_kind::LexemeKind;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Symbol {
    Terminal(LexemeKind),
    NonTerminal(String),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Terminal(lexeme_kind) => write!(f, "{lexeme_kind:?}"),
            Symbol::NonTerminal(name) => write!(f, "{name:?}"),
        }
    }
}
