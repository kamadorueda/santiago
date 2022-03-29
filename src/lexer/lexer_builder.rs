// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexer;
use crate::lexer::LexerRule;
use crate::lexer::LexerRules;
use crate::lexer::NextLexeme;
use std::collections::HashMap;
use std::rc::Rc;

/// Imperative utility for creating [LexerRules].
///
/// Please read the [crate documentation](crate) for more information and examples.
pub struct LexerBuilder {
    rules: LexerRules,
}

impl LexerBuilder {
    /// Creates a new [LexerBuilder] with no rules.
    #[allow(clippy::new_without_default)]
    pub fn new() -> LexerBuilder {
        LexerBuilder { rules: LexerRules { rules: HashMap::new() } }
    }

    fn insert(&mut self, states: &[&str], rule: LexerRule) {
        for state in states {
            let state = state.to_string();

            match self.rules.rules.get_mut(&state) {
                Some(rules) => {
                    rules.push(rule.clone());
                }
                None => {
                    self.rules
                        .rules
                        .insert(state.to_string(), vec![rule.clone()]);
                }
            }
        }
    }

    /// Add a rule that will be active
    /// when the current [Lexer] state matches any of `states`,
    /// with name `name`,
    /// that matches exactly the content of `string`,
    /// and performs the provided `action`.
    pub fn string(
        &mut self,
        states: &[&str],
        name: &str,
        string: &'static str,
        action: fn(&mut Lexer) -> NextLexeme,
    ) -> &mut LexerBuilder {
        self.insert(
            states,
            LexerRule {
                action:  Rc::new(action),
                matcher: Rc::new(move |input: &str| -> Option<usize> {
                    if input.starts_with(&string) {
                        Some(string.len())
                    } else {
                        None
                    }
                }),
                name:    name.to_string(),
            },
        );

        self
    }

    /// Add a rule that will be active
    /// when the current [Lexer] state matches any of `states`,
    /// with name `name`,
    /// that matches the regular expression `pattern`,
    /// and performs the provided `action`.
    #[cfg(feature = "crate_regex")]
    pub fn pattern(
        &mut self,
        states: &[&str],
        name: &str,
        pattern: &str,
        action: fn(&mut Lexer) -> NextLexeme,
    ) -> &mut LexerBuilder {
        let regex =
            crate_regex::Regex::new(&format!(r"\A(?:{pattern})")).unwrap();

        self.insert(
            states,
            LexerRule {
                action:  Rc::new(action),
                matcher: Rc::new(move |input: &str| -> Option<usize> {
                    regex
                        .find_iter(input)
                        .take(1)
                        .map(|match_| match_.end())
                        .next()
                }),
                name:    name.to_string(),
            },
        );

        self
    }

    /// Return the created [LexerRules].
    pub fn finish(&self) -> LexerRules {
        self.rules.clone()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __lexer_rules_helper {
    (
        $builder:ident
        | $rule_name:literal
        = $matcher:ident $matcher_arg:literal
    ) => {
        $builder.$matcher(&["INITIAL"], $rule_name, $matcher_arg, |lexer| {
            lexer.take()
        });
    };
    (
        $builder:ident
        | $rule_name:literal
        = $matcher:ident $matcher_arg:literal
        => $action:expr
    ) => {
        $builder.$matcher(&["INITIAL"], $rule_name, $matcher_arg, $action);
    };
    (
        $builder:ident
        $( $states:literal )+
        | $rule_name:literal
        = $matcher:ident $matcher_arg:literal
    ) => {
        $builder.$matcher(&[$($states),*], $rule_name, $matcher_arg, |lexer| {
            lexer.take()
        });
    };
    (
        $builder:ident
        $( $states:literal )+
        | $rule_name:literal
        = $matcher:ident $matcher_arg:literal
        => $action:expr
    ) => {
        $builder.$matcher(&[$($states),*], $rule_name, $matcher_arg, $action);
    };
}

/// Declarative utility for creating a [Grammar].
///
/// Please read the [module documentation](crate) for more information and examples.
#[macro_export]
macro_rules! lexer_rules {
    ($(
        $( $states:literal )*
        | $rule_name:literal
        = $matcher:ident $matcher_arg:literal
        $( => $action:expr )?
    );* ;) => {{
        let mut builder = santiago::lexer::LexerBuilder::new();

        $(santiago::__lexer_rules_helper!(
            builder
            $( $states )*
            | $rule_name
            = $matcher $matcher_arg
            $( => $action )?
        ));*;

        builder.finish()
    }};
}
