// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use super::production::Production;
use super::rule::Rule;
use super::symbol::Symbol;

pub struct Builder {
    grammar: Vec<Rule>,
}

impl Default for Builder {
    fn default() -> Builder {
        Builder::new()
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder { grammar: Vec::new() }
    }

    fn map_rule_to_terms(&mut self, name: &str, terms: Vec<Symbol>) {
        let name = name.to_string();
        let production = Production { terms };

        match self.grammar.iter().position(|rule| rule.name == name) {
            Some(index) => {
                self.grammar[index].productions.push(production);
            }
            None => {
                self.grammar.push(Rule { name, productions: vec![production] });
            }
        }
    }

    pub fn map_to_lexemes(
        &mut self,
        name: &str,
        rules: &[&str],
    ) -> &mut Builder {
        self.map_rule_to_terms(
            name,
            rules.iter().map(|name| Symbol::Lexeme(name.to_string())).collect(),
        );

        self
    }

    pub fn map_to_rules(&mut self, name: &str, rules: &[&str]) -> &mut Builder {
        self.map_rule_to_terms(
            name,
            rules.iter().map(|name| Symbol::Rule(name.to_string())).collect(),
        );

        self
    }

    pub fn finish(&self) -> Vec<Rule> {
        self.grammar.clone()
    }
}
