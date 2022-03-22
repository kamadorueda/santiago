use santiago::grammar::GrammarRule;
use santiago::languages::nix::grammar_rules;
use santiago::languages::nix::lexing_rules;
use santiago::lexer::lex;
use santiago::lexer::Lexeme;
use santiago::lexer::LexerRule;
use santiago::parser::parse;
use santiago::parser::Forest;

fn main() -> Result<(), String> {
    let lexing_rules: Vec<LexerRule> = lexing_rules();
    let grammar_rules: Vec<GrammarRule> = grammar_rules(&lexing_rules);

    let stdin = get_stdin();
    let lexemes: Vec<Lexeme> = lex(&lexing_rules, &stdin);

    println!("Lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme}");
    }

    let abstract_syntax_trees: Vec<Forest> = parse(&grammar_rules, &lexemes)?;

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
