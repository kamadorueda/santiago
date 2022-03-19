// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use santiago::{
    grammar::{builder::Builder, rule::Rule},
    lexer::{lexeme::Lexeme, Lexer},
    parser::parse::parse,
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
    let grammar: Vec<Rule> = Builder::new()
        .map_to_rules("Sum", &["Sum", "Plus", "Sum"])
        .map_to_rules("Sum", &["Int"])
        .map_to_lexemes("Int", &["1"])
        .map_to_lexemes("Int", &["2"])
        .map_to_lexemes("Int", &["3"])
        .map_to_lexemes("Plus", &["+"])
        .finish();

    println!();
    println!("Grammar:");
    for rule in &grammar {
        println!("  {rule}");
    }
    let result = parse(&grammar, &input);

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
