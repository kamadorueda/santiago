use santiago::grammar::grammar_builder::GrammarBuilder;
use santiago::grammar::rule::Rule;
use santiago::lexer::lex;
use santiago::lexer::lexeme::Lexeme;
use santiago::lexer::lexer_builder::LexerBuilder;
use santiago::lexer::lexer_rule::LexerRule;
use santiago::parser::parse::parse;

fn main() -> Result<(), String> {
    // This is an example of an ambiguous grammar that sums numbers:
    //   Sum  := Sum Plus Sum | "int"
    //   Plus := "plus"
    let grammar: Vec<Rule> = GrammarBuilder::new()
        .map_to_rules("Sum", &["Sum", "Plus", "Sum"])
        .map_to_lexemes("Sum", &["int"])
        .map_to_lexemes("Plus", &["plus"])
        .finish();

    // The lexer consumes the input string with the following rules:
    //   "plus" := "+" (a character)
    //   "int"  := \d+ (regular expression for 1 or more digits)
    //     ∅    := " " (ignore whitespace)
    let lexer_rules: Vec<LexerRule> = LexerBuilder::new()
        .string(&["initial"], "+", |matched, _| Some(("plus", matched)))
        .pattern(&["initial"], r"\d+", |matched, _| Some(("int", matched)))
        .string(&["initial"], " ", |_, _| None)
        .finish();

    // First start by tokenizing the input
    let lexemes: Vec<Lexeme> = lex(&lexer_rules, "1 + 2 + 3");

    // Now parse!
    let forests = parse(&grammar, &lexemes)?;

    println!();
    println!("lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme:?}");
    }

    println!();
    println!("Grammar:");
    for rule in &grammar {
        println!("  {rule}");
    }

    println!();
    println!("Forest:");
    for forest in forests {
        println!("{forest}");
    }

    Ok(())
}
