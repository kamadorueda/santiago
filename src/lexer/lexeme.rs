// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Position;

/// Represents a group or related characters and its position.
///
/// Please read the [crate documentation](crate) for more information and examples.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Lexeme {
    pub(crate) kind:     String,
    pub(crate) raw:      String,
    pub(crate) position: Position,
}

impl std::fmt::Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.kind, self.raw, self.position)
    }
}
