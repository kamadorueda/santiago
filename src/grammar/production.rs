use crate::grammar::symbol::Symbol;

use std;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Production {
    pub terms: Vec<Symbol>,
}

impl std::fmt::Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.terms
                .iter()
                .map(Symbol::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
