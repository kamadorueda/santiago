// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexer;
use crate::lexer::LexerRule;
use regex::Regex;
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
        action: for<'a> fn(&'a str, &mut Lexer) -> Option<(&'a str, &'a str)>,
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

    #[cfg(feature = "regular-expressions")]
    pub fn pattern(
        &mut self,
        states: &[&str],
        pattern: &str,
        action: for<'a> fn(&'a str, &mut Lexer) -> Option<(&'a str, &'a str)>,
    ) -> &mut LexerBuilder {
        let regex = Regex::new(&format!("^(:?{pattern})")).unwrap();

        self.table.push(LexerRule {
            action:  Rc::new(action),
            matcher: Rc::new(move |input: &str| -> Option<usize> {
                match regex.find(input) {
                    Some(match_) => Some(match_.end()),
                    None => None,
                }
            }),
            states:  states.iter().map(|state| state.to_string()).collect(),
        });

        self
    }

    pub fn finish(&self) -> Vec<LexerRule> {
        self.table.clone()
    }
}
