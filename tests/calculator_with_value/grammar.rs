use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

#[derive(Debug)]
pub enum Value {
    Int(isize),
    BinaryOperation(Vec<Value>),
    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
}

pub fn grammar() -> Grammar<Value> {
    santiago::grammar!(
        "expr" => rules "bin_op";
        "expr" => rules "int";

        "bin_op" => rules "expr" "add" "expr" =>
            |values| Value::BinaryOperation(values);
        "bin_op" => rules "expr" "subtract" "expr" =>
            |values| Value::BinaryOperation(values);
        "bin_op" => rules "expr" "multiply" "expr" =>
            |values| Value::BinaryOperation(values);
        "bin_op" => rules "expr" "divide" "expr" =>
            |values| Value::BinaryOperation(values);

        "add" => lexemes "+" =>
            |_| Value::OperatorAdd;
        "subtract" => lexemes "-" =>
            |_| Value::OperatorSubtract;
        "multiply" => lexemes "*" =>
            |_| Value::OperatorMultiply;
        "divide" => lexemes "/" =>
            |_| Value::OperatorDivide;

        "int" => lexemes "INT" =>
            |lexemes| {
                let value = str::parse(&lexemes[0].raw).unwrap();
                Value::Int(value)
            };

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}
