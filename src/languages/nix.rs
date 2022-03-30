// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Lexer and Parser for the [Nix expression language](https://nixos.org/).
//!
//! ```rust
//! let input = "let var = 123; in var";
//!
//! let lexer_rules = santiago::languages::nix::lexer_rules();
//! let lexemes = santiago::lexer::lex(&lexer_rules, input).unwrap();
//!
//! assert_eq!(
//!     vec![
//!         // kind raw (line, column)
//!         r#"LET "let" (1, 1)"#,
//!         r#"ID "var" (1, 5)"#,
//!         r#"= "=" (1, 9)"#,
//!         r#"INT "123" (1, 11)"#,
//!         r#"; ";" (1, 14)"#,
//!         r#"IN "in" (1, 16)"#,
//!         r#"ID "var" (1, 19)"#,
//!     ],
//!     lexemes
//!         .iter()
//!         .map(santiago::lexer::Lexeme::to_string)
//!         .collect::<Vec<String>>()
//! );
//!
//! let grammar = santiago::languages::nix::grammar();
//!
//! let ast = &santiago::parser::parse(&grammar, &lexemes).unwrap()[0];
//! assert_eq!(
//!     vec![
//!         r#"expr_function"#,
//!         r#"  LET"#,
//!         r#"    LET "let" (1, 1)"#,
//!         r#"  binds"#,
//!         r#"    binds"#,
//!         r#"    ID"#,
//!         r#"      ID "var" (1, 5)"#,
//!         r#"    ="#,
//!         r#"      = "=" (1, 9)"#,
//!         r#"    INT"#,
//!         r#"      INT "123" (1, 11)"#,
//!         r#"    ;"#,
//!         r#"      ; ";" (1, 14)"#,
//!         r#"  IN"#,
//!         r#"    IN "in" (1, 16)"#,
//!         r#"  ID"#,
//!         r#"    ID "var" (1, 19)"#,
//!     ],
//!     ast.to_string().lines().collect::<Vec<&str>>(),
//! );
//! ```

use crate::def;
use crate::grammar::Associativity;
use crate::grammar::Grammar;
use crate::grammar::GrammarBuilder;
use crate::lexer::LexerBuilder;
use crate::lexer::LexerRules;

def!(ANY, r".|\n");
def!(ID, r"[a-zA-Z_][a-zA-Z0-9_'\-]*");
def!(INT, r"[0-9]+");
def!(FLOAT, r"(([1-9][0-9]*\.[0-9]*)|(0?\.[0-9]+))([Ee][+-]?[0-9]+)?");
def!(PATH_CHAR, r"[a-zA-Z0-9\._\-\+]");
def!(PATH, concat!(PATH_CHAR!(), r"*(/", PATH_CHAR!(), r"+)+/?"));
def!(PATH_SEG, concat!(PATH_CHAR!(), r"*/"));
def!(HPATH, concat!(r"\~(/", PATH_CHAR!(), r"+)+/?"));
def!(HPATH_START, r"\~/");
def!(SPATH, concat!(r"<", PATH_CHAR!(), r"+(/", PATH_CHAR!(), r"+)*>"));
def!(URI, r"[a-zA-Z][a-zA-Z0-9\+\-\.]*:[a-zA-Z0-9%/\?:@\&=\+\$,\-_\.!\~\*']+");

/// Build a set of lexer rules for The Nix Expression Language.
pub fn lexer_rules() -> LexerRules {
    LexerBuilder::new()
        .string(&["DEFAULT"], "IF", "if", |lexer| lexer.take())
        .string(&["DEFAULT"], "THEN", "then", |lexer| lexer.take())
        .string(&["DEFAULT"], "ELSE", "else", |lexer| lexer.take())
        .string(&["DEFAULT"], "ASSERT", "assert", |lexer| lexer.take())
        .string(&["DEFAULT"], "WITH", "with", |lexer| lexer.take())
        .string(&["DEFAULT"], "LET", "let", |lexer| lexer.take())
        .string(&["DEFAULT"], "IN", "in", |lexer| lexer.take())
        .string(&["DEFAULT"], "REC", "rec", |lexer| lexer.take())
        .string(&["DEFAULT"], "INHERIT", "inherit", |lexer| lexer.take())
        .string(&["DEFAULT"], "OR_KW", "or", |lexer| lexer.take())
        .string(&["DEFAULT"], "ELLIPSIS", "...", |lexer| lexer.take())
        .string(&["DEFAULT"], "EQ", "==", |lexer| lexer.take())
        .string(&["DEFAULT"], "NEQ", "!=", |lexer| lexer.take())
        .string(&["DEFAULT"], "LEQ", "<=", |lexer| lexer.take())
        .string(&["DEFAULT"], "GEQ", ">=", |lexer| lexer.take())
        .string(&["DEFAULT"], "AND", "&&", |lexer| lexer.take())
        .string(&["DEFAULT"], "OR", "||", |lexer| lexer.take())
        .string(&["DEFAULT"], "IMPL", "->", |lexer| lexer.take())
        .string(&["DEFAULT"], "UPDATE", "//", |lexer| lexer.take())
        .string(&["DEFAULT"], "CONCAT", "++", |lexer| lexer.take())
        .pattern(&["DEFAULT"], "ID", ID!(), |lexer| lexer.take())
        .pattern(&["DEFAULT"], "INT", INT!(), |lexer| lexer.take())
        .pattern(&["DEFAULT"], "FLOAT", FLOAT!(), |lexer| lexer.take())
        .string(&["DEFAULT"], "DOLLAR_CURLY", "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .string(&["DEFAULT"], "}", "}", |lexer| {
            lexer.pop_state();
            lexer.take()
        })
        .string(&["DEFAULT"], "{", "{", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .string(&["DEFAULT"], "\"", "\"", |lexer| {
            lexer.push_state("STRING");
            lexer.take()
        })
        .pattern(
            &["STRING"],
            "STR",
            concat!(
                r#"([^\$"\\]|\$[^\{"\\]|\\"#,
                ANY!(),
                r"|\$\\",
                ANY!(),
                r#")*\$/""#
            ),
            |lexer| lexer.take_and_map(unescape_string),
        )
        .pattern(
            &["STRING"],
            "STR",
            concat!(
                r#"([^\$"\\]|\$[^\{"\\]|\\"#,
                ANY!(),
                r"|\$\\",
                ANY!(),
                r")+"
            ),
            |lexer| lexer.take_and_map(unescape_string),
        )
        .string(&["STRING"], "DOLLAR_CURLY", "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .string(&["STRING"], "\"", "\"", |lexer| {
            lexer.pop_state();
            lexer.take()
        })
        .pattern(&["STRING"], "STR", r"\$|\\|\$\\", |lexer| lexer.take())
        .pattern(&["DEFAULT"], "IND_STRING_OPEN", r"''( *\n)?", |lexer| {
            lexer.push_state("IND_STRING");
            lexer.take()
        })
        .pattern(
            &["IND_STRING"],
            "IND_STR",
            r"([^\$']|\$[^\{']|'[^'\$])+",
            |lexer| lexer.take(),
        )
        .string(&["IND_STRING"], "IND_STR", "''$", |lexer| {
            lexer.take_and_map(|_| "$".to_string())
        })
        .string(&["IND_STRING"], "IND_STR", "$", |lexer| lexer.take())
        .string(&["IND_STRING"], "IND_STR", "'''", |lexer| {
            lexer.take_and_map(|_| "''".to_string())
        })
        .pattern(
            &["IND_STRING"],
            "IND_STR",
            concat!(r"''\\", ANY!()),
            |lexer| {
                lexer.take_and_map(|matched| unescape_string(&matched[2..]))
            },
        )
        .string(&["IND_STRING"], "DOLLAR_CURLY", "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .string(&["IND_STRING"], "IND_STRING_CLOSE", "''", |lexer| {
            lexer.pop_state();
            lexer.take()
        })
        .string(&["IND_STRING"], "IND_STR", "'", |lexer| lexer.take())
        .string(&["DEFAULT"], "SKIP", concat!(PATH_SEG!(), "${"), |lexer| {
            lexer.push_state("PATH_START");
            lexer.skip_and_retry()
        })
        .string(&["DEFAULT"], "SKIP", concat!(HPATH_START!(), "${"), |lexer| {
            lexer.push_state("PATH_START");
            lexer.skip_and_retry()
        })
        .pattern(&["PATH_START"], "PATH", PATH_SEG!(), |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH_SLASH");
            lexer.take()
        })
        .pattern(&["PATH_START"], "HPATH", HPATH_START!(), |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH_SLASH");
            lexer.take()
        })
        .pattern(&["DEFAULT"], "PATH", PATH!(), |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take()
        })
        .pattern(&["DEFAULT"], "HPATH", HPATH!(), |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take()
        })
        .string(&["INPATH", "INPATH_SLASH"], "DOLLAR_CURLY", "${", |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH");
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .pattern(
            &["INPATH", "INPATH_SLASH"],
            "STR",
            concat!(PATH!(), "|", PATH_SEG!(), "|", PATH_CHAR!(), "+"),
            |lexer| {
                let matched = lexer.matched();
                if &matched[matched.len() - 1..] == "/" {
                    lexer.pop_state();
                    lexer.push_state("INPATH_SLASH");
                } else {
                    lexer.pop_state();
                    lexer.push_state("INPATH");
                }
                lexer.take()
            },
        )
        .pattern(&["INPATH"], "PATH_END", concat!(ANY!(), "|$"), |lexer| {
            lexer.pop_state();
            lexer.take_and_retry()
        })
        .pattern(&["INPATH_SLASH"], "ERROR", concat!(ANY!(), "|$"), |lexer| {
            lexer.error("Path has a trailing slash")
        })
        .string(&["DEFAULT"], "SPATH", SPATH!(), |lexer| lexer.take())
        .string(&["DEFAULT"], "URI", URI!(), |lexer| lexer.take())
        .pattern(&["DEFAULT"], "WS", r"[ \t\r\n]+", |lexer| lexer.skip())
        .pattern(&["DEFAULT"], "COMMENT", r"\#[^\r\n]*", |lexer| lexer.skip())
        .pattern(&["DEFAULT"], "COMMENT", r"/\*([^*]|\*+[^*/])*\*+/", |lexer| {
            lexer.skip()
        })
        //
        .string(&["DEFAULT"], "*", "*", |lexer| lexer.take())
        .string(&["DEFAULT"], ":", ":", |lexer| lexer.take())
        .string(&["DEFAULT"], ".", ".", |lexer| lexer.take())
        .string(&["DEFAULT"], "=", "=", |lexer| lexer.take())
        .string(&["DEFAULT"], "-", "-", |lexer| lexer.take())
        .string(&["DEFAULT"], "!", "!", |lexer| lexer.take())
        .string(&["DEFAULT"], "(", "(", |lexer| lexer.take())
        .string(&["DEFAULT"], ")", ")", |lexer| lexer.take())
        .string(&["DEFAULT"], "+", "+", |lexer| lexer.take())
        .string(&["DEFAULT"], ";", ";", |lexer| lexer.take())
        .string(&["DEFAULT"], "/", "/", |lexer| lexer.take())
        .string(&["DEFAULT"], "[", "[", |lexer| lexer.take())
        .string(&["DEFAULT"], "]", "]", |lexer| lexer.take())
        .string(&["DEFAULT"], "@", "@", |lexer| lexer.take())
        .string(&["DEFAULT"], "<", "<", |lexer| lexer.take())
        .string(&["DEFAULT"], ">", ">", |lexer| lexer.take())
        .string(&["DEFAULT"], "?", "?", |lexer| lexer.take())
        .string(&["DEFAULT"], ",", ",", |lexer| lexer.take())
        //
        .pattern(&["DEFAULT"], "ANY", ANY!(), |lexer| {
            lexer.error("Unexpected input")
        })
        .finish()
}

/// Build a set of grammar rules for The Nix Expression Language.
pub fn grammar() -> Grammar {
    let mut builder = GrammarBuilder::new();

    for (kind, rules) in &[
        (
            "expr",
            vec![
                vec!["expr_function"],
                //
            ],
        ),
        (
            "expr_function",
            vec![
                vec!["ID", ":", "expr_function"],
                vec!["{", "formals", "}", ":", "expr_function"],
                vec!["{", "formals", "}", "@", "ID", ":", "expr_function"],
                vec!["ID", "@", "{", "formals", "}", ":", "expr_function"],
                vec!["ASSERT", "expr", ";", "expr_function"],
                vec!["WITH", "expr", ";", "expr_function"],
                vec!["LET", "binds", "IN", "expr_function"],
                vec!["expr_if"],
            ],
        ),
        (
            "expr_if",
            vec![
                vec!["IF", "expr", "THEN", "expr", "ELSE", "expr"],
                vec!["expr_op"],
                //
            ],
        ),
        (
            "expr_op",
            vec![
                vec!["NOT", "expr_op"],
                vec!["NEGATE", "expr_op"],
                vec!["expr_op", "EQ", "expr_op"],
                vec!["expr_op", "NEQ", "expr_op"],
                vec!["expr_op", "<", "expr_op"],
                vec!["expr_op", "LEQ", "expr_op"],
                vec!["expr_op", ">", "expr_op"],
                vec!["expr_op", "GEQ", "expr_op"],
                vec!["expr_op", "AND", "expr_op"],
                vec!["expr_op", "OR", "expr_op"],
                vec!["expr_op", "IMPL", "expr_op"],
                vec!["expr_op", "UPDATE", "expr_op"],
                vec!["expr_op", "?", "attrpath"],
                vec!["expr_op", "+", "expr_op"],
                vec!["expr_op", "-", "expr_op"],
                vec!["expr_op", "*", "expr_op"],
                vec!["expr_op", "/", "expr_op"],
                vec!["expr_op", "CONCAT", "expr_op"],
                vec!["expr_app"],
            ],
        ),
        (
            "expr_app",
            vec![
                vec!["expr_app", "expr_select"],
                vec!["expr_select"],
                //
            ],
        ),
        (
            "expr_select",
            vec![
                vec!["expr_simple", ".", "attrpath"],
                vec!["expr_simple", ".", "attrpath", "OR_KW", "expr_select"],
                vec!["expr_simple", "OR_KW"],
                vec!["expr_simple"],
            ],
        ),
        (
            "expr_simple",
            vec![
                vec!["ID"],
                vec!["INT"],
                vec!["FLOAT"],
                vec!["\"", "string_parts", "\""],
                vec!["IND_STRING_OPEN", "ind_string_parts", "IND_STRING_CLOSE"],
                vec!["path_start", "PATH_END"],
                vec!["path_start", "string_parts_interpolated", "PATH_END"],
                vec!["SPATH"],
                vec!["URI"],
                vec!["(", "expr", ")"],
                vec!["LET", "{", "binds", "}"],
                vec!["REC", "{", "binds", "}"],
                vec!["{", "binds", "}"],
                vec!["[", "expr_list", "]"],
            ],
        ),
        (
            "string_parts",
            vec![
                vec!["STR"],
                vec!["string_parts_interpolated"],
                vec![],
                //
            ],
        ),
        (
            "string_parts_interpolated",
            vec![
                vec!["string_parts_interpolated", "STR"],
                vec!["string_parts_interpolated", "DOLLAR_CURLY", "expr", "}"],
                vec!["DOLLAR_CURLY", "expr", "}"],
                vec!["STR", "DOLLAR_CURLY", "expr", "}"],
            ],
        ),
        (
            "path_start",
            vec![
                vec!["PATH"],
                vec!["HPATH"],
                //
            ],
        ),
        (
            "ind_string_parts",
            vec![
                vec!["ind_string_parts", "IND_STR"],
                vec!["ind_string_parts", "DOLLAR_CURLY", "expr", "}"],
                vec![],
            ],
        ),
        (
            "binds",
            vec![
                vec!["binds", "attrpath", "=", "expr", ";"],
                vec!["binds", "INHERIT", "attrs", ";"],
                vec!["binds", "INHERIT", "(", "expr", ")", "attrs", ";"],
                vec![],
            ],
        ),
        (
            "attrs",
            vec![
                vec!["attrs", "attr"],
                vec!["attrs", "string_attr"],
                vec![],
                //
            ],
        ),
        (
            "attrpath",
            vec![
                vec!["attrpath", ".", "attr"],
                vec!["attrpath", ".", "string_attr"],
                vec!["attr"],
                vec!["string_attr"],
            ],
        ),
        (
            "attr",
            vec![
                vec!["ID"],
                vec!["OR_KW"],
                //
            ],
        ),
        (
            "string_attr",
            vec![
                vec!["\"", "string_parts", "\""],
                vec!["DOLLAR_CURLY", "expr", "}"],
                //
            ],
        ),
        (
            "expr_list",
            vec![
                vec!["expr_list", "expr_select"],
                vec![], //
            ],
        ),
        (
            "formals",
            vec![
                vec!["formal", ",", "formals"],
                vec!["formal"],
                vec!["ELLIPSIS"],
                vec![],
                //
            ],
        ),
        (
            "formal",
            vec![
                vec!["ID"],
                vec!["ID", "?", "expr"],
                //
            ],
        ),
    ] {
        for rule in rules.iter() {
            builder.rule_to_rules(kind, rule);
        }
    }

    for lexeme_kind in &[
        "!",
        "\"",
        "(",
        ")",
        "*",
        "+",
        ",",
        ".",
        "/",
        ":",
        ";",
        "<",
        "=",
        ">",
        "?",
        "@",
        "[",
        "]",
        "{",
        "}",
        "AND",
        "ANY",
        "ASSERT",
        "COMMENT",
        "CONCAT",
        "DOLLAR_CURLY",
        "ELLIPSIS",
        "ELSE",
        "EQ",
        "ERROR",
        "FLOAT",
        "GEQ",
        "HPATH",
        "ID",
        "IF",
        "IMPL",
        "IN",
        "IND_STR",
        "IND_STRING_CLOSE",
        "IND_STRING_OPEN",
        "INHERIT",
        "INT",
        "LEQ",
        "LET",
        "NEQ",
        "OR",
        "OR_KW",
        "PATH",
        "PATH_END",
        "REC",
        "SKIP",
        "SPATH",
        "STR",
        "THEN",
        "UPDATE",
        "URI",
        "WITH",
        "WS",
    ] {
        builder.rule_to_lexemes(lexeme_kind, &[lexeme_kind]);
    }
    builder.rule_to_lexemes("NOT", &["!"]);
    builder.rule_to_lexemes("NEGATE", &["-"]);
    builder.rule_to_lexemes("-", &["-"]);

    builder.disambiguate(Associativity::Right, &["IMPL"]);
    builder.disambiguate(Associativity::Left, &["OR"]);
    builder.disambiguate(Associativity::Left, &["AND"]);
    builder.disambiguate(Associativity::None, &["EQ", "NEQ"]);
    builder.disambiguate(Associativity::None, &["<", ">", "LEQ", "GEQ"]);
    builder.disambiguate(Associativity::Right, &["UPDATE"]);
    builder.disambiguate(Associativity::Left, &["NOT"]);
    builder.disambiguate(Associativity::Left, &["+", "-"]);
    builder.disambiguate(Associativity::Left, &["*", "/"]);
    builder.disambiguate(Associativity::Right, &["CONCAT"]);
    builder.disambiguate(Associativity::None, &["?"]);
    builder.disambiguate(Associativity::None, &["NEGATE"]);

    builder.finish()
}

fn unescape_string(input: &str) -> String {
    let mut input_chars = input.chars().peekable();
    let mut output = String::new();

    loop {
        let input_char = input_chars.next();

        if input_char.is_none() {
            break;
        }

        let mut input_char = input_char.unwrap();

        match input_char {
            '\\' => {
                input_char = input_chars.next().unwrap();

                if input_char == 'n' {
                    output.push('\n');
                } else if input_char == 'r' {
                    output.push('\r');
                } else if input_char == 't' {
                    output.push('\t');
                } else {
                    output.push(input_char);
                }
            }
            '\r' => {
                output.push('\n');
                input_chars.next_if(|s| *s == '\n');
            }
            c => {
                output.push(c);
            }
        }
    }

    output
}
