// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Create grammars that are validated for correctness automatically.
//!
//! Please read the [crate documentation](crate) for more information and examples.

mod associativity;
mod disambiguation;
mod grammar_builder;
mod grammar_rule;
mod production;
mod symbol;

pub use associativity::Associativity;
pub(crate) use disambiguation::Disambiguation;
pub use grammar_builder::GrammarBuilder;
pub(crate) use grammar_rule::GrammarRule;
pub(crate) use production::Production;
use std::collections::HashMap;
use std::rc::Rc;
pub(crate) use symbol::Symbol;

pub(crate) const START_RULE_NAME: &str = "Γ";

/// Internal representation of a grammar.
///
/// [Grammar] is exposed so you can use its type and traits
/// but normally you create a [Grammar]
/// by using a [GrammarBuilder](crate::grammar::GrammarBuilder).
#[derive(Clone)]
pub struct Grammar {
    pub(crate) rules: HashMap<Rc<String>, GrammarRule>,
}
