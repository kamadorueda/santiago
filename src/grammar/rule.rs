use crate::grammar::production::Production;
use std::hash::Hasher;

#[derive(Clone)]
pub struct Rule {
    pub name:        String,
    pub productions: Vec<Production>,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} := {}",
            self.name,
            self.productions
                .iter()
                .map(Production::to_string)
                .collect::<Vec<String>>()
                .join(" | ")
        )
    }
}

impl std::hash::Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
