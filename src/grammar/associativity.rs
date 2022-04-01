// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

/// Ways in which repeated uses of rules with the same precedence nest.
#[derive(Clone)]
pub enum Associativity {
    /// Specifies left associativity: `(x op y) op z`.
    Left,
    /// Specifies right associativity: `x op (y op z)`.
    Right,
    /// Specifies that `x op y op z` is considered a syntax error.
    None,
}
