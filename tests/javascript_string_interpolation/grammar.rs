use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
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
        "str" => lexemes "STR";
        "string_start" => lexemes "STRING_START";
        "string_end" => lexemes "STRING_END";
        "var" => lexemes "VAR";
    )
}
