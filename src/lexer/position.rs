// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

/// Counter for column number, line number, and byte index.
///
/// [Position] is exposed so you can use its type and traits,
/// but normally you don't use it directly.
///
/// Please read the [crate documentation](crate) for more information and examples.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    /// Line number.
    pub line:   usize,
    /// Column number.
    pub column: usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Position {
    /// Iterates the `input` updating the [Position] accordingly.
    pub fn consume(&mut self, input: &str) {
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
