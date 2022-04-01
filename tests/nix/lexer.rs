use santiago::lexer::LexerRules;

santiago::def!(ANY, r"(?:.|\n)");
santiago::def!(ID, r"[a-zA-Z_][a-zA-Z0-9_'\-]*");
santiago::def!(INT, r"[0-9]+");
santiago::def!(
    FLOAT,
    r"(([1-9][0-9]*\.[0-9]*)|(0?\.[0-9]+))([Ee][+-]?[0-9]+)?"
);
santiago::def!(PATH_CHAR, r"[a-zA-Z0-9\._\-\+]");
santiago::def!(PATH, concat!(PATH_CHAR!(), r"*(/", PATH_CHAR!(), r"+)+/?"));
santiago::def!(PATH_SEG, concat!(PATH_CHAR!(), r"*/"));
santiago::def!(HPATH, concat!(r"\~(/", PATH_CHAR!(), r"+)+/?"));
santiago::def!(HPATH_START, r"\~/");
santiago::def!(
    SPATH,
    concat!(r"<", PATH_CHAR!(), r"+(/", PATH_CHAR!(), r"+)*>")
);
santiago::def!(
    URI,
    r"[a-zA-Z][a-zA-Z0-9\+\-\.]*:[a-zA-Z0-9%/\?:@\&=\+\$,\-_\.!\~\*']+"
);

pub fn lexer_rules() -> LexerRules {
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
            r#")*\$""#
        )
        => |lexer| {
            lexer.current_match_len -= 1;
            lexer.take_and_map(unescape_string)
        };
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
