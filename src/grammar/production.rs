// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hasher;

/// One possible derivation of a [GrammarRule](crate::grammar::GrammarRule).
#[derive(Clone, Debug)]
pub struct Production {
    /// Kind of symbols.
    pub kind:                  ProductionKind,
    /// Name of the [Grammar Rules](crate::grammar::GrammarRule)
    /// or [Lexemes](crate::lexer::Lexeme)
    /// that this [Production] may yield.
    pub symbols:               Vec<String>,
    pub(crate) target_lexemes: RefCell<HashSet<String>>,
}

/// Kinds of [Symbols].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProductionKind {
    /// All Symbols are [Lexemes](crate::lexer::Lexeme).
    Lexemes,
    /// All Symbols are [Grammar Rules](crate::grammar::GrammarRule).
    Rules,
}

impl std::fmt::Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.kind,
            self.symbols
                .iter()
                .map(|symbol| format!("{symbol:?}"))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl std::cmp::Eq for Production {}

impl std::cmp::PartialEq for Production {
    fn eq(&self, other: &Production) -> bool {
        (&self.kind, &self.symbols).eq(&(&other.kind, &other.symbols))
    }
}

impl std::hash::Hash for Production {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbols.hash(state);
    }
}

impl std::fmt::Display for ProductionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProductionKind::Lexemes => "lexemes",
                ProductionKind::Rules => "rules",
            }
        )
    }
}
