use santiago::grammar::grammar_rule::GrammarRule;
use santiago::languages::nix::grammar_rules;
use santiago::languages::nix::lexing_rules;
use santiago::lexer::lex;
use santiago::lexer::lexeme::Lexeme;
use santiago::lexer::lexer_rule::LexerRule;
use santiago::parser::forest::Forest;
use santiago::parser::parse::parse;

fn main() -> Result<(), String> {
    let lexing_rules: Vec<LexerRule> = lexing_rules();
    let grammar_rules: Vec<GrammarRule> = grammar_rules();

    let stdin = get_stdin();
    let lexemes: Vec<Lexeme> = lex(&lexing_rules, &stdin);
    let abstract_syntax_trees: Vec<Forest> = parse(&grammar_rules, &lexemes)?;

    println!("Lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme}");
    }

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
