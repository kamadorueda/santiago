// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Disambiguation;
use crate::grammar::Production;
use std::hash::Hasher;

/// Internal representation of a grammar rule.
///
/// [GrammarRule] is exposed so you can use its type and traits
/// but normally you create a [GrammarRule]
/// by using a [GrammarBuilder](crate::grammar::GrammarBuilder).
#[derive(Clone)]
pub struct GrammarRule {
    pub(crate) name:           String,
    pub(crate) disambiguation: Option<Disambiguation>,
    pub(crate) productions:    Vec<Production>,
}

impl std::fmt::Display for GrammarRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} := {}",
            self.name,
            self.productions
                .iter()
                .map(Production::to_string)
                .collect::<Vec<String>>()
                .join(" | ")
        )
    }
}

impl std::hash::Hash for GrammarRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
