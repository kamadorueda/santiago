// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Santiago is a lexing and parsing toolkit for Rust.
//!
//! It can parse all [context-free languages](https://en.wikipedia.org/wiki/Context-free_grammar),
//! including [ambiguous](https://en.wikipedia.org/wiki/Ambiguous_grammar)
//! and [recursive](https://en.wikipedia.org/wiki/Recursive_grammar) grammars.
//!
//! It aims to be an alternative to
//! [GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison),
//! [Yacc](https://en.wikipedia.org/wiki/Yacc) and
//! [Flex](https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)).
pub mod grammar;
pub mod lexer;
pub mod parser;

const START_RULE_NAME: &str = "Γ";
