use santiago::grammar::Grammar;

pub fn grammar() -> Grammar {
    santiago::grammar!(
        // A string in the form: `str_content`
        "string" => rules "string_start" "str_content" "string_end";

        // Either empty
        // or followed by a "str"
        // or followed by a "var"
        "str_content" => empty;
        "str_content" => rules "str_content" "str";
        "str_content" => rules "str_content" "var";

        // Map rules to their corresponding Lexemes
        "str" => lexeme "STR";
        "string_start" => lexeme "STRING_START";
        "string_end" => lexeme "STRING_END";
        "var" => lexeme "VAR";
    )
}
