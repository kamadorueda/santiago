use santiago::grammar::Associativity;
use santiago::grammar::Grammar;
use santiago::grammar::GrammarBuilder;

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
