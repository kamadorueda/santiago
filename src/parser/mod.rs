// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Build a data structure representing the input.
//!
//! Please read the [crate documentation](crate) for more information and examples.

mod parse;
mod parser_column;
mod parser_state;
mod tree;

pub use parse::parse;
pub(crate) use parser_column::ParserColumn;
pub(crate) use parser_state::ParserState;
pub use tree::Tree;
