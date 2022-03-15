fn main() {
    santiago::test();
    // let mut builder = GrammarBuilder::default();
    // builder.add_rule("P", vec![Symbol::NonTerminal("S".to_string())]);
    // builder.add_rule(
    //     "S",
    //     vec![
    //         Symbol::NonTerminal("S".to_string()),
    //         Symbol::Terminal(LexemeKind::Plus),
    //         Symbol::NonTerminal("S".to_string()),
    //     ],
    // );
    // builder.add_rule("S", vec![Symbol::NonTerminal("T".to_string())]);
    // // builder.add_rule(
    // //     "M",
    // //     vec![
    // //         Symbol::NonTerminal("M".to_string()),
    // //         Symbol::Terminal(LexemeKind::Asterisk),
    // //         Symbol::NonTerminal("T".to_string()),
    // //     ],
    // // );
    // // builder.add_rule("M", vec![Symbol::NonTerminal("T".to_string())]);
    // builder.add_rule("T", vec![Symbol::Terminal(LexemeKind::Int)]);

    // let grammar = builder.build();

    // println!("Grammar:");
    // for rule in &grammar {
    //     println!("  {}", rule);
    // }
    // println!();
    // println!("Parsed:");
    // santiago::parser::parse(
    //     &grammar,
    //     &[
    //         Lexeme {
    //             kind:     LexemeKind::Int,
    //             raw:      "2".to_string(),
    //             position: Position { column: 1, index: 0, line: 1 },
    //         },
    //         Lexeme {
    //             kind:     LexemeKind::Plus,
    //             raw:      "+".to_string(),
    //             position: Position { column: 2, index: 1, line: 1 },
    //         },
    //         Lexeme {
    //             kind:     LexemeKind::Int,
    //             raw:      "3".to_string(),
    //             position: Position { column: 3, index: 2, line: 1 },
    //         },
    //         Lexeme {
    //             kind:     LexemeKind::Plus,
    //             raw:      "+".to_string(),
    //             position: Position { column: 4, index: 3, line: 1 },
    //         },
    //         Lexeme {
    //             kind:     LexemeKind::Int,
    //             raw:      "4".to_string(),
    //             position: Position { column: 5, index: 4, line: 1 },
    //         },
    //     ],
    // )
    // .unwrap();
}
