use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

#[derive(Debug)]
pub enum AST {
    Int(isize),
    BinaryOperation(Vec<AST>),
    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
}

pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "expr" => rules "bin_op";
        "expr" => rules "int";

        "bin_op" => rules "expr" "add" "expr" =>
            |values| AST::BinaryOperation(values);
        "bin_op" => rules "expr" "subtract" "expr" =>
            |values| AST::BinaryOperation(values);
        "bin_op" => rules "expr" "multiply" "expr" =>
            |values| AST::BinaryOperation(values);
        "bin_op" => rules "expr" "divide" "expr" =>
            |values| AST::BinaryOperation(values);

        "add" => lexemes "+" =>
            |_| AST::OperatorAdd;
        "subtract" => lexemes "-" =>
            |_| AST::OperatorSubtract;
        "multiply" => lexemes "*" =>
            |_| AST::OperatorMultiply;
        "divide" => lexemes "/" =>
            |_| AST::OperatorDivide;

        "int" => lexemes "INT" =>
            |lexemes| {
                let value = str::parse(&lexemes[0].raw).unwrap();
                AST::Int(value)
            };

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}
