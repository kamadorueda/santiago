use santiago::lexer::lex;
use santiago::lexer::lexeme::Lexeme;
use santiago::lexer::lexer_rule::LexerRule;
use santiago::nix::lexing_rules;
use std::io::Read;

fn main() -> Result<(), String> {
    eprintln!("Reading stdin");

    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin).unwrap();
    let lexing_rules: Vec<LexerRule> = lexing_rules();
    let lexemes: Vec<Lexeme> = lex(&lexing_rules, &stdin);

    println!("Input: {stdin}");
    println!("Lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme}");
    }
    Ok(())
}
