// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Lexer and Parser for a calculator.
//!
//! Example usage:
//! ```rust
//! # let input = include_str!("../../tests/language_calculator/cases/example/input");
//! let lexer_rules = santiago::languages::calculator::lexer_rules();
//! let grammar = santiago::languages::calculator::grammar();
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

///
pub enum Value<'a> {
    ///
    Int(isize),
    ///
    Operation {
        ///
        kind: &'a str,
        ///
        args: Vec<Value<'a>>,
    },
    ///
    Operator(&'a str),
}

/// Build a grammar for this language.
pub fn grammar<'a>() -> Grammar<Value<'a>> {
    use crate as santiago;

    santiago::grammar!(
        "expr" => rules "bin_op" => |mut values| {
            values.swap_remove(0)
        };
        "expr" => rules "int" => |mut values| {
            values.swap_remove(0)
        };

        "bin_op" => rules "expr" "add" "expr" => |values| {
            Value::Operation {
                kind: "add",
                args: values,
            }
        };
        "bin_op" => rules "expr" "subtract" "expr" => |values| {
            Value::Operation {
                kind: "subtract",
                args: values,
            }
        };
        "bin_op" => rules "expr" "multiply" "expr" => |values| {
            Value::Operation {
                kind: "multiply",
                args: values,
            }
        };
        "bin_op" => rules "expr" "divide" "expr" => |values| {
            Value::Operation {
                kind: "divide",
                args: values,
            }
        };

        "int" => lexemes "INT" => |lexemes| {
            let value = str::parse(&lexemes[0].raw).unwrap();
            Value::Int(value)
        };

        "add" => lexemes "+" => |_| Value::Operator("+");
        "subtract" => lexemes "-" => |_| Value::Operator("-");
        "multiply" => lexemes "*" => |_| Value::Operator("*");
        "divide" => lexemes "/" => |_| Value::Operator("/");

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}

impl<'a> Value<'a> {
    ///
    pub fn eval(&self) -> isize {
        match self {
            Value::Int(int) => *int,
            Value::Operation { kind, args } => match *kind {
                "add" => args[0].eval() + args[2].eval(),
                "subtract" => args[0].eval() - args[2].eval(),
                "multiply" => args[0].eval() * args[2].eval(),
                "divide" => args[0].eval() / args[2].eval(),
                kind => unreachable!("{}", kind),
            },
            Value::Operator(_) => unreachable!(),
        }
    }
}
