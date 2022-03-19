// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::lexer::Lexer;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
pub struct Rule {
    pub(crate) action:
        Rc<dyn for<'a> Fn(&'a str, &mut Lexer) -> Option<(&'a str, &'a str)>>,
    pub(crate) matcher: Rc<dyn Fn(&str) -> Option<usize>>,
    pub(crate) states:  HashSet<String>,
}
