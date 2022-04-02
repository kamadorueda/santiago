// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Production;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

/// Internal representation of a [Production] that has been matched
/// up to certain symbol,
/// starting at `start_column` and ending at `end_column`
/// relative the input [Lexemes](crate::lexer::Lexeme).
pub struct ParserState<AST> {
    pub(crate) rule_name:    Rc<String>,
    pub(crate) production:   Rc<Production<AST>>,
    pub(crate) dot_index:    usize,
    pub(crate) start_column: usize,
    pub(crate) end_column:   usize,
}

impl<AST> std::fmt::Debug for ParserState<AST> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<AST> std::fmt::Display for ParserState<AST> {
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

impl<AST> core::clone::Clone for ParserState<AST> {
    fn clone(&self) -> ParserState<AST> {
        ParserState {
            rule_name:    self.rule_name.clone(),
            production:   self.production.clone(),
            dot_index:    self.dot_index,
            start_column: self.start_column,
            end_column:   self.end_column,
        }
    }
}

impl<AST> std::cmp::Eq for ParserState<AST> {}

impl<AST> std::cmp::PartialEq for ParserState<AST> {
    fn eq(&self, other: &ParserState<AST>) -> bool {
        let left = (
            &self.rule_name,
            &self.production,
            &self.dot_index,
            &self.start_column,
            &self.end_column,
        );
        let right = (
            &other.rule_name,
            &other.production,
            &other.dot_index,
            &other.start_column,
            &other.end_column,
        );

        left.eq(&right)
    }
}
impl<AST> ParserState<AST> {
    pub(crate) fn completed(&self) -> bool {
        self.dot_index >= self.production.symbols.len()
    }

    pub(crate) fn next_symbol(&self) -> Option<&String> {
        self.production.symbols.get(self.dot_index)
    }

    pub(crate) fn hash_me(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.rule_name.hash(&mut hasher);
        self.production.hash(&mut hasher);
        self.dot_index.hash(&mut hasher);
        self.start_column.hash(&mut hasher);
        self.end_column.hash(&mut hasher);
        hasher.finish()
    }
}
