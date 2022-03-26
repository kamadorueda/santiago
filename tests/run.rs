pub mod ambiguous_integer_addition;
pub mod integer_addition;

#[test]
fn ambiguous_integer_addition() {
    run(
        "ambiguous_integer_addition",
        &ambiguous_integer_addition::lexer::lexer(),
        &ambiguous_integer_addition::grammar::grammar(),
    );
}

#[test]
fn integer_addition() {
    run(
        "integer_addition",
        &integer_addition::lexer::lexer(),
        &integer_addition::grammar::grammar(),
    );
}

fn run(
    name: &str,
    lexer_rules: &[santiago::lexer::LexerRule],
    grammar: &santiago::grammar::Grammar,
) {
    use std::io::Write;
    let should_update = std::env::var("UPDATE").is_ok();

    let cases_dir = format!("tests/{name}/cases");

    let cases: Vec<String> = std::fs::read_dir(&cases_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    for case in cases {
        let path_input = format!("{cases_dir}/{case}/input");
        let path_lexemes = format!("{cases_dir}/{case}/lexemes");
        let path_forest = format!("{cases_dir}/{case}/forest");

        dbg!(&path_input);
        let input = std::fs::read_to_string(&path_input).unwrap();

        let lexemes = santiago::lexer::lex(lexer_rules, &input);
        let lexemes_str: String = lexemes
            .iter()
            .map(santiago::lexer::Lexeme::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        let forest = santiago::parser::parse(grammar, &lexemes).unwrap();
        let forest_str: String = forest
            .iter()
            .map(|ast| format!("---\n{ast}"))
            .collect::<String>()
            .lines()
            .collect::<Vec<&str>>()
            .join("\n");

        if should_update {
            std::fs::File::create(&path_lexemes)
                .unwrap()
                .write_all(lexemes_str.as_bytes())
                .unwrap();
            std::fs::File::create(&path_forest)
                .unwrap()
                .write_all(forest_str.as_bytes())
                .unwrap();
        }

        assert_eq!(
            lexemes_str,
            std::fs::read_to_string(&path_lexemes).unwrap()
        );
        assert_eq!(forest_str, std::fs::read_to_string(&path_forest).unwrap());
    }
}
