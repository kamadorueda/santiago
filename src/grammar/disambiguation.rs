// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Associativity;

/// Internal representation of precedence and [Associativity].
///
/// [Disambiguation] is exposed so you can use its type and traits
/// but normally you create a [Disambiguation]
/// by using a [GrammarBuilder](crate::grammar::GrammarBuilder).
#[derive(Clone)]
pub(crate) struct Disambiguation {
    pub(crate) associativity: Associativity,
    pub(crate) precedence:    usize,
}
