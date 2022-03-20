// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Symbol {
    Lexeme(String),
    Rule(String),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Lexeme(kind) => write!(f, "{kind:?}"),
            Symbol::Rule(rule) => write!(f, "{rule}"),
        }
    }
}
