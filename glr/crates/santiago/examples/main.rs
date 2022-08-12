#[rustfmt::skip]
/// PRODUCTIONS =
///   0 | "Γ" -> "sum"
///   1 | "sum" -> "sum" "+" "sum"
///   2 | "sum" -> "INT"
//
/// PARSER_INSTRUCTIONS =
///   0 "INT" -> Shift(1)
///   0 "sum" -> Shift(2)
///   1 "+" -> Reduce(2)
///   1 "INT" -> Reduce(2)
///   1 "Ω" -> Reduce(2)
///   2 "+" -> Shift(3)
///   2 "Ω" -> Accept
///   3 "INT" -> Shift(1)
///   3 "sum" -> Shift(2) Shift(4)
///   4 "+" -> Reduce(1)
///   4 "INT" -> Reduce(1)
///   4 "Ω" -> Reduce(1)
//
///  0 | 0 INT
///  1 | 1 +
///  2 | 0 SUM
///  3 | 2 +
///  4 | 3 INT
///  5 | 1 +
//
///                      |                                                | 0           | INT0 +1 INT1 +2 INT2
///  0 | Shift(1)        | INT0                                           | 0 1         | +1 INT1 +2 INT2
///  1 | Reduce(2)       |                                                | 0           | SUM(INT0) +1 INT1 +2 INT2
///  2 | Shift(2)        | SUM(INT0)                                      | 0 2         | +1 INT1 +2 INT2
///  3 | Shift(3)        | SUM(INT0) +1                                   | 0 2 3       | INT1 +2 INT2
///  4 | Shift(1)        | SUM(INT0) +1 INT1                              | 0 2 3 1     | +2 INT2
///  5 | Reduce(2)       | SUM(INT0) +1                                   | 0 2 3       | SUM(INT1) +2 INT2
///  6 | \ 1Shift(2)      | SUM(INT0) +1 SUM(INT1)                         | 0 2 3 2     | +2 INT2
///  7 |   1Shift(3)      | SUM(INT0) +1 SUM(INT1) +2                      | 0 2 3 2 3   | INT2
///  8 |   1Shift(1)      | SUM(INT0) +1 SUM(INT1) +2 INT2                 | 0 2 3 2 3 1 |
///  9 |   1Reduce(2)     | SUM(INT0) +1 SUM(INT1) +2                      | 0 2 3 2 3   | SUM(INT2)
/// 10 |   1\ 1Shift(2)    | SUM(INT0) +1 SUM(INT1) +2 SUM(INT2)            | 0 2 3 2 3 2 |
/// 11 |   1  1Reject      | SUM(INT0) +1 SUM(INT1) +2 SUM(INT2)            | 0 2 3 2 3   |
/// 12 |   1\ 1Shift(4)    | SUM(INT0) +1 SUM(INT1) +2 SUM(INT2)            | 0 2 3 2 3 4 |
/// 13 |   1  1Reduce(1)   | SUM(INT0) +1                                   | 0 2 3       | SUM(SUM(INT1) +2 SUM(INT2))
/// 14 |   1  1\ 1Shift(2)  | SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2))       | 0 2 3 2     |
/// 15 |   1  1  1Reject    | SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2))       | 0 2 3       |
/// 16 |   1  1\ 1Shift(4)  | SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2))       | 0 2 3 4     |
/// 17 |   1  1  1Reduce(1) |                                                | 0           | SUM(SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2)))
/// 18 |   1  1  1Shift(2)  | SUM(SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2)))  | 0 2         |
/// 19 |   1  1  1Accept    | SUM(SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2)))  | 0           |
/// 20 | \ 2Shift(4)      | SUM(INT0) +1 SUM(INT1)                         | 0 2 3 4     | +2 INT2
/// 21 |   2Reduce(1)     |                                                | 0           | SUM(SUM(INT0) +1 SUM(INT1)) +2 INT2
/// 22 |   2Shift(2)      | SUM(SUM(INT0) +1 SUM(INT1))                    | 0 2         | +2 INT2
/// 23 |   2Shift(3)      | SUM(SUM(INT0) +1 SUM(INT1)) +2                 | 0 2 3       | INT2
/// 24 |   2Shift(1)      | SUM(SUM(INT0) +1 SUM(INT1)) +2 INT2            | 0 2 3 1     |
/// 25 |   2Reduce(2)     | SUM(SUM(INT0) +1 SUM(INT1)) +2                 | 0 2 3       | SUM(INT2)
/// 26 |   2\ 1Shift(2)    | SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2)       | 0 2 3 2     |
/// 27 |   2  1Reject      | SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2)       | 0 2 3       |
/// 28 |   2\ 1Shift(4)    | SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2)       | 0 2 3 4     |
/// 29 |   2  1Reduce(1)   |                                                | 0           | SUM(SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2))
/// 30 |   2  1Shift(2)    | SUM(SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2))  | 0 2         |
/// 31 |   2  1Accept      | SUM(SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2))  | 0           |
//
//
//
///                         |                                                | 0           | INT0 +1 INT1 +2 INT2
///  0 | Shift(1)           | INT0                                           | 0 1         | +1 INT1 +2 INT2
///  1 | Reduce(2)          |                                                | 0           | SUM(INT0) +1 INT1 +2 INT2
///  2 | Shift(2)           | SUM(INT0)                                      | 0 2         | +1 INT1 +2 INT2
///  3 | Shift(3)           | SUM(INT0) +1                                   | 0 2 3       | INT1 +2 INT2
///  4 | Shift(1)           | SUM(INT0) +1 INT1                              | 0 2 3 1     | +2 INT2
///  5 | Reduce(2)          | SUM(INT0) +1                                   | 0 2 3       | SUM(INT1) +2 INT2
///  6 | \ 1Shift(2)        | SUM(INT0) +1 SUM(INT1)                         | 0 2 3 2     | +2 INT2
/// 20 | \ 2Shift(4)        | SUM(INT0) +1 SUM(INT1)                         | 0 2 3 4     | +2 INT2
/// 21 |   2Reduce(1)       |                                                | 0           | SUM(SUM(INT0) +1 SUM(INT1)) +2 INT2
/// 22 |   2Shift(2)        | SUM(SUM(INT0) +1 SUM(INT1))                    | 0 2         | +2 INT2
/// 23 |   2Shift(3)        | SUM(SUM(INT0) +1 SUM(INT1)) +2                 | 0 2 3       | INT2
/// 24 |   2Shift(1)        | SUM(SUM(INT0) +1 SUM(INT1)) +2 INT2            | 0 2 3 1     |
/// 25 |   2Reduce(2)       | SUM(SUM(INT0) +1 SUM(INT1)) +2                 | 0 2 3       | SUM(INT2)
/// 26 |   2\ 1Shift(2)     | SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2)       | 0 2 3 2     |
/// 27 |   2  1Reject       | SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2)       | 0 2 3       |
/// 28 |   2\ 1Shift(4)     | SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2)       | 0 2 3 4     |
/// 29 |   2  1Reduce(1)    |                                                | 0           | SUM(SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2))
/// 30 |   2  1Shift(2)     | SUM(SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2))  | 0 2         |
/// 31 |   2  1Accept       | SUM(SUM(SUM(INT0) +1 SUM(INT1)) +2 SUM(INT2))  | 0           |
///  7 |   1Shift(3)        | SUM(INT0) +1 SUM(INT1) +2                      | 0 2 3 2 3   | INT2
///  8 |   1Shift(1)        | SUM(INT0) +1 SUM(INT1) +2 INT2                 | 0 2 3 2 3 1 |
///  9 |   1Reduce(2)       | SUM(INT0) +1 SUM(INT1) +2                      | 0 2 3 2 3   | SUM(INT2)
/// 10 |   1\ 1Shift(2)     | SUM(INT0) +1 SUM(INT1) +2 SUM(INT2)            | 0 2 3 2 3 2 |
/// 12 |   1\ 1Shift(4)     | SUM(INT0) +1 SUM(INT1) +2 SUM(INT2)            | 0 2 3 2 3 4 |

/// 11 |   1  1Reject       | SUM(INT0) +1 SUM(INT1) +2 SUM(INT2)            | 0 2 3 2 3   |
/// 13 |   1  1Reduce(1)    | SUM(INT0) +1                                   | 0 2 3       | SUM(SUM(INT1) +2 SUM(INT2))
/// 14 |   1  1\ 1Shift(2)  | SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2))       | 0 2 3 2     |
/// 15 |   1  1  1Reject    | SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2))       | 0 2 3       |
/// 16 |   1  1\ 1Shift(4)  | SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2))       | 0 2 3 4     |
/// 17 |   1  1  1Reduce(1) |                                                | 0           | SUM(SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2)))
/// 18 |   1  1  1Shift(2)  | SUM(SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2)))  | 0 2         |
/// 19 |   1  1  1Accept    | SUM(SUM(INT0) +1 SUM(SUM(INT1) +2 SUM(INT2)))  | 0           |
mod unambiguous_calculator {
    santiago::macros::build!([
        [
            // Grammar
            "sum" = ["sum", "+", "INT"],
            "sum" = ["INT"],
        ],
        [
            // Lexer
            [["DEFAULT"], "INT", regex(r"[0-9]+")],
            [["DEFAULT"], "+", literal(r"+")],
        ]
    ]);
}

mod ambiguous_calculator {
    santiago::macros::build!([
        [
            // Grammar
            "sum" = ["sum", "+", "sum"],
            "sum" = ["INT"],
        ],
        [
            // Lexer
            [["DEFAULT"], "INT", regex(r"[0-9]+")],
            [["DEFAULT"], "+", literal(r"+")],
        ]
    ]);
}

mod unary_string {
    santiago::macros::build!([
        [
            // Grammar
            "A" = ["a", "A"],
            "A" = ["A", "a"],
            "A" = ["a"],
        ],
        [
            // Lexer
            [["DEFAULT"], "a", literal(r"a")],
        ]
    ]);
}

fn main() {
    // use unary_string::*;
    use ambiguous_calculator::*;
    let inputs = [
        "10+20", /* "10+",    //
                 * "10+20+", // */
    ];

    println!("SYMBOLS =");
    println!("{}", debug(&SYMBOLS));
    println!("PRODUCTIONS =");
    println!("{}", debug(&PRODUCTIONS));
    println!("PARSER_INSTRUCTIONS =");
    println!("{}", debug(&PARSER_INSTRUCTIONS));

    for input in inputs {
        println!("INPUT={input:?}");
        let lexemes = lex(input);
        println!("LEXEMES=");
        println!("{}", debug(&lexemes));

        let forest = parse(&lexemes);
    }
}
