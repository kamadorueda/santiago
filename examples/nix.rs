use santiago::languages::nix::grammar_rules;
use santiago::languages::nix::lexer_rules;
use santiago::lexer::lex;
use santiago::lexer::Lexeme;
use santiago::parser::parse;
use santiago::parser::Tree;

fn main() -> Result<(), String> {
    let lexing_rules = lexer_rules();
    let grammar_rules = grammar_rules(&lexing_rules);

    let stdin = get_stdin();
    let lexemes: Vec<Lexeme> = lex(&lexing_rules, &stdin);

    println!("Lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme}");
    }

    let abstract_syntax_trees: Vec<Tree> = parse(&grammar_rules, &lexemes)?;

    println!("AST:");
    for ast in abstract_syntax_trees {
        println!("{ast}");
    }

    Ok(())
}

fn get_stdin() -> String {
    use std::io::Read;

    eprintln!("Reading stdin");
    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin).unwrap();
    stdin
}
