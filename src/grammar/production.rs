// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexeme;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hasher;
use std::rc::Rc;

/// One possible derivation of a [GrammarRule](crate::grammar::GrammarRule).
pub struct Production<AST> {
    /// Kind of symbols.
    pub kind:                  ProductionKind,
    /// Name of the [Grammar Rules](crate::grammar::GrammarRule)
    /// or [Lexemes](crate::lexer::Lexeme)
    /// that this [Production] may yield.
    pub symbols:               Vec<String>,
    /// Action that this rule will perform at evaluation time.
    pub action:                Rc<ProductionAction<AST>>,
    pub(crate) target_lexemes: RefCell<HashSet<String>>,
}

impl<AST> std::fmt::Debug for Production<AST> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<AST> std::fmt::Display for Production<AST> {
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

impl<AST> std::cmp::Eq for Production<AST> {}

impl<AST> std::cmp::PartialEq for Production<AST> {
    fn eq(&self, other: &Production<AST>) -> bool {
        (&self.kind, &self.symbols).eq(&(&other.kind, &other.symbols))
    }
}

impl<AST> std::hash::Hash for Production<AST> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.symbols.hash(state);
    }
}

/// Action that a production will perform once evaluated.
pub enum ProductionAction<AST> {
    /// Action to execute when this [Production] is of kind [ProductionKind::Lexemes].
    Lexemes(Rc<dyn Fn(&[&Lexeme]) -> AST>),
    /// Action to execute when this [Production] is of kind [ProductionKind::Rules]
    Rules(Rc<dyn Fn(Vec<AST>) -> AST>),
}

/// Kinds of symbols.
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
