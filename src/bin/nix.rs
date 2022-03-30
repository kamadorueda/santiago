// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

fn main() {
    use std::io::Read;

    let lexing_rules = santiago::languages::nix::lexer_rules();
    let grammar = santiago::languages::nix::grammar();

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
                }
                Err(error) => {
                    println!("Parsing Error:");
                    println!("{error}");
                }
            }
        }
        Err(error) => {
            println!("Parsing Error:");
            println!("{error}");
        }
    }
}
