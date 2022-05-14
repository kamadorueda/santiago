use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "expr" => rules "int";

        "expr" => rules "expr" "add" "expr";
        "expr" => rules "expr" "subtract" "expr";
        "expr" => rules "expr" "multiply" "expr";
        "expr" => rules "expr" "divide" "expr";

        "int" => lexemes "INT";

        "add" => lexemes "+";
        "subtract" => lexemes "-";
        "multiply" => lexemes "*";
        "divide" => lexemes "/";

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}
