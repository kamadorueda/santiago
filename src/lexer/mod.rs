// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Convert an input of characters into strings with an assigned meaning.

mod lexeme;
mod lexer_builder;
mod lexer_rule;
mod position;

pub use lexeme::Lexeme;
pub use lexer_builder::LexerBuilder;
pub use lexer_rule::LexerRule;
pub use position::Position;
use std::collections::LinkedList;

pub struct Lexer<'a> {
    input:             &'a str,
    current_match_len: usize,
    current_rule_name: &'a str,
    position:          Position,
    states_stack:      LinkedList<&'a str>,
}

pub enum NextLexeme {
    Lexeme((String, String)),
    Skip,
    Finished,
}

impl<'a> Lexer<'a> {
    fn next_lexeme(&mut self, rules: &'a [LexerRule]) -> NextLexeme {
        if self.position.index < self.input.len() {
            let mut matches_: LinkedList<(usize, usize)> = LinkedList::new();
            let input = &self.input[self.position.index..];
            let state = self.states_stack.back().unwrap();

            for (rule_index, rule) in rules.iter().enumerate() {
                if rule.states.contains(*state) {
                    let matcher = &rule.matcher;

                    if let Some(len) = matcher(input) {
                        matches_.push_back((len, rule_index));
                    }
                }
            }

            if matches_.is_empty() {
                panic!("Unable to lex input with the provided rules: {input}");
            }

            // Pick matches with the same maximum length
            // And take the first rule declared
            let max_len = matches_
                .iter()
                .max_by(|left, right| left.0.cmp(&right.0))
                .unwrap()
                .0;
            let (len, rule_index): (usize, usize) = matches_
                .into_iter()
                .filter(|match_| match_.0 == max_len)
                .min_by(|left, right| left.1.cmp(&right.1))
                .unwrap();

            self.current_match_len = len;
            self.current_rule_name = &rules[rule_index].name;

            rules[rule_index].action.clone()(self)
        } else {
            NextLexeme::Finished
        }
    }

    pub fn matched(&self) -> &str {
        &self.input
            [self.position.index..self.position.index + self.current_match_len]
    }

    pub fn take(&mut self) -> NextLexeme {
        self.take_and_map(|matched| matched.to_string())
    }

    pub fn take_and_map(&mut self, function: fn(&str) -> String) -> NextLexeme {
        let matched = self.matched().to_string();
        self.position.consume(&matched);

        let kind = self.current_rule_name.to_string();
        let raw = function(&matched);

        NextLexeme::Lexeme((kind, raw))
    }

    pub fn skip(&mut self) -> NextLexeme {
        let matched = self.matched().to_string();
        self.position.consume(&matched);

        NextLexeme::Skip
    }

    pub fn take_and_retry(&mut self) -> NextLexeme {
        self.take_and_map_and_retry(|matched| matched.to_string())
    }

    pub fn take_and_map_and_retry(
        &mut self,
        function: fn(&str) -> String,
    ) -> NextLexeme {
        let matched = self.matched().to_string();

        let kind = self.current_rule_name.to_string();
        let raw = function(&matched);

        NextLexeme::Lexeme((kind, raw))
    }

    pub fn skip_and_retry(&mut self) -> NextLexeme {
        NextLexeme::Skip
    }

    pub fn current_state(&mut self) -> &str {
        self.states_stack.back().unwrap()
    }

    pub fn pop_state(&mut self) {
        self.states_stack.pop_back();
    }

    pub fn push_state(&mut self, state: &'a str) {
        self.states_stack.push_back(state);
    }

    pub fn error(&mut self, msg: &str) -> NextLexeme {
        panic!(
            "\n\n{}\nWhile lexing input: {:?}\nAt rule: {}\nAt position: \
             {}\nWith states stack: {:?}\n\n",
            msg,
            self.matched().chars().collect::<Vec<char>>(),
            self.current_rule_name,
            self.position,
            self.states_stack,
        );
    }
}

pub fn lex(rules: &[LexerRule], input: &str) -> Vec<Lexeme> {
    let mut lexer = Lexer {
        input,
        current_match_len: 0,
        current_rule_name: "",
        position: Position::new(),
        states_stack: LinkedList::new(),
    };

    lexer.push_state("INITIAL");

    let mut lexemes = LinkedList::new();

    loop {
        let position = lexer.position.clone();

        match lexer.next_lexeme(rules) {
            NextLexeme::Lexeme((kind, raw)) => {
                lexemes.push_back(Lexeme { kind, position, raw })
            }
            NextLexeme::Skip => {}
            NextLexeme::Finished => {
                break;
            }
        }
    }

    lexemes.into_iter().collect()
}
