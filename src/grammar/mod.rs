// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Create grammars that are validated for correctness automatically.
//!
//! Please read the [crate documentation](crate) for more information and examples.

mod grammar_builder;
mod grammar_rule;
mod production;
mod symbol;

pub use grammar_builder::GrammarBuilder;
pub use grammar_rule::GrammarRule;
pub(crate) use production::Production;
pub(crate) use symbol::Symbol;
