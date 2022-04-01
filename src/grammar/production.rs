// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexeme;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hasher;
use std::rc::Rc;

/// One possible derivation of a [GrammarRule](crate::grammar::GrammarRule).
pub struct Production<Value> {
    /// Kind of symbols.
    pub kind:                  ProductionKind,
    /// Name of the [Grammar Rules](crate::grammar::GrammarRule)
    /// or [Lexemes](crate::lexer::Lexeme)
    /// that this [Production] may yield.
    pub symbols:               Vec<String>,
    /// Action that this rule will perform at evaluation time.
    pub action:                Rc<ProductionAction<Value>>,
    pub(crate) target_lexemes: RefCell<HashSet<String>>,
}

impl<Value> std::fmt::Debug for Production<Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<Value> std::fmt::Display for Production<Value> {
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

impl<Value> std::cmp::Eq for Production<Value> {}

impl<Value> std::cmp::PartialEq for Production<Value> {
    fn eq(&self, other: &Production<Value>) -> bool {
        (&self.kind, &self.symbols).eq(&(&other.kind, &other.symbols))
    }
}

impl<Value> std::hash::Hash for Production<Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.symbols.hash(state);
    }
}

/// Action that a production will perform once evaluated.
pub enum ProductionAction<Value> {
    /// Action to execute when this [Production] is of kind [ProductionKind::Lexemes].
    Lexemes(Rc<dyn Fn(&[&Lexeme]) -> Value>),
    /// Action to execute when this [Production] is of kind [ProductionKind::Rules]
    Rules(Rc<dyn Fn(Vec<Value>) -> Value>),
}

/// Kinds of [Symbols].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ProductionKind {
    /// All Symbols are [Lexemes](crate::lexer::Lexeme).
    Lexemes,
    /// All Symbols are [Grammar Rules](crate::grammar::GrammarRule).
    Rules,
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
