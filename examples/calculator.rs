use santiago::{
    parse::parse,
    production::Production,
    rule::Rule,
    symbol::Symbol,
};

fn main() {
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
                Production { terms: vec![Symbol::Char('1')] },
                Production { terms: vec![Symbol::Char('2')] },
                Production { terms: vec![Symbol::Char('3')] },
            ],
        },
        Rule {
            name:        "Plus".to_string(),
            productions: vec![Production { terms: vec![Symbol::Char('+')] }],
        },
    ];

    println!("Grammar:");
    for rule in grammar {
        println!("  {rule}");
    }

    let input: Vec<char> = "1+2+3".chars().collect();

    println!();
    println!("input: {input:?}");

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
