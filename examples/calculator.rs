use santiago::{
    lex::{lexeme::Lexeme, lexer::Lexer},
    parse::parse::parse,
    production::Production,
    rule::Rule,
    symbol::Symbol,
};

fn main() {
    let input: Vec<Lexeme> = Lexer::lex("1+2+3");

    println!();
    println!("input:");
    for lexeme in &input {
        println!("  {lexeme:?}");
    }

    // This is an example of an ambiguous grammar:
    //   Sum := Sum Plus Sum | Int
    //   Int := "1" | "2" | "3"
    //   Plus := "+"
    let grammar = &[
        Rule {
            name:        "Sum".to_string(),
            productions: vec![
                Production {
                    terms: vec![
                        Symbol::Rule("Sum".to_string()),
                        Symbol::Rule("Plus".to_string()),
                        Symbol::Rule("Sum".to_string()),
                    ],
                },
                Production { terms: vec![Symbol::Rule("Int".to_string())] },
            ],
        },
        Rule {
            name:        "Int".to_string(),
            productions: vec![
                Production { terms: vec![Symbol::Lexeme("1".to_string())] },
                Production { terms: vec![Symbol::Lexeme("2".to_string())] },
                Production { terms: vec![Symbol::Lexeme("3".to_string())] },
            ],
        },
        Rule {
            name:        "Plus".to_string(),
            productions: vec![Production {
                terms: vec![Symbol::Lexeme("+".to_string())],
            }],
        },
    ];

    println!();
    println!("Grammar:");
    for rule in grammar {
        println!("  {rule}");
    }
    let result = parse(grammar, &input);

    println!();
    println!("Forest:");
    match result {
        Ok(forests) => {
            for forest in forests {
                println!("{forest}");
            }
        }
        Err(_) => println!("{result:?}"),
    }
}
