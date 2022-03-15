use crate::symbol::Symbol;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Rule {
    pub from:       Symbol,
    pub to:         Vec<Symbol>,
    pub rule_index: usize,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}] ::= {}",
            self.from,
            self.rule_index,
            self.to
                .iter()
                .map(|symbol| format!("{symbol}"))
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}
