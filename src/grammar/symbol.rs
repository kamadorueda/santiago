// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

/// Terminal or non-terminal element of a [Production](crate::grammar::Production).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Symbol {
    /// A terminal symbol.
    Lexeme(String),
    /// A non-terminal symbol.
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
