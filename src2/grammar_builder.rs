use crate::{rule::Rule, symbol::Symbol};

pub struct GrammarBuilder {
    rules: Vec<Rule>,
}

impl Default for GrammarBuilder {
    fn default() -> GrammarBuilder {
        GrammarBuilder { rules: vec![] }
    }
}

impl GrammarBuilder {
    pub fn add_rule(&mut self, from: &str, to: Vec<Symbol>) {
        let rule_index = self
            .rules
            .iter()
            .filter(|rule| match &rule.from {
                Symbol::NonTerminal(name) => name == from,
                _ => false,
            })
            .count();

        self.rules.push(Rule {
            from: Symbol::NonTerminal(from.to_string()),
            to,
            rule_index,
        });
    }

    pub fn build(&self) -> Vec<Rule> {
        self.rules.clone()
    }
}
