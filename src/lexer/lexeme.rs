// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::position::Position;

#[derive(Clone, Debug, Hash)]
pub struct Lexeme {
    pub kind:     String,
    pub raw:      String,
    pub position: Position,
}

impl std::fmt::Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.kind, self.raw, self.position)
    }
}
