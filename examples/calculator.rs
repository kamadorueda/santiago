use santiago::{
    lexeme::Lexeme,
    lexer::Lexer,
    parse::parse,
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

    // S := S + S | T
    // T := 1 | 2 | 3
    let grammar = &[
        Rule {
            name:        "S".to_string(),
            productions: vec![
                Production {
                    terms: vec![
                        Symbol::Rule("S".to_string()),
                        Symbol::Rule("Plus".to_string()),
                        Symbol::Rule("S".to_string()),
                    ],
                },
                Production { terms: vec![Symbol::Rule("T".to_string())] },
            ],
        },
        Rule {
            name:        "T".to_string(),
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

    println!();
    println!("Forest:");
    let result = parse(grammar, &input);
    match result {
        Ok(forests) => {
            for forest in forests {
                println!("{forest}");
            }
        }
        Err(_) => println!("{result:?}"),
    }
}
