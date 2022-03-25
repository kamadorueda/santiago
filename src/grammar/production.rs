// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Symbol;

use std;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Production {
    pub symbols: Vec<Symbol>,
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
