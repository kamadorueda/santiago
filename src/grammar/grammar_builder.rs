// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use super::grammar_rule::GrammarRule;
use super::Production;
use super::Symbol;

/// Utility for creating [grammar rules](GrammarRule).
///
/// Please read the [module documentation](crate::grammar) for more information and examples.
pub struct GrammarBuilder {
    grammar: Vec<GrammarRule>,
}

impl Default for GrammarBuilder {
    fn default() -> GrammarBuilder {
        GrammarBuilder::new()
    }
}

impl GrammarBuilder {
    /// Creates a new [GrammarBuilder] with no rules.
    pub fn new() -> GrammarBuilder {
        GrammarBuilder { grammar: Vec::new() }
    }

    fn rule_to_symbols(&mut self, name: &str, symbols: Vec<Symbol>) {
        let name = name.to_string();
        let production = Production { symbols };

        match self.grammar.iter().position(|rule| rule.name == name) {
            Some(index) => {
                self.grammar[index].productions.push(production);
            }
            None => {
                self.grammar
                    .push(GrammarRule { name, productions: vec![production] });
            }
        }
    }

    /// Map a rule with name `name` to zero or more [crate::lexer::LexerRule].
    pub fn rule_to_lexemes(
        &mut self,
        kind: &str,
        lexeme_kinds: &[&str],
    ) -> &mut GrammarBuilder {
        self.rule_to_symbols(
            kind,
            lexeme_kinds
                .iter()
                .map(|kind| Symbol::Lexeme(kind.to_string()))
                .collect(),
        );

        self
    }

    /// Map a rule with name `name` to zero or more [GrammarRule].
    pub fn rule_to_rules(
        &mut self,
        name: &str,
        rule_names: &[&str],
    ) -> &mut GrammarBuilder {
        self.rule_to_symbols(
            name,
            rule_names
                .iter()
                .map(|name| Symbol::Rule(name.to_string()))
                .collect(),
        );

        self
    }

    pub fn finish(&self) -> Vec<GrammarRule> {
        for grammar_rule in &self.grammar {
            for production in &grammar_rule.productions {
                for symbol in &production.symbols {
                    match &symbol {
                        Symbol::Rule(name) => {
                            if !self
                                .grammar
                                .iter()
                                .any(|grammar_rule| grammar_rule.name == *name)
                            {
                                panic!(
                                    "\n\nError at rule: {}\nIn production: \
                                     {production}\nYour grammar references a \
                                     rule with name: {name}\nBut this rule \
                                     has not been defined in the grammar.\n\n",
                                    grammar_rule.name,
                                )
                            }
                        }
                        Symbol::Lexeme(_) => {}
                    }
                }
            }
        }

        self.grammar.clone()
    }
}
