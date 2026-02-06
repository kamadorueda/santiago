// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Position;

/// Internal representation of an error encountered by the [Lexer](crate::lexer::Lexer).
#[derive(Debug)]
pub struct LexerError {
    /// Byte index relative to the [Lexer](crate::lexer::Lexer)
    /// input where the error was encountered.
    pub byte_index:   usize,
    /// Length of the current match,
    /// or none if the [Lexer](crate::lexer::Lexer) did not match something.
    pub match_len:    Option<usize>,
    /// Human readable representation of the error.
    pub message:      String,
    /// [Position] where the error was found.
    pub position:     Position,
    /// Current stack of states in the [Lexer](crate::lexer::Lexer).
    pub states_stack: Vec<String>,
    /// Current marked positions in the [Lexer](crate::lexer::Lexer).
    pub marks: Vec<Position>,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", &self.message)?;
        writeln!(f, "At: {}", self.position)?;
        writeln!(f, "With states stack: {:?}", self.states_stack)?;
        write!(f, "With marks: {:?}", self.marks)
    }
}
