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
//! - [Calculator](languages::calculator)
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
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
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
//! <plus> ::= "PLUS"
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
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
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
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let abstract_syntax_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! With this constraint our grammar becomes
//! [deterministic](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar)
//! and we will always have a single unambiguous Abstract Syntax Tree:
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
//! For convenience, the stack of states is initially populated with `"DEFAULT"`.
//!
//! ## Example: Smallest lexer possible
//!
//! This lexer will copy char by char the input:
//! ```rust
#![doc = include_str!("../tests/smallest/lexer.rs")]
//! ```
//! 
//! For example:
//! ```rust
//! # mod m {
//! #   include!("../tests/smallest/lexer.rs");
//! # }
//! # use m::*;
//! let input = "abcd";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! ```
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/smallest/cases/multiple/lexemes")]
//! ```
//! 
//! ## Example: JavaScript string interpolations:
//! This lexer can handle strings interpolations in the form:
//! - ``` `Hello ${ name }, your age is: ${ age }.` ```
//!
//! Similar to those you find in many programming languages.
//! ```rust
#![doc = include_str!("../tests/javascript_string_interpolation/lexer.rs")]
//! ```
//! 
//! For example:
//! ```rust
//! # mod m {
//! #   include!("../tests/javascript_string_interpolation/lexer.rs");
//! # }
//! # use m::*;
//! let input = "`a${ b }c${ d }e`";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! ```
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/javascript_string_interpolation/cases/multiple/lexemes")]
//! ```
//! 
//! # Grammars
//!
//! A [Grammar](https://en.wikipedia.org/wiki/Formal_grammar)
//! is a simple way of describing a language,
//! like `JSON`, `TOML`, `YAML`, `Python`, `Go`, or `Rust`.
//!
//! Grammars are commonly described in [Backus–Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).
//! Mapping this syntax to Santiago is very straight forward.
//!
//! ## Example: Smallest grammar possible
//!
//! Note: This example is a continuation of the same example in the lexer section.
//!
//! Let's build a grammar for a sequence of characters:
//! ```rust
#![doc = include_str!("../tests/smallest/grammar.rs")]
//! ```
//! And parse!
//! ```rust
//! # mod m {
//! #   include!("../tests/smallest/grammar.rs");
//! #   include!("../tests/smallest/lexer.rs");
//! # }
//! # use m::*;
//! let input = "abcd";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let abstract_syntax_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/smallest/cases/multiple/forest")]
//! ```
//! 
//! ## Example: JavaScript string interpolations:
//! Note: This example is a continuation of the same example in the lexer section.
//!
//! Let's build a grammar for strings interpolations in the form:
//! - ``` `Hello ${ name }, your age is: ${ age }.` ```
//! ```rust
#![doc = include_str!("../tests/javascript_string_interpolation/grammar.rs")]
//! ```
//! 
//! And parse!
//! ```rust
//! # mod m {
//! #   include!("../tests/javascript_string_interpolation/grammar.rs");
//! #   include!("../tests/javascript_string_interpolation/lexer.rs");
//! # }
//! # use m::*;
//! let input = "`a${ b }c${ d }e`";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let abstract_syntax_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/javascript_string_interpolation/cases/multiple/forest")]
//! ```
//! 
//! # Next steps
//!
//! This tutorial ends here,
//! you should now have everything to lex and parse the world!
//!
//! You can checkout more examples in the tests folder:
//! - <https://github.com/kamadorueda/santiago/tree/main/tests>
//!
//! And real life lexers and grammars in the built-in languages section:
//! - <https://github.com/kamadorueda/santiago/tree/main/src/languages>
//!
//! We hope you find Santiago useful!
//!
//! And don't forget to give us a star ⭐
//! - <https://github.com/kamadorueda/santiago>
//!
//! Cheers ❤️
pub mod grammar;
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
