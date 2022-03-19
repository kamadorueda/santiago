// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

pub mod builder;
pub mod lexeme;
pub mod position;
pub mod rule;

use self::lexeme::Lexeme;
use self::position::Position;
use self::rule::Rule;
use std::collections::LinkedList;

pub struct Lexer {
    input_index:  usize,
    states_stack: LinkedList<String>,
}

impl Lexer {
    fn next_lexeme(
        &mut self,
        rules: &[Rule],
        input: &str,
    ) -> Option<(String, String)> {
        while self.input_index < input.len() {
            let mut matches_: LinkedList<(usize, usize)> = LinkedList::new();
            let input = &input[self.input_index..];
            let state = self.states_stack.back().unwrap();

            for (rule_index, rule) in rules.iter().enumerate() {
                if rule.states.contains(state) {
                    let matcher = &rule.matcher;

                    match matcher(input) {
                        Some(len) => {
                            matches_.push_back((len, rule_index));
                        }
                        None => {}
                    }
                }
            }

            if matches_.len() == 0 {
                panic!("Unable to lex input with the provided rules: {input}");
            }

            let (len, rule_index) = matches_
                .into_iter()
                .max_by(|(left, _), (right, _)| left.cmp(right))
                .unwrap();

            match rules[rule_index].action.clone()(&input[..len], self) {
                Some((kind, raw)) => {
                    self.input_index += len;
                    let kind = kind.to_string();
                    let raw = raw.to_string();
                    return Some((kind, raw));
                }
                None => {
                    self.input_index += len;
                    continue;
                }
            }
        }

        match &input[self.input_index..].chars().next() {
            Some(raw) => {
                let raw = raw.to_string();
                let kind = raw.clone();
                self.input_index += 1;

                Some((kind, raw))
            }
            None => None,
        }
    }
}

pub fn lex(rules: &[Rule], input: &str) -> Vec<Lexeme> {
    let mut lexer = Lexer { input_index: 0, states_stack: LinkedList::new() };

    lexer.states_stack.push_back("initial".to_string());

    let mut column: usize = 1;
    let mut line: usize = 1;
    let mut lexemes = LinkedList::new();

    loop {
        let position = Position { column, index: lexer.input_index, line };

        match lexer.next_lexeme(rules, input) {
            Some((kind, raw)) => {
                for char in input[position.index..lexer.input_index].chars() {
                    if char == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }

                lexemes.push_back(Lexeme { kind, position, raw })
            }
            _ => {
                break;
            }
        }
    }

    lexemes.into_iter().collect()
}
