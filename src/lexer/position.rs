// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use std::hash::Hasher;

/// Counter for column number, line number, and byte index.
///
/// [Position] is exposed so you can use its type and traits,
/// but normally you don't use it directly.
///
/// Please read the [crate documentation](crate) for more information and examples.
#[derive(Clone, Eq)]
pub struct Position {
    pub(crate) byte_index: usize,
    pub(crate) column:     usize,
    pub(crate) line:       usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Position) -> std::cmp::Ordering {
        self.byte_index.cmp(&other.byte_index)
    }
}

impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.byte_index == other.byte_index
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::hash::Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.byte_index.hash(state);
    }
}

impl Position {
    pub(crate) fn consume(&mut self, input: &str) {
        self.byte_index += input.len();
        for char in input.chars() {
            if char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}
