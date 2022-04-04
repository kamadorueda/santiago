// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::rc::Rc;

use crate::lexer::Lexeme;
use crate::parser::ParserState;

/// Internal representation of an error encountered by [crate::parser::parse()].
pub struct ParseError<AST> {
    /// [Lexeme] where the error was found.
    pub at:     Option<Rc<Lexeme>>,
    /// Matched, partially matched, and expected lexemes up at this point.
    pub states: Vec<ParserState<AST>>,
}

impl<AST> std::fmt::Debug for ParseError<AST> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<AST> std::fmt::Display for ParseError<AST> {
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
