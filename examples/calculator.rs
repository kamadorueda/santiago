// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

include!("../tests/calculator_with_value/lexer.rs");
include!("../tests/calculator_with_value/grammar.rs");
include!("../tests/calculator_with_value/eval.rs");

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
                Ok(parse_trees) => {
                    println!("Parse Trees:");
                    let parse_tree = &parse_trees[0];
                    println!("{parse_tree}");

                    let ast = parse_tree.as_abstract_syntax_tree();

                    println!("Abstract Syntax Tree:");
                    println!("{ast:?}");

                    println!("Evaluated:");
                    println!("{}", eval(&ast));

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
