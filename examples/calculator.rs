// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use santiago::grammar::builder::Builder as GrammarBuilder;
use santiago::grammar::rule::Rule;
use santiago::lexer::builder::Builder as LexerBuilder;
use santiago::lexer::lex;
use santiago::lexer::lexeme::Lexeme;
use santiago::lexer::rule::Rule as LexerRule;
use santiago::parser::parse::parse;

fn main() {
    let lexer_rules: Vec<LexerRule> = LexerBuilder::new()
        .string(&["initial"], "+", |matched, _| Some(("plus", matched)))
        .pattern(&["initial"], r"\d+", |matched, _| Some(("int", matched)))
        .string(&["initial"], " ", |_, _| None) // Discard whitespace
        .finish();

    let lexemes: Vec<Lexeme> = lex(&lexer_rules, "1 + 2 + 3");

    println!();
    println!("lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme:?}");
    }

    // This is an example of an ambiguous grammar:
    //   Sum := Sum Plus Sum | Int
    //   Int := "1" | "2" | "3"
    //   Plus := "+"
    let grammar: Vec<Rule> = GrammarBuilder::new()
        .map_to_rules("Sum", &["Sum", "Plus", "Sum"])
        .map_to_lexemes("Sum", &["int"])
        .map_to_lexemes("Plus", &["plus"])
        .finish();

    println!();
    println!("Grammar:");
    for rule in &grammar {
        println!("  {rule}");
    }
    let result = parse(&grammar, &lexemes);

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
