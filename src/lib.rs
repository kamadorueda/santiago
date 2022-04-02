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
//! It provides a library for defining any
//! [context-free grammar](https://en.wikipedia.org/wiki/Context-free_grammar),
//! a [lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis) module,
//! and facilities for building evaluators of the language.
//!
//! With Santiago, you have everything you need to build your own programming
//! language, and a compiler or interpreter for it.
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
//! # Examples
//!
//! ## Calculator
//!
//! For this example
//! we are interested in lexing and parsing the addition of integer numbers
//! like:
//! - `10 + 20 + 30`
//!
//! And evaluating it to a single value: `60`.
//!
//! In the process we will create an
//! [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
//! like:
//!
//! ```text
//! BinaryOperation(vec![
//!     BinaryOperation(vec![
//!         Int(10),
//!         OperatorAdd,
//!         Int(20),
//!     ]),
//!     OperatorAdd,
//!     Int(30),
//! ])
//! ```
//!
//! So let's start with a lexer to:
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
//! let input = "10 + 20 + 30";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! ```
//! 
//! A [Lexeme](lexer::Lexeme) gives us information like:
//! - Token kind
//! - Contents
//! - Position (line and column number)
//!
//! In this case we have two kinds of tokens, an `INT` and a `PLUS`:
//! ```text
#![doc = include_str!("../tests/ambiguous_integer_addition/cases/addition/lexemes")]
//! ```
//! 
//! At this point all we are missing is creating a parser.
//!
//! Let's create a grammar to recognize the addition of integer numbers:
//! ```rust
#![doc = include_str!("../tests/ambiguous_integer_addition/grammar.rs")]
//! ```
//! 
//! Now we can generate a Parse Tree!
//! ```rust
//! # mod m {
//! #   include!("../tests/ambiguous_integer_addition/grammar.rs");
//! #   include!("../tests/ambiguous_integer_addition/lexer.rs");
//! # }
//! # use m::*;
//! # let input = "10 + 20 + 30";
//! #
//! # let lexer_rules = lexer_rules();
//! # let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! #
//! let grammar = grammar();
//! let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! And voilà!
//! ```text
#![doc = include_str!("../tests/ambiguous_integer_addition/cases/addition/parse_trees")]
//! ```
//! 
//! Notice that we obtained 2 possible abstract syntax trees,
//! since we can understand the input as:
//! - `(11 + 22) + 33`
//! - `11 + (22 + 33)`
//!
//! This happens because we created an
//! [ambiguous grammar](https://en.wikipedia.org/wiki/Ambiguous_grammar),
//! but this is no problem for Santiago!
//! We can remove the ambiguities
//! by adding associativity constraints to the "plus" rule,
//! in order to select `(11 + 22) + 33` as our source of truth.
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
//! # let input = "10 + 20 + 30";
//! #
//! # let lexer_rules = lexer_rules();
//! # let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! #
//! let grammar = grammar();
//! let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! This time our grammar is
//! [deterministic](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar)
//! and we will always have a single unambiguous Parse Tree:
//! ```text
#![doc = include_str!("../tests/integer_addition/cases/addition/parse_trees")]
//! ```
//! 
//! All we are missing now is evaluating the addition,
//! for this let's modify the grammar
//! so that each time a rule matches
//! we produce an amenable data-structure:
//! ```rust
#![doc = include_str!("../tests/integer_addition_with_value/grammar.rs")]
//! ```
//! 
//! We just need to call Santiago's builtin-function `as_abstract_syntax_tree()`.
//! ```rust
//! # mod m {
//! #   include!("../tests/integer_addition_with_value/eval.rs");
//! #   include!("../tests/integer_addition_with_value/grammar.rs");
//! #   include!("../tests/integer_addition_with_value/lexer.rs");
//! # }
//! # use m::*;
//! # let input = "10 + 20 + 30";
//! #
//! # let lexer_rules = lexer_rules();
//! # let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! #
//! # let grammar = grammar();
//! # let parse_tree = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];
//! use AST::*;
//!
//! let ast = parse_tree.as_abstract_syntax_tree();
//!
//! assert_eq!(
//!     value,
//!     BinaryOperation(vec![
//!         BinaryOperation(vec![
//!             Int(10),
//!             OperatorAdd,
//!             Int(20),
//!         ]),
//!         OperatorAdd,
//!         Int(30),
//!     ]),
//! )
//! ```
//! 
//! And now we can traverse this data-structure and compute a result:
//! ```rust
//! # mod m {
//! #   include!("../tests/integer_addition_with_value/grammar.rs");
//! # }
//! # use m::*;
//! #
#![doc = include_str!("../tests/integer_addition_with_value/eval.rs")]
//! #
//! ```
//! 
//! Like this:
//! ```rust
//! # mod m {
//! #   include!("../tests/integer_addition_with_value/eval.rs");
//! #   include!("../tests/integer_addition_with_value/grammar.rs");
//! #   include!("../tests/integer_addition_with_value/lexer.rs");
//! # }
//! # use m::*;
//! # let input = "10 + 20 + 30";
//! #
//! # let lexer_rules = lexer_rules();
//! # let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! #
//! # let grammar = grammar();
//! # let parse_tree = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];
//! #
//! let ast = parse_tree.as_abstract_syntax_tree();
//!
//! assert_eq!(eval(&value), 60);
//! ```
//! 
//! How nice is that?
//!
//! We just created:
//! - Our own programming language (a calculator)
//! - An interpreter for our language!
//!
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
//! ## Grammars
//!
//! A [Grammar](https://en.wikipedia.org/wiki/Formal_grammar)
//! is a simple way of describing a language,
//! like `JSON`, `TOML`, `YAML`, `Python`, `Go`, or `Rust`.
//! They are commonly described in
//! [Backus–Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).
//!
//! Grammars are composed of [grammar rules](grammar::GrammarRule),
//! which define how a rule can be produce other rules
//! or [Lexemes](lexer::Lexeme),
//! for example,
//! a full name is composed of a given name and a family name:
//! - `"full_name" => rules "given_name" "family_name"`
//!
//! And a given name can be "Jane" or "Kevin", and so on:
//! - `"given_name" => lexemes "Jane"`
//! - `"given_name" => lexemes "Kevin"`
//! - `"given_name" => lexemes "..."`
//!
//! # Examples
//!
//! In this section we explore a few more full examples,
//! ordered by complexity.
//!
//! ## Smallest lexer possible
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
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/smallest/cases/multiple/lexemes")]
//! ```
//! 
//! And we can build a grammar to recognize a sequence of characters:
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
//! let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/smallest/cases/multiple/parse_trees")]
//! ```
//! 
//! ## Calculator with four operations
//!
//! This lexer can handle integer arithmetic in the form:
//!
//! - `1 + 2 * 3 / 6 - 7`
//!
//! Similar to those you find in a basic calculator.
//! ```rust
#![doc = include_str!("../tests/calculator/lexer.rs")]
//! ```
//! 
//! For example:
//! ```rust
//! # mod m {
//! #   include!("../tests/calculator/lexer.rs");
//! # }
//! # use m::*;
//! let input = "1 + 2 * 3 / 6 - 7";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/calculator/cases/example/lexemes")]
//! ```
//! 
//! Now let's build a Parse Tree:
//! ```rust
#![doc = include_str!("../tests/calculator/grammar.rs")]
//! ```
//! 
//! And parse!
//! ```rust
//! # mod m {
//! #   include!("../tests/calculator/grammar.rs");
//! #   include!("../tests/calculator/lexer.rs");
//! # }
//! # use m::*;
//! let input = "1 + 2 * 3 / 6 - 7";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/calculator/cases/example/parse_trees")]
//! ```
//! 
//! We can also create an interpreter that performs the indicated
//! additions, subtractions, multiplications and divisions.
//!
//! For this let's create a more complete grammar:
//! ```rust
#![doc = include_str!("../tests/calculator_with_value/grammar.rs")]
//! ```
//! 
//! And a function to perform the arithmetic:
//! ```rust
//! # mod m {
//! #   include!("../tests/calculator_with_value/grammar.rs");
//! # }
//! # use m::*;
#![doc = include_str!("../tests/calculator_with_value/eval.rs")]
//! ```
//! 
//! Now the interpreter can be used like:
//! ```rust
//! # mod m {
//! #   include!("../tests/calculator_with_value/eval.rs");
//! #   include!("../tests/calculator_with_value/grammar.rs");
//! #   include!("../tests/calculator_with_value/lexer.rs");
//! # }
//! # use m::*;
//! let input = "1 + 2 * 3 / 6 - 7";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let parse_tree = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];
//!
//! let ast = parse_tree.as_abstract_syntax_tree();
//!
//! assert_eq!(eval(&value), -6);
//! ```
//! 
//! ## JavaScript string interpolations
//!
//! This lexer can handle strings interpolations in the form:
//!
//! - `'Hello ${ name }, your age is: ${ age }.'`
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
//! let input = "'a${ b }c${ d }e'";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/javascript_string_interpolation/cases/multiple/lexemes")]
//! ```
//! 
//! Now let's build a Parse Tree:
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
//! let input = "'a${ b }c${ d }e'";
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/javascript_string_interpolation/cases/multiple/parse_trees")]
//! ```
//! 
//! ## Nix Expression Language
//!
//! This lexer can handle the Nix expression language,
//! whose original lexer and parser is written in Flex and GNU Bison:
//! - <https://github.com/NixOS/nix/blob/9174d884d750b7b49a571bd55275f0883c2dabda/src/libexpr/lexer.l>.
//! - <https://github.com/NixOS/nix/blob/9174d884d750b7b49a571bd55275f0883c2dabda/src/libexpr/parser.y>.
//! ```rust
#![doc = include_str!("../tests/nix/lexer.rs")]
//! ```
//! 
//! Example input:
//! ```nix
#![doc = include_str!("../tests/nix/cases/pkg/input")]
//! ```
//! 
//! Let's perform lexical analysis:
//! ```rust
//! # mod m {
//! #   include!("../tests/nix/lexer.rs");
//! # }
//! # use m::*;
//! let input = include_str!("../tests/nix/cases/pkg/input");
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/nix/cases/pkg/lexemes")]
//! ```
//! 
//! Now let's build a Parse Tree:
//! ```rust
#![doc = include_str!("../tests/nix/grammar.rs")]
//! ```
//! 
//! And parse!
//! ```rust
//! # mod m {
//! #   include!("../tests/nix/grammar.rs");
//! #   include!("../tests/nix/lexer.rs");
//! # }
//! # use m::*;
//! let input = include_str!("../tests/nix/cases/pkg/input");
//!
//! let lexer_rules = lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//!
//! let grammar = grammar();
//! let parse_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//! 
//! Which outputs:
//! ```text
#![doc = include_str!("../tests/nix/cases/pkg/parse_trees")]
//! ```
//! 
//! # Next steps
//!
//! This tutorial ends here,
//! you should now have everything to lex,
//! parse, and as_abstract_syntax_tree the world,
//! and build your own programming languages, compilers and interpreters!
//!
//! You can checkout more examples in the tests folder:
//! - <https://github.com/kamadorueda/santiago/tree/main/tests>
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
