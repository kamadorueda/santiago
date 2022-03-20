// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexer;
use crate::lexer::LexerRule;
use crate::lexer::NextLexeme;
use std::rc::Rc;

pub struct LexerBuilder {
    table: Vec<LexerRule>,
}

impl Default for LexerBuilder {
    fn default() -> LexerBuilder {
        LexerBuilder::new()
    }
}

impl LexerBuilder {
    pub fn new() -> LexerBuilder {
        LexerBuilder { table: Vec::new() }
    }

    pub fn string(
        &mut self,
        states: &[&str],
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
            states:  states.iter().map(|state| state.to_string()).collect(),
        });

        self
    }

    #[cfg(feature = "crate_regex")]
    pub fn pattern(
        &mut self,
        states: &[&str],
        pattern: &str,
        action: fn(&mut Lexer) -> NextLexeme,
    ) -> &mut LexerBuilder {
        let regex =
            crate_regex::Regex::new(&format!("^(:?{pattern})")).unwrap();

        self.table.push(LexerRule {
            action:  Rc::new(action),
            matcher: Rc::new(move |input: &str| -> Option<usize> {
                regex.find(input).map(|match_| match_.end())
            }),
            states:  states.iter().map(|state| state.to_string()).collect(),
        });

        self
    }

    pub fn finish(&self) -> Vec<LexerRule> {
        self.table.clone()
    }
}
