// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod lexeme;
pub mod lexer_builder;
pub mod lexer_rule;
pub mod position;

use self::lexeme::Lexeme;
use self::lexer_rule::LexerRule;
use self::position::Position;
use std::collections::LinkedList;

pub struct Lexer<'a> {
    input:             &'a str,
    current_match_len: usize,
    position:          Position,
    states_stack:      LinkedList<&'a str>,
}

pub enum NextLexeme {
    Consumed((String, String)),
    Ignored,
    Finished,
}

impl<'a> Lexer<'a> {
    fn next_lexeme(&mut self, rules: &[LexerRule]) -> NextLexeme {
        while self.position.index < self.input.len() {
            let mut matches_: LinkedList<(usize, usize)> = LinkedList::new();
            let input = &self.input[self.position.index..];
            let state = self.states_stack.back().unwrap();

            for (rule_index, rule) in rules.iter().enumerate() {
                if rule.states.contains(*state) {
                    let matcher = &rule.matcher;

                    match matcher(input) {
                        Some(len) => {
                            matches_.push_back((len, rule_index));
                        }
                        None => {}
                    }
                }
            }

            if matches_.is_empty() {
                panic!("Unable to lex input with the provided rules: {input}");
            }

            let (len, rule_index) = matches_
                .into_iter()
                .max_by(|(left, _), (right, _)| left.cmp(right))
                .unwrap();

            self.current_match_len = len;

            return rules[rule_index].action.clone()(self);
        }

        NextLexeme::Finished
    }

    pub fn matched(&self) -> &str {
        &self.input
            [self.position.index..self.position.index + self.current_match_len]
    }

    pub fn consume_as(&mut self, kind: &str) -> NextLexeme {
        let kind = kind.to_string();
        let matched = self.matched().to_string();

        self.position.consume(&matched);

        NextLexeme::Consumed((kind, matched))
    }

    pub fn ignore(&mut self) -> NextLexeme {
        let matched = self.matched().to_string();

        self.position.consume(&matched);

        NextLexeme::Ignored
    }
}

pub fn lex(rules: &[LexerRule], input: &str) -> Vec<Lexeme> {
    let mut lexer = Lexer {
        input,
        current_match_len: 0,
        position:          Position::new(),
        states_stack:      LinkedList::new(),
    };

    lexer.states_stack.push_back("initial");

    let mut lexemes = LinkedList::new();

    loop {
        let position = lexer.position.clone();

        match lexer.next_lexeme(rules) {
            NextLexeme::Consumed((kind, raw)) => {
                lexemes.push_back(Lexeme { kind, position, raw })
            }
            NextLexeme::Ignored => {}
            NextLexeme::Finished => {
                break;
            }
        }
    }

    lexemes.into_iter().collect()
}
