// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Position;

/// Represents a group or related characters and its position.
///
/// Please read the [crate documentation](crate) for more information and examples.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Lexeme {
    /// Name of the [LexerRules](crate::lexer::LexerRules)
    /// that produced this [Lexeme].
    pub kind:     String,
    /// Raw content of this [Lexeme],
    /// as matched by the [Lexer](crate::lexer::Lexer).
    pub raw:      String,
    /// [Position] of this [Lexeme] relative to the input
    /// passed to the [Lexer](crate::lexer::Lexer).
    pub position: Position,
}

impl std::fmt::Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.kind, self.raw, self.position)
    }
}
