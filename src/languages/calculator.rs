// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Lexer and Parser for a calculator.
//!
//! Example usage:
//! ```rust
//! # let input = include_str!("../../tests/language_calculator/cases/example/input");
//! let lexer_rules = santiago::languages::nix::lexer_rules();
//! let grammar = santiago::languages::nix::grammar();
//!
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! let abstract_syntax_trees =
//!     santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//!
//! Example input:
//!
//! ```nix
#![doc = include_str!("../../tests/language_calculator/cases/example/input")]
//! ```
//! 
//! Lexemes:
//! ```text
#![doc = include_str!("../../tests/language_calculator/cases/example/lexemes")]
//! ```
//! 
//! Abstract Syntax Tree:
//! ```text
#![doc = include_str!("../../tests/language_calculator/cases/example/forest")]
//! ```

use crate::grammar::Associativity;
use crate::grammar::Grammar;
use crate::lexer::LexerRules;

/// Build a set of lexer rules for this language.
pub fn lexer_rules() -> LexerRules {
    use crate as santiago;
    santiago::lexer_rules!(
        "DEFAULT" | "INT" = pattern r"[0-9]+";
        "DEFAULT" | "+" = string "+";
        "DEFAULT" | "-" = string "-";
        "DEFAULT" | "*" = string "*";
        "DEFAULT" | "/" = string "/";
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    )
}

/// Build a grammar for this language.
pub fn grammar() -> Grammar {
    use crate as santiago;
    santiago::grammar!(
        "sum" => rules "sum" "plus" "sum";
        "sum" => rules "sum" "subtract" "sum";
        "sum" => rules "sum" "multiply" "sum";
        "sum" => rules "sum" "divide" "sum";

        "sum" => lexemes "INT";

        "plus" => lexemes "+";
        "subtract" => lexemes "-";
        "multiply" => lexemes "*";
        "divide" => lexemes "/";

        Associativity::Left => rules "plus" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}