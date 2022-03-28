// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::LexerRule;
use std::collections::HashMap;

/// Group of lexer rules optimized for performance.
///
/// [LexerRules] is exposed so you can use its type and traits,
/// but normally you create [LexerRules]
/// by using a [LexerBuilder](crate::lexer::LexerBuilder).
#[derive(Clone)]
pub struct LexerRules {
    pub(crate) rules: HashMap<String, Vec<LexerRule>>,
}
