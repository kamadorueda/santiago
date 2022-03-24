// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexer;
use crate::lexer::LexerRule;
use crate::lexer::NextLexeme;
use std::rc::Rc;

/// Utility for creating [lexer rules](LexerRule).
///
/// Please read the [crate documentation](crate) for more information and examples.
pub struct LexerBuilder {
    table: Vec<LexerRule>,
}

impl Default for LexerBuilder {
    fn default() -> LexerBuilder {
        LexerBuilder::new()
    }
}

impl LexerBuilder {
    /// Creates a new [LexerBuilder] with no rules.
    pub fn new() -> LexerBuilder {
        LexerBuilder { table: Vec::new() }
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
        self.table.push(LexerRule {
            action:  Rc::new(action),
            matcher: Rc::new(move |input: &str| -> Option<usize> {
                if input.starts_with(&string) {
                    Some(string.len())
                } else {
                    None
                }
            }),
            name:    name.to_string(),
            states:  states.iter().map(|state| state.to_string()).collect(),
        });

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

        self.table.push(LexerRule {
            action:  Rc::new(action),
            matcher: Rc::new(move |input: &str| -> Option<usize> {
                regex.find_iter(input).map(|match_| match_.end()).next()
            }),
            name:    name.to_string(),
            states:  states.iter().map(|state| state.to_string()).collect(),
        });

        self
    }

    /// Return the created [LexerRule]s.
    pub fn finish(&self) -> Vec<LexerRule> {
        self.table.clone()
    }
}
