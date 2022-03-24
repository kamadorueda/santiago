// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Associativity;
use crate::grammar::Disambiguation;
use crate::grammar::Grammar;
use crate::grammar::GrammarRule;
use crate::grammar::Production;
use crate::grammar::Symbol;
use crate::grammar::START_RULE_NAME;
use std::collections::HashMap;

/// Utility for creating [grammar rules](GrammarRule).
///
/// Please read the [module documentation](crate::grammar) for more information and examples.
pub struct GrammarBuilder {
    current_precedence: usize,
    grammar:            Grammar,
}

impl Default for GrammarBuilder {
    fn default() -> GrammarBuilder {
        GrammarBuilder::new()
    }
}

impl GrammarBuilder {
    /// Creates a new [GrammarBuilder] with no rules.
    pub fn new() -> GrammarBuilder {
        GrammarBuilder {
            current_precedence: 0,
            grammar:            Grammar { rules: HashMap::new() },
        }
    }

    fn rule_to_symbols(&mut self, rule_name: &str, symbols: Vec<Symbol>) {
        let rule_name = rule_name.to_string();
        let production = Production { symbols };

        if self.grammar.rules.is_empty() && rule_name != START_RULE_NAME {
            self.rule_to_rules(START_RULE_NAME, &[&rule_name]);
        }

        match self.grammar.rules.get_mut(&rule_name) {
            Some(rule) => {
                rule.productions.push(production);
            }
            None => {
                self.grammar.rules.insert(
                    rule_name.clone(),
                    GrammarRule {
                        name:           rule_name,
                        disambiguation: None,
                        productions:    vec![production],
                    },
                );
            }
        }
    }

    /// Map a rule with name `name` to zero or more [crate::lexer::LexerRule].
    pub fn rule_to_lexemes(
        &mut self,
        lexeme_kind: &str,
        lexeme_kinds: &[&str],
    ) -> &mut GrammarBuilder {
        self.rule_to_symbols(
            lexeme_kind,
            lexeme_kinds
                .iter()
                .map(|lexeme_kind| Symbol::Lexeme(lexeme_kind.to_string()))
                .collect(),
        );

        self
    }

    /// Map a rule with name `name` to zero or more [GrammarRule].
    pub fn rule_to_rules(
        &mut self,
        rule_name: &str,
        rule_names: &[&str],
    ) -> &mut GrammarBuilder {
        self.rule_to_symbols(
            rule_name,
            rule_names
                .iter()
                .map(|rule_name| Symbol::Rule(rule_name.to_string()))
                .collect(),
        );

        self
    }

    /// Create a [Disambiguation]
    /// with the specified `associativity`,
    /// granting the rules with names `rule_names` equal precedence.
    ///
    /// Increases the precedence counter for future invocations.
    pub fn disambiguate(
        &mut self,
        associativity: Associativity,
        rule_names: &[&str],
    ) -> &mut GrammarBuilder {
        for rule_name in rule_names {
            let rule_name = rule_name.to_string();

            match self.grammar.rules.get_mut(&rule_name) {
                Some(rule) => {
                    rule.disambiguation = Some(Disambiguation {
                        associativity: associativity.clone(),
                        precedence:    self.current_precedence,
                    })
                }
                None => {
                    panic!(
                        "\n\nError while trying to disambiguate a rule with \
                         name: {rule_name}\nWhich has not been previously \
                         defined.\n\n"
                    );
                }
            }
        }

        self.current_precedence += 1;

        self
    }

    /// Return the created [Grammar], performing a few validations first.
    pub fn finish(&self) -> Grammar {
        for (rule_name, rule) in self.grammar.rules.iter() {
            for production in &rule.productions {
                for symbol in &production.symbols {
                    match &symbol {
                        Symbol::Rule(name) => {
                            if !self.grammar.rules.contains_key(name) {
                                panic!(
                                    "\n\nError at rule: {rule_name}\nIn \
                                     production: {production}\nYour grammar \
                                     references a rule with name: {name}\nBut \
                                     this rule has not been defined in the \
                                     grammar.\n\n",
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
