use santiago::grammar::Grammar;

pub fn grammar() -> Grammar {
    santiago::grammar!(
        // A rule for 0 characters
        "chars" => empty;
        // A rule that maps to itself plus one character (recursion)
        "chars" => rules "chars" "char";
        // A char comes from the lexeme "CHAR"
        "char" => lexeme "CHAR";
    )
}
