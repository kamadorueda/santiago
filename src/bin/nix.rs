// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

fn main() -> Result<(), String> {
    use std::io::Read;

    let lexing_rules = santiago::languages::nix::lexer_rules();
    let grammar_rules = santiago::languages::nix::grammar();

    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin).unwrap();

    let lexemes = santiago::lexer::lex(&lexing_rules, &stdin).unwrap();

    println!("Lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme}");
    }

    let abstract_syntax_trees =
        santiago::parser::parse(&grammar_rules, &lexemes)?;

    println!("AST:");
    for ast in abstract_syntax_trees {
        println!("{ast}");
    }

    Ok(())
}
