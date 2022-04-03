// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

mod ambiguous_integer_addition;
mod calculator;
mod calculator_with_value;
mod integer_addition;
mod integer_addition_with_value;
mod javascript_string_interpolation;
mod nix;
mod smallest;

#[test]
fn ambiguous_integer_addition() {
    run(
        "ambiguous_integer_addition",
        &ambiguous_integer_addition::lexer::lexer_rules(),
        &ambiguous_integer_addition::grammar::grammar(),
        false,
    );
}

#[test]
fn calculator() {
    run(
        "calculator",
        &calculator::lexer::lexer_rules(),
        &calculator::grammar::grammar(),
        false,
    );
}

#[test]
fn calculator_with_value() {
    run(
        "calculator_with_value",
        &calculator_with_value::lexer::lexer_rules(),
        &calculator_with_value::grammar::grammar(),
        true,
    );
}

#[test]
fn javascript_string_interpolation() {
    run(
        "javascript_string_interpolation",
        &javascript_string_interpolation::lexer::lexer_rules(),
        &javascript_string_interpolation::grammar::grammar(),
        false,
    );
}

#[test]
fn integer_addition() {
    run(
        "integer_addition",
        &integer_addition::lexer::lexer_rules(),
        &integer_addition::grammar::grammar(),
        false,
    );
}

#[test]
fn integer_addition_with_value() {
    run(
        "integer_addition_with_value",
        &integer_addition_with_value::lexer::lexer_rules(),
        &integer_addition_with_value::grammar::grammar(),
        true,
    );
}

#[test]
fn nix() {
    run("nix", &nix::lexer::lexer_rules(), &nix::grammar::grammar(), false);
}

#[test]
fn smallest() {
    run(
        "smallest",
        &smallest::lexer::lexer_rules(),
        &smallest::grammar::grammar(),
        false,
    );
}

fn run<AST>(
    name: &str,
    lexer_rules: &santiago::lexer::LexerRules,
    grammar: &santiago::grammar::Grammar<AST>,
    test_ast: bool,
) where
    AST: std::fmt::Debug,
{
    use std::io::Write;
    let should_update = std::env::var("UPDATE").is_ok();

    let cases_dir = format!("tests/{name}/cases");

    let cases: Vec<String> = std::fs::read_dir(&cases_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();

    for case in cases {
        println!("{cases_dir}/{case}");
        let path_input = format!("{cases_dir}/{case}/input");
        let path_lexemes = format!("{cases_dir}/{case}/lexemes");
        let path_earley = format!("{cases_dir}/{case}/earley");
        let path_parse_trees = format!("{cases_dir}/{case}/parse_trees");
        let path_asts = format!("{cases_dir}/{case}/values");

        let input = std::fs::read_to_string(&path_input)
            .unwrap()
            .trim_end_matches('\n')
            .to_string();

        let lexemes = santiago::lexer::lex(lexer_rules, &input).unwrap();
        let lexemes_str: String = lexemes
            .iter()
            .map(santiago::lexer::Lexeme::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        #[cfg(not(tarpaulin))]
        if should_update {
            std::fs::File::create(&path_lexemes)
                .unwrap()
                .write_all(lexemes_str.as_bytes())
                .unwrap();
        }

        let earley = santiago::parser::earley(grammar, &lexemes);
        let earley_str: String = earley
            .iter()
            .map(|column| format!("{column}"))
            .collect::<Vec<String>>()
            .join("\n");

        #[cfg(not(tarpaulin))]
        if should_update {
            std::fs::File::create(&path_earley)
                .unwrap()
                .write_all(earley_str.as_bytes())
                .unwrap();
        }

        let parse_trees = santiago::parser::parse(grammar, &lexemes).unwrap();
        let parse_trees_str: String = parse_trees
            .iter()
            .map(|parse_tree| format!("---\n{parse_tree}"))
            .collect::<String>()
            .lines()
            .collect::<Vec<&str>>()
            .join("\n");

        #[cfg(not(tarpaulin))]
        if should_update {
            std::fs::File::create(&path_parse_trees)
                .unwrap()
                .write_all(parse_trees_str.as_bytes())
                .unwrap();
        }
        assert_eq!(
            lexemes_str,
            std::fs::read_to_string(&path_lexemes).unwrap()
        );
        assert_eq!(earley_str, std::fs::read_to_string(&path_earley).unwrap());
        assert_eq!(
            parse_trees_str,
            std::fs::read_to_string(&path_parse_trees).unwrap()
        );

        if test_ast {
            let ast: Vec<AST> = parse_trees
                .iter()
                .map(|parse_tree| parse_tree.as_abstract_syntax_tree())
                .collect();
            let ast_str: String = ast
                .iter()
                .map(|ast| format!("---\n{ast:#?}"))
                .collect::<String>()
                .lines()
                .collect::<Vec<&str>>()
                .join("\n");

            #[cfg(not(tarpaulin))]
            if should_update {
                std::fs::File::create(&path_asts)
                    .unwrap()
                    .write_all(ast_str.as_bytes())
                    .unwrap();
            }

            assert_eq!(ast_str, std::fs::read_to_string(&path_asts).unwrap());
        }
    }
}
