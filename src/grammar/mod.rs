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

pub use associativity::Associativity;
pub use disambiguation::Disambiguation;
pub use grammar_builder::GrammarBuilder;
pub use grammar_rule::GrammarRule;
pub use production::Production;
pub use production::ProductionAction;
pub use production::ProductionKind;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) const START_RULE_NAME: &str = "Γ";

/// Internal representation of a grammar.
///
/// [Grammar] is exposed so you can use its type and traits
/// but normally you create a [Grammar]
/// by using a [GrammarBuilder](crate::grammar::GrammarBuilder).
pub struct Grammar<Value> {
    /// Internal representation of the rules of this [Grammar].
    pub rules: HashMap<Rc<String>, GrammarRule<Value>>,
}

impl<Value> std::clone::Clone for Grammar<Value> {
    fn clone(&self) -> Grammar<Value> {
        Grammar { rules: self.rules.clone() }
    }
}
