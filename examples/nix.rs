include!("../tests/nix/lexer.rs");
include!("../tests/nix/grammar.rs");

fn main() -> Result<(), ()> {
    use std::io::Read;

    let lexing_rules = lexer_rules();
    let grammar = grammar();

    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin).unwrap();

    match santiago::lexer::lex(&lexing_rules, &stdin) {
        Ok(lexemes) => {
            println!("Lexemes:");
            for lexeme in &lexemes {
                println!("  {lexeme}");
            }

            match santiago::parser::parse(&grammar, &lexemes) {
                Ok(abstract_syntax_trees) => {
                    println!("Abstract Syntax Trees:");
                    for ast in abstract_syntax_trees {
                        println!("{ast}");
                    }
                    Ok(())
                }
                Err(error) => {
                    println!("Parsing Error:");
                    println!("{error}");
                    Err(())
                }
            }
        }
        Err(error) => {
            println!("Parsing Error:");
            println!("{error}");
            Err(())
        }
    }
}
