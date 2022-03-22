// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Santiago is a lexing and parsing toolkit for Rust.
//! It provides a library for defining
//! [context-free grammars](https://en.wikipedia.org/wiki/Context-free_grammar),
//! including [ambiguous](https://en.wikipedia.org/wiki/Ambiguous_grammar)
//! and [recursive](https://en.wikipedia.org/wiki/Recursive_grammar) grammars;
//! a [lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis) module,
//! and out-of-the-box parsers for the following languages:
//!
//! - [Nix Expression Language](languages::nix)
//!
//! Santiago aims to be the Rust alternative to
//! [GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison),
//! [Yacc](https://en.wikipedia.org/wiki/Yacc) and
//! [Flex](https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)).
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/santiago)
//! and can be used by adding `santiago`
//! to your dependencies in your project's Cargo.toml
//!
//! ```toml
//! [dependencies]
//! santiago = "*"
//! ```
//!
//! # Example: calculator
//!
//! General use of this library includes creating a set of lexer and grammar rules
//! and then using them to lex and parse some input
//! in order to build an Abstract Syntax Tree.
//!
//! For example, to lex and parse the addition of integer numbers:
//!
//! ```rust
//! let input = "11 + 22 + 33";
//!
//! let lexer_rules = santiago::lexer::LexerBuilder::new()
//!     // One more sequential digits from 0 to 9 will be mapped to an "INT"
//!     .pattern(&["INITIAL"], "INT", r"[0-9]+", |lexer| lexer.take())
//!     // A literal "+" will be mapped to "PLUS"
//!     .string(&["INITIAL"], "PLUS", "+", |lexer| lexer.take())
//!     // Whitespace " " will be ignored
//!     .string(&["INITIAL"], "WS", " ", |lexer| lexer.skip())
//!     .finish();
//!
//! let lexemes = santiago::lexer::lex(&lexer_rules, input);
//!
//! assert_eq!(
//!     vec![
//!         // kind raw (line, column)
//!         r#"INT "11" (1, 1)"#,
//!         r#"PLUS "+" (1, 4)"#,
//!         r#"INT "22" (1, 6)"#,
//!         r#"PLUS "+" (1, 9)"#,
//!         r#"INT "33" (1, 11)"#
//!     ],
//!     lexemes
//!         .iter()
//!         .map(santiago::lexer::Lexeme::to_string)
//!         .collect::<Vec<String>>()
//! );
//!
//! // At this point we can define a grammar for this language:
//! let grammar = santiago::grammar::GrammarBuilder::new()
//!     // Map the rule "sum" to the sequence of rules "sum", "plus", and "sum"
//!     .map_to_rules("sum", &["sum", "plus", "sum"])
//!     // Map the rule "sum" to the lexeme "INT"
//!     .map_to_lexemes("sum", &["INT"])
//!     // Map the rule "plus" to the lexeme "PLUS"
//!     .map_to_lexemes("plus", &["PLUS"])
//!     .finish();
//!
//! // With this we can now parse the input and the the Abstract Syntax Tree:
//! let ast = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];
//! assert_eq!(
//!     vec![
//!         r#"Γ"#,
//!         r#"  sum"#,
//!         r#"    sum"#,
//!         r#"      sum"#,
//!         r#"        INT "11" (1, 1)"#,
//!         r#"      plus"#,
//!         r#"        PLUS "+" (1, 4)"#,
//!         r#"      sum"#,
//!         r#"        INT "22" (1, 6)"#,
//!         r#"    plus"#,
//!         r#"      PLUS "+" (1, 9)"#,
//!         r#"    sum"#,
//!         r#"      INT "33" (1, 11)"#,
//!         r#""#
//!     ],
//!     ast.to_string().split('\n').collect::<Vec<&str>>(),
//! );
//! ```

pub mod grammar;
pub mod languages;
pub mod lexer;
pub mod parser;

#[macro_export]
macro_rules! def {
    ($name:ident, $value:expr) => {
        macro_rules! $name {
            () => {
                $value
            };
        }
    };
}
