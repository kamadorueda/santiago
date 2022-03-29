// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Symbol;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hasher;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Production {
    pub(crate) symbols:        Vec<Symbol>,
    pub(crate) target_lexemes: RefCell<HashSet<String>>,
}

impl std::fmt::Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.symbols
                .iter()
                .map(Symbol::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::hash::Hash for Production {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbols.hash(state);
    }
}
