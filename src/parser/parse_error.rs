// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexeme;
use crate::parser::ParserState;

/// Internal representation of an error encountered by [parse()].
#[derive(Debug)]
pub struct ParseError {
    /// [Lexeme] where the error was found.
    pub at:     Option<Lexeme>,
    /// Matched, partially matched, and expected lexemes up at this point.
    pub states: Vec<ParserState>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(at) = &self.at {
            writeln!(f, "At: {}", at)?;
        } else {
            writeln!(f, "At start of the input")?;
        }

        write!(
            f,
            "States:\n  {}",
            self.states
                .iter()
                .map(ParserState::to_string)
                .collect::<Vec<String>>()
                .join("\n  ")
        )
    }
}
