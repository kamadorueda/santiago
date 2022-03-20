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
        .string(&["DEFAULT", "INITIAL"], "if", |lexer| lexer.take("IF"))
        .string(&["DEFAULT", "INITIAL"], "then", |lexer| lexer.take("THEN"))
        .string(&["DEFAULT", "INITIAL"], "else", |lexer| lexer.take("ELSE"))
        .string(&["DEFAULT", "INITIAL"], "assert", |lexer| lexer.take("ASSERT"))
        .string(&["DEFAULT", "INITIAL"], "with", |lexer| lexer.take("WITH"))
        .string(&["DEFAULT", "INITIAL"], "let", |lexer| lexer.take("LET"))
        .string(&["DEFAULT", "INITIAL"], "in", |lexer| lexer.take("IN"))
        .string(&["DEFAULT", "INITIAL"], "rec", |lexer| lexer.take("REC"))
        .string(&["DEFAULT", "INITIAL"], "inherit", |lexer| {
            lexer.take("INHERIT")
        })
        .string(&["DEFAULT", "INITIAL"], "or", |lexer| lexer.take("OR_KW"))
        .string(&["DEFAULT", "INITIAL"], "...", |lexer| lexer.take("ELLIPSIS"))
        .string(&["DEFAULT", "INITIAL"], "==", |lexer| lexer.take("EQ"))
        .string(&["DEFAULT", "INITIAL"], "!=", |lexer| lexer.take("NEQ"))
        .string(&["DEFAULT", "INITIAL"], "<=", |lexer| lexer.take("LEQ"))
        .string(&["DEFAULT", "INITIAL"], ">=", |lexer| lexer.take("GEQ"))
        .string(&["DEFAULT", "INITIAL"], "&&", |lexer| lexer.take("AND"))
        .string(&["DEFAULT", "INITIAL"], "||", |lexer| lexer.take("OR"))
        .string(&["DEFAULT", "INITIAL"], "->", |lexer| lexer.take("IMPL"))
        .string(&["DEFAULT", "INITIAL"], "//", |lexer| lexer.take("UPDATE"))
        .string(&["DEFAULT", "INITIAL"], "++", |lexer| lexer.take("CONCAT"))
        .pattern(&["DEFAULT", "INITIAL"], ID!(), |lexer| lexer.take("ID"))
        .pattern(&["DEFAULT", "INITIAL"], INT!(), |lexer| lexer.take("INT"))
        .pattern(&["DEFAULT", "INITIAL"], FLOAT!(), |lexer| lexer.take("float"))
        .string(&["DEFAULT", "INITIAL"], "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take("DOLLAR_CURLY")
        })
        .string(&["DEFAULT", "INITIAL"], "}", |lexer| {
            if lexer.current_state() != "INITIAL" {
                lexer.pop_state();
            }
            lexer.take("}")
        })
        .string(&["DEFAULT", "INITIAL"], "{", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take("{")
        })
        .string(&["DEFAULT", "INITIAL"], "\"", |lexer| {
            lexer.push_state("STRING");
            lexer.take("\"")
        })
        .pattern(
            &["STRING"],
            concat!(
                r#"([^\$"\\]|\$[^\{"\\]|\\"#,
                ANY!(),
                r"|\$\\",
                ANY!(),
                r#")*\$/""#
            ),
            |lexer| lexer.take_and_map("STR", unescape_string),
        )
        .pattern(
            &["STRING"],
            concat!(
                r#"([^\$"\\]|\$[^\{"\\]|\\"#,
                ANY!(),
                r"|\$\\",
                ANY!(),
                r")+"
            ),
            |lexer| lexer.take_and_map("STR", unescape_string),
        )
        .string(&["STRING"], "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take("DOLLAR_CURLY")
        })
        .string(&["STRING"], "\"", |lexer| {
            lexer.pop_state();
            lexer.take("\"")
        })
        .pattern(&["STRING"], r"\$|\\|\$\\", |lexer| lexer.take("STR"))
        .pattern(&["DEFAULT", "INITIAL"], r"''( *\n)?", |lexer| {
            lexer.push_state("IND_STRING");
            lexer.take("IND_STRING_OPEN")
        })
        .pattern(&["IND_STRING"], r"([^\$']|\$[^\{']|'[^'\$])+", |lexer| {
            lexer.take("IND_STR")
        })
        .string(&["IND_STRING"], "''$", |lexer| {
            lexer.take_and_map("IND_STR", |_| "$".to_string())
        })
        .string(&["IND_STRING"], "$", |lexer| lexer.take("IND_STR"))
        .string(&["IND_STRING"], "'''", |lexer| {
            lexer.take_and_map("IND_STR", |_| "''".to_string())
        })
        .pattern(&["IND_STRING"], concat!(r"''\\", ANY!()), |lexer| {
            lexer.take_and_map("IND_STR", |matched| {
                unescape_string(&matched[2..])
            })
        })
        .string(&["IND_STRING"], "${", |lexer| {
            lexer.push_state("DEFAULT");
            lexer.take("DOLLAR_CURLY")
        })
        .string(&["IND_STRING"], "''", |lexer| {
            lexer.pop_state();
            lexer.take("IND_STRING_CLOSE")
        })
        .string(&["IND_STRING"], "'", |lexer| lexer.take("IND_STR"))
        .string(&["DEFAULT", "INITIAL"], concat!(PATH_SEG!(), "${"), |lexer| {
            lexer.push_state("PATH_START");
            lexer.skip_and_retry()
        })
        .string(
            &["DEFAULT", "INITIAL"],
            concat!(HPATH_START!(), "${"),
            |lexer| {
                lexer.push_state("PATH_START");
                lexer.skip_and_retry()
            },
        )
        .pattern(&["PATH_START"], PATH_SEG!(), |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH_SLASH");
            lexer.take("PATH")
        })
        .pattern(&["PATH_START"], HPATH_START!(), |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH_SLASH");
            lexer.take("HPATH")
        })
        .pattern(&["DEFAULT", "INITIAL"], PATH!(), |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take("PATH")
        })
        .pattern(&["DEFAULT", "INITIAL"], HPATH!(), |lexer| {
            let matched = lexer.matched();
            if &matched[matched.len() - 1..] == "/" {
                lexer.push_state("INPATH_SLASH");
            } else {
                lexer.push_state("INPATH");
            }
            lexer.take("HPATH")
        })
        .string(&["INPATH", "INPATH_SLASH"], "${", |lexer| {
            lexer.pop_state();
            lexer.push_state("INPATH");
            lexer.push_state("DEFAULT");
            lexer.take("DOLLAR_CURLY")
        })
        .pattern(
            &["INPATH", "INPATH_SLASH"],
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
                lexer.take("STR")
            },
        )
        .pattern(&["INPATH"], concat!(ANY!(), "|$"), |lexer| {
            lexer.pop_state();
            lexer.take_and_retry("PATH_END")
        })
        .pattern(&["INPATH_SLASH"], concat!(ANY!(), "|$"), |lexer| {
            lexer.error("Path has a trailing slash")
        })
        .pattern(&["DEFAULT", "INITIAL"], SPATH!(), |lexer| lexer.take("SPATH"))
        .pattern(&["DEFAULT", "INITIAL"], URI!(), |lexer| lexer.take("URI"))
        .pattern(&["DEFAULT", "INITIAL"], r"[ \t\r\n]+", |lexer| lexer.skip())
        .pattern(&["DEFAULT", "INITIAL"], r"\#[^\r\n]*", |lexer| lexer.skip())
        .pattern(&["DEFAULT", "INITIAL"], r"/\*([^*]|\*+[^*/])*\*+/", |lexer| {
            lexer.skip()
        })
        .pattern(&["DEFAULT", "INITIAL"], ANY!(), |lexer| lexer.take("OTHER"))
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
