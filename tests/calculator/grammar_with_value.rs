use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

/// Build a grammar for this language.
pub fn grammar<'a>() -> Grammar<Value<'a>> {
    santiago::grammar!(
        "expr" => rules "bin_op" => |mut values| {
            values.swap_remove(0)
        };
        "expr" => rules "int" => |mut values| {
            values.swap_remove(0)
        };

        "bin_op" => rules "expr" "add" "expr" => |values| {
            Value::Operation {
                kind: "add",
                args: values,
            }
        };
        "bin_op" => rules "expr" "subtract" "expr" => |values| {
            Value::Operation {
                kind: "subtract",
                args: values,
            }
        };
        "bin_op" => rules "expr" "multiply" "expr" => |values| {
            Value::Operation {
                kind: "multiply",
                args: values,
            }
        };
        "bin_op" => rules "expr" "divide" "expr" => |values| {
            Value::Operation {
                kind: "divide",
                args: values,
            }
        };

        "int" => lexemes "INT" => |lexemes| {
            let value = str::parse(&lexemes[0].raw).unwrap();
            Value::Int(value)
        };

        "add" => lexemes "+" => |_| Value::Operator("+");
        "subtract" => lexemes "-" => |_| Value::Operator("-");
        "multiply" => lexemes "*" => |_| Value::Operator("*");
        "divide" => lexemes "/" => |_| Value::Operator("/");

        Associativity::Left => rules "add" "subtract";
        Associativity::Left => rules "multiply" "divide";
    )
}

pub enum Value<'a> {
    Int(isize),
    Operation { kind: &'a str, args: Vec<Value<'a>> },
    Operator(&'a str),
}

impl<'a> Value<'a> {
    pub fn eval(&self) -> isize {
        match self {
            Value::Int(int) => *int,
            Value::Operation { kind, args } => match *kind {
                "add" => args[0].eval() + args[2].eval(),
                "subtract" => args[0].eval() - args[2].eval(),
                "multiply" => args[0].eval() * args[2].eval(),
                "divide" => args[0].eval() / args[2].eval(),
                kind => unreachable!("{}", kind),
            },
            Value::Operator(_) => unreachable!(),
        }
    }
}
