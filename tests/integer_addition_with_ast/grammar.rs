use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

#[derive(Debug, PartialEq)]
pub enum AST {
    // A single integer.
    Int(isize),
    // A binary operation with three arguments: `left`, `op`, and `right`.
    BinaryOperation(Vec<AST>),
    // The binary operator for addition (a.k.a. `+`).
    OperatorAdd,
}

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "sum" => rules "sum" "plus" "sum" =>
            AST::BinaryOperation;

        "sum" => lexemes "INT" => |lexemes| {
            // &str to isize conversion
            let value = str::parse::<isize>(&lexemes[0].raw).unwrap();

            AST::Int(value)
        };

        "plus" => lexemes "PLUS" =>
            |_| AST::OperatorAdd;

        Associativity::Left => rules "plus";
    )
}
