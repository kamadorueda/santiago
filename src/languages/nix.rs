// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::grammar_builder::GrammarBuilder;
use crate::grammar::grammar_rule::GrammarRule;
use crate::lexer::lexer_builder::LexerBuilder;
use crate::lexer::lexer_rule::LexerRule;

macro_rules! ANY {
    () => {
        r".|\n"
    };
}
macro_rules! ID {
    () => {
        r"[a-zA-Z_][a-zA-Z0-9_'\-]*"
    };
}
macro_rules! INT {
    () => {
        r"[0-9]+"
    };
}
macro_rules! FLOAT {
    () => {
        r"(([1-9][0-9]*\.[0-9]*)|(0?\.[0-9]+))([Ee][+-]?[0-9]+)?"
    };
}
macro_rules! PATH_CHAR {
    () => {
        r"[a-zA-Z0-9\._\-\+]"
    };
}
macro_rules! PATH {
    () => {
        concat!(PATH_CHAR!(), r"*(/", PATH_CHAR!(), r"+)+/?")
    };
}
macro_rules! PATH_SEG {
    () => {
        concat!(PATH_CHAR!(), r"*/")
    };
}
macro_rules! HPATH {
    () => {
        concat!(r"\~(/", PATH_CHAR!(), r"+)+/?")
    };
}
macro_rules! HPATH_START {
    () => {
        r"\~/"
    };
}
macro_rules! SPATH {
    () => {
        concat!(r"<", PATH_CHAR!(), r"+(/", PATH_CHAR!(), r"+)*>")
    };
}
macro_rules! URI {
    () => {
        r"[a-zA-Z][a-zA-Z0-9\+\-\.]*:[a-zA-Z0-9%/\?:@\&=\+\$,\-_\.!\~\*']+"
    };
}

pub fn lexing_rules() -> Vec<LexerRule> {
    LexerBuilder::new()
        .string(&["DEFAULT", "INITIAL"], "IF", "if", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "THEN", "then", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "ELSE", "else", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "ASSERT", "assert", |lexer| {
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "WITH", "with", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "LET", "let", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "IN", "in", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "REC", "rec", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "INHERIT", "inherit", |lexer| {
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "OR_KW", "or", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "ELLIPSIS", "...", |lexer| {
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "EQ", "==", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "NEQ", "!=", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "LEQ", "<=", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "GEQ", ">=", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "AND", "&&", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "OR", "||", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "IMPL", "->", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "UPDATE", "//", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "CONCAT", "++", |lexer| lexer.take())
        .pattern(&["DEFAULT", "INITIAL"], "ID", ID!(), |lexer| lexer.take())
        .pattern(&["DEFAULT", "INITIAL"], "INT", INT!(), |lexer| lexer.take())
        .pattern(&["DEFAULT", "INITIAL"], "FLOAT", FLOAT!(), |lexer| {
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "DOLLAR_CURLY", "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "}", "}", |lexer| {
            if lexer.current_state() != "INITIAL" {
                lexer.pop_state();
            }
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "{", "{", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "\"", "\"", |lexer| {
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
        .pattern(
            &["DEFAULT", "INITIAL"],
            "IND_STRING_OPEN",
            r"''( *\n)?",
            |lexer| {
                lexer.push_state("IND_STRING");
                lexer.take()
            },
        )
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
        .string(
            &["DEFAULT", "INITIAL"],
            "SKIP",
            concat!(PATH_SEG!(), "${"),
            |lexer| {
                lexer.push_state("PATH_START");
                lexer.skip_and_retry()
            },
        )
        .string(
            &["DEFAULT", "INITIAL"],
            "SKIP",
            concat!(HPATH_START!(), "${"),
            |lexer| {
                lexer.push_state("PATH_START");
                lexer.skip_and_retry()
            },
        )
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
        .pattern(&["DEFAULT", "INITIAL"], "PATH", PATH!(), |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take()
        })
        .pattern(&["DEFAULT", "INITIAL"], "HPATH", HPATH!(), |lexer| {
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
        .string(&["DEFAULT", "INITIAL"], "SPATH", SPATH!(), |lexer| {
            lexer.take()
        })
        .string(&["DEFAULT", "INITIAL"], "URI", URI!(), |lexer| lexer.take())
        .pattern(&["DEFAULT", "INITIAL"], "WS", r"[ \t\r\n]+", |lexer| {
            lexer.skip()
        })
        .pattern(&["DEFAULT", "INITIAL"], "COMMENT", r"\#[^\r\n]*", |lexer| {
            lexer.skip()
        })
        .pattern(
            &["DEFAULT", "INITIAL"],
            "COMMENT",
            r"/\*([^*]|\*+[^*/])*\*+/",
            |lexer| lexer.skip(),
        )
        //
        .string(&["DEFAULT", "INITIAL"], "*", "*", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], ":", ":", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], ".", ".", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "=", "=", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "-", "-", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "!", "!", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "(", "(", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], ")", ")", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "+", "+", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], ";", ";", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "/", "/", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "[", "[", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "]", "]", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "@", "@", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "<", "<", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], ">", ">", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], "?", "?", |lexer| lexer.take())
        .string(&["DEFAULT", "INITIAL"], ",", ",", |lexer| lexer.take())
        //
        .pattern(&["DEFAULT", "INITIAL"], "ANY", ANY!(), |lexer| {
            lexer.error("Unexpected input")
        })
        .finish()
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
