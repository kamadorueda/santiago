use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "sum" => rules "sum" "plus" "sum";
        "sum" => lexemes "INT";
        "plus" => lexemes "PLUS";

        Associativity::Left => rules "plus";
    )
}
