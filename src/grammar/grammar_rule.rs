// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Disambiguation;
use crate::grammar::Production;
use std::hash::Hasher;
use std::rc::Rc;

/// Internal representation of a grammar rule.
///
/// [GrammarRule] is exposed so you can use its type and traits
/// but normally you create a [GrammarRule]
/// by using a [GrammarBuilder](crate::grammar::GrammarBuilder).
pub struct GrammarRule<AST> {
    pub(crate) name:           Rc<String>,
    pub(crate) disambiguation: Option<Disambiguation>,
    pub(crate) productions:    Vec<Rc<Production<AST>>>,
}

impl<AST> std::clone::Clone for GrammarRule<AST> {
    fn clone(&self) -> GrammarRule<AST> {
        GrammarRule {
            name:           self.name.clone(),
            disambiguation: self.disambiguation.clone(),
            productions:    self.productions.clone(),
        }
    }
}

impl<AST> std::fmt::Display for GrammarRule<AST> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} := {}",
            self.name,
            self.productions
                .iter()
                .map(|production| production.to_string())
                .collect::<Vec<String>>()
                .join("\n    | ")
        )
    }
}

impl<AST> std::hash::Hash for GrammarRule<AST> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
