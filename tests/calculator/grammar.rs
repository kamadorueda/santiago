use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "expr" => rules "bin_op";
        "expr" => rules "int";

        "bin_op" => rules "expr" "add" "expr";
        "bin_op" => rules "expr" "subtract" "expr";
        "bin_op" => rules "expr" "multiply" "expr";
        "bin_op" => rules "expr" "divide" "expr";

        "int" => lexemes "INT";

        "add" => lexemes "+";
        "subtract" => lexemes "-";
        "multiply" => lexemes "*";
        "divide" => lexemes "/";

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}
