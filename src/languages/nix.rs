// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Lexer and Parser for the [Nix expression language](https://nixos.org/).
//!
//! Example usage:
//! ```rust
//! # let input = include_str!("../../tests/language_nix/cases/pkg/input");
//! let lexer_rules = santiago::languages::nix::lexer_rules();
//! let grammar = santiago::languages::nix::grammar();
//!
//! let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
//! let abstract_syntax_trees = santiago::parser::parse(&grammar, &lexemes).unwrap();
//! ```
//!
//! Example input:
//!
//! ```nix
#![doc = include_str!("../../tests/language_nix/cases/pkg/input")]
//! ```
//! 
//! Lexemes:
//! ```text
#![doc = include_str!("../../tests/language_nix/cases/pkg/lexemes")]
//! ```
//! 
//! Abstract Syntax Tree:
//! ```text
#![doc = include_str!("../../tests/language_nix/cases/pkg/forest")]
//! ```

use crate::def;
use crate::grammar::Associativity;
use crate::grammar::Grammar;
use crate::grammar::GrammarBuilder;
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
    use crate as santiago;
    santiago::lexer_rules!(
        "DEFAULT" | "IF" = string "if";
        "DEFAULT" | "THEN" = string "then";
        "DEFAULT" | "ELSE" = string "else";
        "DEFAULT" | "ASSERT" = string "assert";
        "DEFAULT" | "WITH" = string "with";
        "DEFAULT" | "LET" = string "let";
        "DEFAULT" | "IN" = string "in";
        "DEFAULT" | "REC" = string "rec";
        "DEFAULT" | "INHERIT" = string "inherit";
        "DEFAULT" | "OR_KW" = string "or";
        "DEFAULT" | "ELLIPSIS" = string "...";
        "DEFAULT" | "EQ" = string "==";
        "DEFAULT" | "NEQ" = string "!=";
        "DEFAULT" | "LEQ" = string "<=";
        "DEFAULT" | "GEQ" = string ">=";
        "DEFAULT" | "AND" = string "&&";
        "DEFAULT" | "OR" = string "||";
        "DEFAULT" | "IMPL" = string "->";
        "DEFAULT" | "UPDATE" = string "//";
        "DEFAULT" | "CONCAT" = string "++";
        "DEFAULT" | "ID" = pattern ID!();
        "DEFAULT" | "INT" = pattern INT!();
        "DEFAULT" | "FLOAT" = pattern FLOAT!();
        "DEFAULT" | "DOLLAR_CURLY" = string "${" => |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        };
        "DEFAULT" | "}" = string "}" => |lexer| {
            lexer.pop_state();
            lexer.take()
        };
        "DEFAULT" | "{" = string "{" => |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        };
        "DEFAULT" | "\"" = string "\"" => |lexer| {
            lexer.push_state("STRING");
            lexer.take()
        };
        "STRING" | "STR"
        = pattern concat!(
            r#"([^\$"\\]|\$[^\{"\\]|\\"#,
            ANY!(),
            r"|\$\\",
            ANY!(),
            r#")*\$/""#
        )
        => |lexer| lexer.take_and_map(unescape_string);
        "STRING" | "STR"
        = pattern concat!(
            r#"([^\$"\\]|\$[^\{"\\]|\\"#,
            ANY!(),
            r"|\$\\",
            ANY!(),
            r")+"
        )
        => |lexer| lexer.take_and_map(unescape_string);
        "STRING" | "DOLLAR_CURLY" = string "${" => |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        };
        "STRING" | "\"" = string "\"" => |lexer| {
            lexer.pop_state();
            lexer.take()
        };
        "STRING" | "STR" = pattern r"\$|\\|\$\\";
        "DEFAULT" | "IND_STRING_OPEN" = pattern r"''( *\n)?" => |lexer| {
            lexer.push_state("IND_STRING");
            lexer.take()
        };
        "IND_STRING" | "IND_STR" = pattern r"([^\$']|\$[^\{']|'[^'\$])+";
        "IND_STRING" | "IND_STR" = string "''$" => |lexer| {
            lexer.take_and_map(|_| "$".to_string())
        };
        "IND_STRING" | "IND_STR" = string "$";
        "IND_STRING" | "IND_STR" = string "'''" => |lexer| {
            lexer.take_and_map(|_| "''".to_string())
        };
        "IND_STRING" | "IND_STR" = pattern concat!(r"''\\", ANY!()) => |lexer| {
                lexer.take_and_map(|matched| unescape_string(&matched[2..]))
        };
        "IND_STRING" | "DOLLAR_CURLY" = string "${" => |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        };
        "IND_STRING" | "IND_STRING_CLOSE" = string "''" => |lexer| {
            lexer.pop_state();
            lexer.take()
        };
        "IND_STRING" | "IND_STR" = string "'";
        "DEFAULT" | "SKIP" = string concat!(PATH_SEG!(), "${") => |lexer| {
            lexer.push_state("PATH_START");
            lexer.skip_and_retry()
        };
        "DEFAULT" | "SKIP" = string concat!(HPATH_START!(), "${") => |lexer| {
            lexer.push_state("PATH_START");
            lexer.skip_and_retry()
        };
        "PATH_START" | "PATH" = pattern PATH_SEG!() => |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH_SLASH");
            lexer.take()
        };
        "PATH_START" | "HPATH" = pattern HPATH_START!() => |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH_SLASH");
            lexer.take()
        };
        "DEFAULT" | "PATH" = pattern PATH!() => |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take()
        };
        "DEFAULT" | "HPATH" = pattern HPATH!() => |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take()
        };
        "INPATH" "INPATH_SLASH" | "DOLLAR_CURLY" = string "${" => |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH");
            lexer.push_state("DEFAULT");
            lexer.take()
        };
        "INPATH" "INPATH_SLASH" | "STR"
        = pattern concat!(PATH!(), "|", PATH_SEG!(), "|", PATH_CHAR!(), "+")
        => |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.pop_state();
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.pop_state();
                lexer.push_state("INPATH");
            }
            lexer.take()
        };
        "INPATH" | "PATH_END" = pattern concat!(ANY!(), "|$") => |lexer| {
            lexer.pop_state();
            lexer.take_and_retry()
        };
        "INPATH_SLASH" | "ERROR" = pattern concat!(ANY!(), "|$") => |lexer| {
            lexer.error("Path has a trailing slash")
        };
        "DEFAULT" | "SPATH" = pattern SPATH!();
        "DEFAULT" | "URI" = pattern URI!();
        "DEFAULT" | "WS" = pattern r"[ \t\r\n]+" => |lexer| lexer.skip();
        "DEFAULT" | "COMMENT" = pattern r"\#[^\r\n]*" => |lexer| lexer.skip();
        "DEFAULT" | "COMMENT" = pattern r"/\*([^*]|\*+[^*/])*\*+/" => |lexer| {
            lexer.skip()
        };
        //
        "DEFAULT" | "*" = string "*";
        "DEFAULT" | ":" = string ":";
        "DEFAULT" | "." = string ".";
        "DEFAULT" | "=" = string "=";
        "DEFAULT" | "-" = string "-";
        "DEFAULT" | "!" = string "!";
        "DEFAULT" | "(" = string "(";
        "DEFAULT" | ")" = string ")";
        "DEFAULT" | "+" = string "+";
        "DEFAULT" | ";" = string ";";
        "DEFAULT" | "/" = string "/";
        "DEFAULT" | "[" = string "[";
        "DEFAULT" | "]" = string "]";
        "DEFAULT" | "@" = string "@";
        "DEFAULT" | "<" = string "<";
        "DEFAULT" | ">" = string ">";
        "DEFAULT" | "?" = string "?";
        "DEFAULT" | "," = string ",";
        //
        "DEFAULT" | "ANY" = pattern ANY!() => |lexer| {
            lexer.error("Unexpected input")
        };
    )
}

/// Build a set of grammar rules for The Nix Expression Language.
pub fn grammar() -> Grammar<()> {
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
            builder.rule_to_rules(kind, rule, |_| todo!());
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
        builder.rule_to_lexemes(lexeme_kind, &[lexeme_kind], |_| todo!());
    }
    builder.rule_to_lexemes("NOT", &["!"], |_| todo!());
    builder.rule_to_lexemes("NEGATE", &["-"], |_| todo!());
    builder.rule_to_lexemes("-", &["-"], |_| todo!());

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
