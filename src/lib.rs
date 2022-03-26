// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

#![deny(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::private_doc_tests)]
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
//! ## Examples
//!
//! # Calculator
//!
//! For this example
//! we are interested in lexing and parsing the addition of integer numbers
//! like:
//! - `11 + 22 + 33`
//!
//! And turning them into an
//! [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
//! like:
//!
//! ```text
#![doc = include_str!("../tests/integer_addition/cases/addition/forest")]
//! ```
//! 
//! So let's start with creating a lexer to:
//! - Group the digits into integers called `"INT"`
//! - Capture the plus sign (`+`) and name it `"PLUS"`
//! - Ignore all whitespace
//!
//! In code this would be:
//! ```rust
#![doc = include_str!("../tests/ambiguous_integer_addition/lexer.rs")]
//! ```
//! 
//! Once we have our rules defined, we can start lexing:
//! ```rust
//! # mod m {
//! #   include!("../tests/ambiguous_integer_addition/lexer.rs");
//! # }
//! # use m::*;
//! let input = "11 + 22 + 33";
//!
//! let lexer_rules = lexer();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input);
//! ```
//! 
//! A [Lexeme](lexer::Lexeme) gives us information like:
//! - Token kind
//! - Contents
//! - Position (line and column number)
//! ```text
#![doc = include_str!("../tests/ambiguous_integer_addition/cases/addition/lexemes")]
//! ```
//! 
//! At this point all we are missing is creating a parser.
//!
//! Let's create a grammar to recognize the addition of integer numbers:
//! ```bnf
//! <sum> ::= <sum> <plus> <sum> | "INT"
//! <plus> := "PLUS"
//! ```
//! 
//! In code this would be:
//! ```rust
#![doc = include_str!("../tests/ambiguous_integer_addition/grammar.rs")]
//! ```
//! 
//! Now we can generate all possible Abstract Syntax Trees!
//! ```rust
//! # mod m {
//! #   include!("../tests/ambiguous_integer_addition/grammar.rs");
//! #   include!("../tests/ambiguous_integer_addition/lexer.rs");
//! # }
//! # use m::*;
//! let input = "11 + 22 + 33";
//!
//! let lexer_rules = lexer();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input);
//!
//! let grammar = grammar();
//! let abstract_syntax_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! And voilà!
//! ```text
#![doc = include_str!("../tests/ambiguous_integer_addition/cases/addition/forest")]
//! ```
//! 
//! Notice that we intentionally created an
//! [ambiguous grammar](https://en.wikipedia.org/wiki/Ambiguous_grammar)
//! and this is why we have 2 possible Abstract Syntax Trees.
//!
//! However, we can do better and remove the ambiguities
//! by adding associativity constraints to the "plus" rule.
//!
//! In code, we only need to add one line at the end of our previous grammar:
//! ```rust
#![doc = include_str!("../tests/integer_addition/grammar.rs")]
//! ```
//! 
//! And parse again!
//! ```rust
//! # mod m {
//! #   include!("../tests/integer_addition/grammar.rs");
//! #   include!("../tests/integer_addition/lexer.rs");
//! # }
//! # use m::*;
//! let input = "11 + 22 + 33";
//!
//! let lexer_rules = lexer();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input);
//!
//! let grammar = grammar();
//! let abstract_syntax_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! With this constraint our grammar becomes
//! [deterministic](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar).
//!
//! And we will always have a single unambiguous Abstract Syntax Tree:
//! ```text
#![doc = include_str!("../tests/integer_addition/cases/addition/forest")]
//! ```
//! # Technical details
//!
//! ## Lexical Analysis
//!
//! A Lexer splits an input of characters
//! into small groups of characters with related meaning,
//! while discarding irrelevant characters like whitespace.
//!
//! For example: `1 + 2` is transformed into: `[INT, PLUS, INT]`.
//!
//! A lexer analyzes its input
//! by looking for strings which match any of its active rules:
//! - If it finds more than one match,
//!   it takes the one matching the most text.
//! - If it finds two or more matches of the same length,
//!   the rule listed first is chosen.
//! - A rule is considered active if any of its applicable states
//!   matches the current state.
//!
//! Once the match is determined the corresponding rule action is executed,
//! which can in turn:
//! - Retrieve the current matched string with
//!   [matched](lexer::Lexer::matched()).
//! - Manipulate the states stack with
//!   [push_state](lexer::Lexer::push_state()) and
//!   [pop_state](lexer::Lexer::pop_state()).
//! - And finally [take](lexer::Lexer::take()),
//!   [skip](lexer::Lexer::skip()),
//!   [take_and_retry](lexer::Lexer::take_and_retry()),
//!   [skip_and_retry](lexer::Lexer::skip_and_retry()),
//!   the current match,
//!   or signal an [error](lexer::Lexer::error()).
//!
//! For convenience, the stack of states is initially populated with `"INITIAL"`.
//!
//! ## Example: Smallest lexer possible
//!
//! This lexer will copy char by char the input
//! ```rust
//! let input = "abc";
//!
//! let lexer_rules = santiago::lexer::LexerBuilder::new()
//!     .pattern(&["INITIAL"], "CHAR", ".", |lexer| lexer.take())
//!     .finish();
//!
//! let lexemes = santiago::lexer::lex(&lexer_rules, input);
//!
//! assert_eq!(
//!     vec![
//!         // kind raw (line, column)
//!         r#"CHAR "a" (1, 1)"#,
//!         r#"CHAR "b" (1, 2)"#,
//!         r#"CHAR "c" (1, 3)"#,
//!     ],
//!     lexemes
//!         .iter()
//!         .map(santiago::lexer::Lexeme::to_string)
//!         .collect::<Vec<String>>()
//! );
//! ```
//! 
//! ## Example: JavaScript string interpolations:
//! ```rust
//! let input = "    `a${ variable }b`    ";
//!
//! let lexer_rules = santiago::lexer::LexerBuilder::new()
//!     // If the current state is "INITIAL",
//!     // associate a "`" with the beginning of the string,
//!     // and make the current state be "INSIDE_STRING".
//!     .string(&["INITIAL"], "STRING_START", "`", |lexer| {
//!         lexer.push_state("INSIDE_STRING");
//!         lexer.take()
//!     })
//!     // If the current state is "INSIDE_STRING"
//!     // associate "${" with nothing,
//!     // make the current state be "INSIDE_STRING_INTERPOLATION"
//!     // and skip the current match.
//!     .string(&["INSIDE_STRING"], "", "${", |lexer| {
//!         lexer.push_state("INSIDE_STRING_INTERPOLATION");
//!         lexer.skip()
//!     })
//!     // If the current state is "INSIDE_STRING_INTERPOLATION"
//!     // associate one or more latin letters to a variable.
//!     .pattern(&["INSIDE_STRING_INTERPOLATION"], "VAR", "[a-z]+", |lexer| {
//!         lexer.take()
//!     })
//!     // If the current state is "INSIDE_STRING_INTERPOLATION"
//!     // associate a "}" with nothing,
//!     // and skip the current match.
//!     .string(&["INSIDE_STRING_INTERPOLATION"], "STR", "}", |lexer| {
//!         lexer.pop_state();
//!         lexer.skip()
//!     })
//!     // If the current state is "INSIDE_STRING",
//!     // associate a "`" with the end of the string
//!     // and go back to the previous state.
//!     .string(&["INSIDE_STRING"], "STRING_END", "`", |lexer| {
//!         lexer.pop_state();
//!         lexer.take()
//!     })
//!     // If the current state is "INSIDE_STRING"
//!     // associate anything with a "STR".
//!     //
//!     // Note how the "`" in the previous rule takes precedence over this one.
//!     .pattern(&["INSIDE_STRING"], "STR", ".", |lexer| lexer.take())
//!     // If the current state is "INITIAL" or "INSIDE_STRING_INTERPOLATION"
//!     // associate a " " with whitespace, and skip it.
//!     .string(
//!         &["INITIAL", "INSIDE_STRING_INTERPOLATION"],
//!         "WS",
//!         " ",
//!         |lexer| lexer.skip(),
//!     )
//!     .finish();
//!
//! let lexemes = santiago::lexer::lex(&lexer_rules, input);
//!
//! assert_eq!(
//!     vec![
//!         // kind raw (line, column)
//!         r#"STRING_START "`" (1, 5)"#,
//!         r#"STR "a" (1, 6)"#,
//!         r#"VAR "variable" (1, 10)"#,
//!         r#"STR "b" (1, 20)"#,
//!         r#"STRING_END "`" (1, 21)"#,
//!     ],
//!     lexemes
//!         .iter()
//!         .map(santiago::lexer::Lexeme::to_string)
//!         .collect::<Vec<String>>()
//! );
//! ```
//! # Grammars
//!
//! A [Grammar](https://en.wikipedia.org/wiki/Formal_grammar)
//! is a simple way of describing a language,
//! like `JSON`, `TOML`, `YAML`, `Python`, `Go`, or `Rust`.
//!
//! Regular use of this module uses a [GrammarBuilder](grammar::GrammarBuilder)
//! to construct a list of [GrammarRule](grammar::GrammarRule).
//!
//! For example, a language that recognizes "numbers",
//! which can be of integer or float type:
//! ```rust
//! santiago::grammar::GrammarBuilder::new()
//!     // A `number` can be an `int` or a `float`
//!     .rule_to_rules("number", &["int"])
//!     .rule_to_rules("number", &["float"])
//!     // An `int` comes from a lexeme of kind `INT`:
//!     .rule_to_lexemes("int", &["INT"])
//!     // An `float` comes from a lexeme of kind `FLOAT`:
//!     .rule_to_lexemes("float", &["FLOAT"])
//!     .finish();
//! ```
//! 
//! This module checks for grammar correctness.
//! For instance, the following example references a rule
//! that is not defined later in the grammar,
//! which constitutes an error:
//! ```should_panic
//! // Map rule `A` to `B` and don't define rule B:
//! santiago::grammar::GrammarBuilder::new()
//!     .rule_to_rules("A", &["B"])
//!     .finish();
//! ```
pub mod grammar;
pub mod languages;
pub mod lexer;
pub mod parser;

/// Create reusable definitions.
///
/// # Example
///
/// Reuse regular expressions.
///
/// ```rust
/// santiago::def!(INT, r"\d+"); // 1 or more digits.
/// santiago::def!(SIGN, r"[+-]?"); // either "+" or "-", optional.
/// santiago::def!(SIGNED_INT, concat!(SIGN!(), INT!())); // A sign, then an integer.
///
/// assert_eq!(SIGNED_INT!(), r"[+-]?\d+");
/// ```
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
