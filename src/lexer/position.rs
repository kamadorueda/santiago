// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::hash::Hasher;

#[derive(Clone, Eq)]
pub struct Position {
    pub column: usize,
    pub index:  usize,
    pub line:   usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Position) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.index == other.index
    }
}

impl std::default::Default for Position {
    fn default() -> Position {
        Position::new()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::hash::Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl Position {
    pub(crate) fn consume(&mut self, input: &str) {
        self.index += input.len();
        for char in input.chars() {
            if char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

    pub fn new() -> Position {
        Position { column: 1, index: 0, line: 1 }
    }
}
