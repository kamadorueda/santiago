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
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

/// Imperative utility for creating a [Grammar].
///
/// Please read the [crate documentation](crate) for more information and examples.
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
        let rule_name = Rc::new(rule_name.to_string());
        let production = Rc::new(Production {
            target_lexemes: RefCell::new(HashSet::new()),
            symbols,
        });

        if self.grammar.rules.is_empty() && *rule_name != START_RULE_NAME {
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
                        name:           rule_name.clone(),
                        disambiguation: None,
                        productions:    vec![production],
                    },
                );
            }
        }
    }

    /// Map a rule with name `name` to zero or more lexemes.
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

    /// Map a rule with name `name` to zero or more rules.
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

    /// Create a disambiguation
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

    fn compute_target_lexemes(&mut self) {
        loop {
            let mut converged = true;

            for rule in self.grammar.rules.values() {
                for production in rule.productions.iter() {
                    if production.symbols.is_empty() {
                        continue;
                    }
                    match &production.symbols[0] {
                        Symbol::Lexeme(lexeme_kind) => {
                            if !production
                                .target_lexemes
                                .borrow()
                                .contains(lexeme_kind)
                            {
                                production
                                    .target_lexemes
                                    .borrow_mut()
                                    .insert(lexeme_kind.clone());
                                converged = false;
                            }
                        }
                        Symbol::Rule(target_rule_name) => {
                            for from_production in self
                                .grammar
                                .rules
                                .get(target_rule_name)
                                .unwrap()
                                .productions
                                .iter()
                            {
                                if from_production != production
                                    && !from_production.symbols.is_empty()
                                {
                                    for target_lexeme in from_production
                                        .target_lexemes
                                        .borrow()
                                        .iter()
                                    {
                                        if !production
                                            .target_lexemes
                                            .borrow()
                                            .contains(target_lexeme)
                                        {
                                            production
                                                .target_lexemes
                                                .borrow_mut()
                                                .insert(target_lexeme.clone());
                                            converged = false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if converged {
                break;
            }
        }
    }

    /// Return the created [Grammar], performing a few validations first.
    pub fn finish(&mut self) -> Grammar {
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

        self.compute_target_lexemes();

        self.grammar.clone()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __grammar_helper {
    ($grammar:ident $rule_name:literal => empty) => {
        $grammar.rule_to_rules($rule_name, &[]);
    };
    ($grammar:ident $rule_name:literal => rules $($rule_names:literal)*) => {
        $grammar.rule_to_rules($rule_name, &[$($rule_names),*]);
    };
    ($grammar:ident $rule_name:literal => rule $($rule_names:literal)*) => {
        $grammar.rule_to_rules($rule_name, &[$($rule_names),*]);
    };
    ($grammar:ident $rule_name:literal => lexemes $($lexeme_kinds:literal)*) => {
        $grammar.rule_to_lexemes($rule_name, &[$($lexeme_kinds),*]);
    };
    ($grammar:ident $rule_name:literal => lexeme $($lexeme_kinds:literal)*) => {
        $grammar.rule_to_lexemes($rule_name, &[$($lexeme_kinds),*]);
    };
    ($grammar:ident $associativity:expr => rules $($rule_names:literal)*) => {
        $grammar.disambiguate($associativity, &[$($rule_names),*]);
    };
    ($grammar:ident $associativity:path => rule $($rule_names:literal)*) => {
        $grammar.disambiguate($associativity, &[$($rule_names),*]);
    };
}

/// Declarative utility for creating a [Grammar].
///
/// Please read the [crate documentation](crate) for more information and examples.
#[macro_export]
macro_rules! grammar {
    ($($target:expr => $action:ident $($args:literal)*);* ;) => {{
        let mut builder = santiago::grammar::GrammarBuilder::new();

        $(santiago::__grammar_helper!(builder $target => $action $($args)*));*;

        builder.finish()
    }};
}
