// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Create grammars that are validated for correctness automatically.
//!
//! Regular use of this module uses a [GrammarBuilder]
//! to construct a [`Vec<GrammarRule>`](GrammarRule).
//!
//! For example:
//!
//! ```rust
//! santiago::grammar::GrammarBuilder::new()
//!     // A `number` can be an `int` or a `float`
//!     .map_to_rules("number", &["int"])
//!     .map_to_rules("number", &["float"])
//!     // An `int` comes from a lexeme of kind `INT`:
//!     .map_to_lexemes("int", &["INT"])
//!     // An `float` comes from a lexeme of kind `FLOAT`:
//!     .map_to_lexemes("float", &["FLOAT"])
//!     .finish();
//! ```
//!
//! This module checks for grammar correctness.
//! For instance, the following example references a rule
//! that is not defined later in the grammar,
//! which constitutes an error:
//!
//! ```should_panic
//! // Map rule `A` to `B` and don't define rule B:
//! santiago::grammar::GrammarBuilder::new().map_to_rules("A", &["B"]).finish();
//! ```

mod grammar_builder;
mod grammar_rule;
mod production;
mod symbol;

pub use grammar_builder::GrammarBuilder;
pub use grammar_rule::GrammarRule;
pub(crate) use production::Production;
pub(crate) use symbol::Symbol;
