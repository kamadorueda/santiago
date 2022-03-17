use crate::symbol::Symbol;

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

impl Production {
    pub(crate) fn rules(&self) -> Vec<String> {
        self.terms
            .iter()
            .filter_map(|symbol| match symbol {
                Symbol::Char(_) => None,
                Symbol::Rule(name) => Some(name.clone()),
            })
            .collect()
    }
}
