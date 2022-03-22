// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexer;
use crate::lexer::NextLexeme;
use std::collections::HashSet;
use std::rc::Rc;

/// Internal type for a lexer rule.
///
/// [LexerRule] is exposed so you can use its type and traits,
/// but normally you create [LexerRule]s
/// by using a [LexerBuilder](crate::lexer::LexerBuilder).
#[derive(Clone)]
pub struct LexerRule {
    pub(crate) action:  Rc<dyn Fn(&mut Lexer) -> NextLexeme>,
    pub(crate) matcher: Rc<dyn Fn(&str) -> Option<usize>>,
    pub(crate) name:    String,
    pub(crate) states:  HashSet<String>,
}
