// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Associativity;
use crate::grammar::Disambiguation;
use crate::grammar::GrammarRule;
use crate::grammar::Production;
use crate::grammar::Symbol;

/// Utility for creating [grammar rules](GrammarRule).
///
/// Please read the [module documentation](crate::grammar) for more information and examples.
pub struct GrammarBuilder {
    current_precedence: usize,
    grammar:            Vec<GrammarRule>,
}

impl Default for GrammarBuilder {
    fn default() -> GrammarBuilder {
        GrammarBuilder::new()
    }
}

impl GrammarBuilder {
    /// Creates a new [GrammarBuilder] with no rules.
    pub fn new() -> GrammarBuilder {
        GrammarBuilder { current_precedence: 0, grammar: Vec::new() }
    }

    fn rule_to_symbols(&mut self, rule_name: &str, symbols: Vec<Symbol>) {
        let rule_name = rule_name.to_string();
        let production = Production { symbols };

        match self.grammar.iter().position(|rule| rule.name == rule_name) {
            Some(index) => {
                self.grammar[index].productions.push(production);
            }
            None => {
                self.grammar.push(GrammarRule {
                    name:           rule_name,
                    disambiguation: None,
                    productions:    vec![production],
                });
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
            match self.grammar.iter().position(|rule| &rule.name == rule_name) {
                Some(index) => {
                    self.grammar[index].disambiguation = Some(Disambiguation {
                        associativity: associativity.clone(),
                        precedence:    self.current_precedence,
                    })
                }
                None => panic!(
                    "\n\nError while trying to disambiguate a rule with name: \
                     {rule_name}\nWhich has not been previously defined.\n\n"
                ),
            }
        }

        self.current_precedence += 1;

        self
    }

    /// Return the created [GrammarRule]s, performing a few validations first.
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
