use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "expr" => rules "expr_function";

        "expr_function" => rules "ID" ":" "expr_function";
        "expr_function" => rules "{" "formals" "}" ":" "expr_function";
        "expr_function" => rules "{" "formals" "}" "@" "ID" ":" "expr_function";
        "expr_function" => rules "ID" "@" "{" "formals" "}" ":" "expr_function";
        "expr_function" => rules "ASSERT" "expr" ";" "expr_function";
        "expr_function" => rules "WITH" "expr" ";" "expr_function";
        "expr_function" => rules "LET" "binds" "IN" "expr_function";
        "expr_function" => rules "expr_if";

        "expr_if" => rules "IF" "expr" "THEN" "expr" "ELSE" "expr";
        "expr_if" => rules "expr_op";

        "expr_op" => rules "NOT" "expr_op";
        "expr_op" => rules "NEGATE" "expr_op";
        "expr_op" => rules "expr_op" "EQ" "expr_op";
        "expr_op" => rules "expr_op" "NEQ" "expr_op";
        "expr_op" => rules "expr_op" "<" "expr_op";
        "expr_op" => rules "expr_op" "LEQ" "expr_op";
        "expr_op" => rules "expr_op" ">" "expr_op";
        "expr_op" => rules "expr_op" "GEQ" "expr_op";
        "expr_op" => rules "expr_op" "AND" "expr_op";
        "expr_op" => rules "expr_op" "OR" "expr_op";
        "expr_op" => rules "expr_op" "IMPL" "expr_op";
        "expr_op" => rules "expr_op" "UPDATE" "expr_op";
        "expr_op" => rules "expr_op" "?" "attrpath";
        "expr_op" => rules "expr_op" "+" "expr_op";
        "expr_op" => rules "expr_op" "-" "expr_op";
        "expr_op" => rules "expr_op" "*" "expr_op";
        "expr_op" => rules "expr_op" "/" "expr_op";
        "expr_op" => rules "expr_op" "CONCAT" "expr_op";
        "expr_op" => rules "expr_app";

        "expr_app" => rules "expr_app" "expr_select";
        "expr_app" => rules "expr_select";

        "expr_select" => rules "expr_simple" "." "attrpath";
        "expr_select" => rules "expr_simple" "." "attrpath" "OR_KW" "expr_select";
        "expr_select" => rules "expr_simple" "OR_KW";
        "expr_select" => rules "expr_simple";

        "expr_simple" => rules "ID";
        "expr_simple" => rules "INT";
        "expr_simple" => rules "FLOAT";
        "expr_simple" => rules "\"" "string_parts" "\"";
        "expr_simple" => rules "IND_STRING_OPEN" "ind_string_parts" "IND_STRING_CLOSE";
        "expr_simple" => rules "path_start" "PATH_END";
        "expr_simple" => rules "path_start" "string_parts_interpolated" "PATH_END";
        "expr_simple" => rules "SPATH";
        "expr_simple" => rules "URI";
        "expr_simple" => rules "(" "expr" ")";
        "expr_simple" => rules "LET" "{" "binds" "}";
        "expr_simple" => rules "REC" "{" "binds" "}";
        "expr_simple" => rules "{" "binds" "}";
        "expr_simple" => rules "[" "expr_list" "]";

        "string_parts" => rules "STR";
        "string_parts" => rules "string_parts_interpolated";
        "string_parts" => empty;

        "string_parts_interpolated" => rules "string_parts_interpolated" "STR";
        "string_parts_interpolated" => rules "string_parts_interpolated" "DOLLAR_CURLY" "expr" "}";
        "string_parts_interpolated" => rules "DOLLAR_CURLY" "expr" "}";
        "string_parts_interpolated" => rules "STR" "DOLLAR_CURLY" "expr" "}";

        "path_start" => rules "PATH";
        "path_start" => rules "HPATH";

        "ind_string_parts" => rules "ind_string_parts" "IND_STR";
        "ind_string_parts" => rules "ind_string_parts" "DOLLAR_CURLY" "expr" "}";
        "ind_string_parts" => empty;

        "binds" => rules "binds" "attrpath" "=" "expr" ";";
        "binds" => rules "binds" "INHERIT" "attrs" ";";
        "binds" => rules "binds" "INHERIT" "(" "expr" ")" "attrs" ";";
        "binds" => empty;

        "attrs" => rules "attrs" "attr";
        "attrs" => rules "attrs" "string_attr";
        "attrs" => empty;

        "attrpath" => rules "attrpath" "." "attr";
        "attrpath" => rules "attrpath" "." "string_attr";
        "attrpath" => rules "attr";
        "attrpath" => rules "string_attr";

        "attr" => rules "ID";
        "attr" => rules "OR_KW";

        "string_attr" => rules "\"" "string_parts" "\"";
        "string_attr" => rules "DOLLAR_CURLY" "expr" "}";

        "expr_list" => rules "expr_list" "expr_select";
        "expr_list" => empty;

        "formals" => rules "formal" "," "formals";
        "formals" => rules "formal";
        "formals" => rules "ELLIPSIS";
        "formals" => empty;

        "formal" => rules "ID";
        "formal" => rules "ID" "?" "expr";

        // All lexemes
        "!" => lexemes "!";
        "\"" => lexemes "\"";
        "(" => lexemes "(";
        ")" => lexemes ")";
        "*" => lexemes "*";
        "+" => lexemes "+";
        "," => lexemes ",";
        "." => lexemes ".";
        "/" => lexemes "/";
        ":" => lexemes ":";
        ";" => lexemes ";";
        "<" => lexemes "<";
        "=" => lexemes "=";
        ">" => lexemes ">";
        "?" => lexemes "?";
        "@" => lexemes "@";
        "[" => lexemes "[";
        "]" => lexemes "]";
        "{" => lexemes "{";
        "}" => lexemes "}";
        "AND" => lexemes "AND";
        "ANY" => lexemes "ANY";
        "ASSERT" => lexemes "ASSERT";
        "COMMENT" => lexemes "COMMENT";
        "CONCAT" => lexemes "CONCAT";
        "DOLLAR_CURLY" => lexemes "DOLLAR_CURLY";
        "ELLIPSIS" => lexemes "ELLIPSIS";
        "ELSE" => lexemes "ELSE";
        "EQ" => lexemes "EQ";
        "ERROR" => lexemes "ERROR";
        "FLOAT" => lexemes "FLOAT";
        "GEQ" => lexemes "GEQ";
        "HPATH" => lexemes "HPATH";
        "ID" => lexemes "ID";
        "IF" => lexemes "IF";
        "IMPL" => lexemes "IMPL";
        "IN" => lexemes "IN";
        "IND_STR" => lexemes "IND_STR";
        "IND_STRING_CLOSE" => lexemes "IND_STRING_CLOSE";
        "IND_STRING_OPEN" => lexemes "IND_STRING_OPEN";
        "INHERIT" => lexemes "INHERIT";
        "INT" => lexemes "INT";
        "LEQ" => lexemes "LEQ";
        "LET" => lexemes "LET";
        "NEQ" => lexemes "NEQ";
        "OR" => lexemes "OR";
        "OR_KW" => lexemes "OR_KW";
        "PATH" => lexemes "PATH";
        "PATH_END" => lexemes "PATH_END";
        "REC" => lexemes "REC";
        "SKIP" => lexemes "SKIP";
        "SPATH" => lexemes "SPATH";
        "STR" => lexemes "STR";
        "THEN" => lexemes "THEN";
        "UPDATE" => lexemes "UPDATE";
        "URI" => lexemes "URI";
        "WITH" => lexemes "WITH";
        "WS" => lexemes "WS";
        "NOT" => lexemes "!";
        "NEGATE" => lexemes "-";
        "-" => lexemes "-";

        Associativity::Right => rules "IMPL";
        Associativity::Left => rules "OR";
        Associativity::Left => rules "AND";
        Associativity::None => rules "EQ" "NEQ";
        Associativity::None => rules "<" ">" "LEQ" "GEQ";
        Associativity::Right => rules "UPDATE";
        Associativity::Left => rules "NOT";
        Associativity::Left => rules "+" "-";
        Associativity::Left => rules "*" "/";
        Associativity::Right => rules "CONCAT";
        Associativity::None => rules "?";
        Associativity::None => rules "NEGATE";
    )
}
