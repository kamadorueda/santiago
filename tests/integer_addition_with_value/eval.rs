pub fn eval(value: &Value) -> isize {
    match value {
        Value::Int(int) => *int,
        Value::BinaryOperation(args) => match &args[1] {
            Value::OperatorAdd => eval(&args[0]) + eval(&args[2]),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
