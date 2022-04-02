// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

pub fn eval(value: &Value) -> isize {
    match value {
        Value::Int(int) => *int,
        Value::BinaryOperation(args) => match &args[1] {
            Value::OperatorAdd => eval(&args[0]) + eval(&args[2]),
            Value::OperatorSubtract => eval(&args[0]) - eval(&args[2]),
            Value::OperatorMultiply => eval(&args[0]) * eval(&args[2]),
            Value::OperatorDivide => eval(&args[0]) / eval(&args[2]),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
