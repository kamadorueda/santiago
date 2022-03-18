use crate::grammar::{production::Production, symbol::Symbol};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) struct State {
    pub(crate) name:         String,
    pub(crate) production:   Production,
    pub(crate) dot_index:    usize,
    pub(crate) start_column: usize,
    pub(crate) end_column:   usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut terms: Vec<String> =
            self.production.terms.iter().map(Symbol::to_string).collect();
        terms.insert(self.dot_index, "•".to_string());

        write!(
            f,
            "{} := {} [{}-{}]",
            self.name,
            terms.join(" "),
            self.start_column,
            self.end_column,
        )
    }
}

impl State {
    pub(crate) fn completed(&self) -> bool {
        self.dot_index >= self.production.terms.len()
    }

    pub(crate) fn next_term(&self) -> Option<Symbol> {
        if self.completed() {
            None
        } else {
            Some(self.production.terms[self.dot_index].clone())
        }
    }

    pub(crate) fn hash_me(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
