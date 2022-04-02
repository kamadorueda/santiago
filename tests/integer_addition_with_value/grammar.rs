use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

#[derive(Debug, PartialEq)]
pub enum Value {
    // A single integer.
    Int(isize),
    // A binary operation with three arguments: `left`, `op`, and `right`.
    BinaryOperation(Vec<Value>),
    // The binary operator for addition (a.k.a. `+`).
    OperatorAdd,
}

pub fn grammar() -> Grammar<Value> {
    santiago::grammar!(
        "sum" => rules "sum" "plus" "sum" =>
            |values| Value::BinaryOperation(values);

        "sum" => lexemes "INT" => |lexemes| {
            // &str to isize conversion
            let value = str::parse::<isize>(&lexemes[0].raw).unwrap();

            Value::Int(value)
        };

        "plus" => lexemes "PLUS" =>
            |_| Value::OperatorAdd;

        Associativity::Left => rules "plus";
    )
}
