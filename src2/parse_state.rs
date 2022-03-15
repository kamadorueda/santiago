use crate::rule::Rule;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ParseState {
    pub rule:             Rule,
    pub dot_index:        usize,
    pub end_lexeme_index: usize,
}

impl ParseState {
    pub fn advance(&self) -> ParseState {
        if self.completed() {
            self.clone()
        } else {
            ParseState { dot_index: self.dot_index + 1, ..self.clone() }
        }
    }

    pub fn completed(&self) -> bool {
        self.dot_index >= self.rule.to.len()
    }
}

impl std::fmt::Display for ParseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}] ::= {} • {} @{:?}",
            self.rule.from,
            self.rule.rule_index,
            self.rule.to[..self.dot_index]
                .iter()
                .map(|symbol| format!("{symbol}"))
                .collect::<Vec<String>>()
                .join(" "),
            self.rule.to[self.dot_index..]
                .iter()
                .map(|symbol| format!("{symbol}"))
                .collect::<Vec<String>>()
                .join(" "),
            self.end_lexeme_index,
        )
    }
}
