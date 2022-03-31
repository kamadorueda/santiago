// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Production;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

/// Internal representation of a [Production] that has been matched
/// up to certain [Symbol],
/// starting at `start_column` and ending at `end_column`
/// relative the input [Lexemes](crate::lexer::Lexeme).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParserState {
    pub(crate) rule_name:    Rc<String>,
    pub(crate) production:   Rc<Production>,
    pub(crate) dot_index:    usize,
    pub(crate) start_column: usize,
    pub(crate) end_column:   usize,
}

impl std::fmt::Display for ParserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} := {} {}• {}[{}-{}]",
            self.rule_name,
            self.production.kind,
            self.production.symbols[0..self.dot_index]
                .iter()
                .map(|symbol| format!("{symbol:?} "))
                .collect::<Vec<String>>()
                .join(""),
            self.production.symbols[self.dot_index..]
                .iter()
                .map(|symbol| format!("{symbol:?} "))
                .collect::<Vec<String>>()
                .join(""),
            self.start_column,
            if self.end_column == usize::MAX {
                "".to_string()
            } else {
                self.end_column.to_string()
            },
        )
    }
}

impl ParserState {
    pub(crate) fn completed(&self) -> bool {
        self.dot_index >= self.production.symbols.len()
    }

    pub(crate) fn next_symbol(&self) -> Option<&String> {
        self.production.symbols.get(self.dot_index)
    }

    pub(crate) fn hash_me(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
