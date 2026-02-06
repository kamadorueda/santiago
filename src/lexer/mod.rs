// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Transform an input of characters
//! into groups of characters with related meaning.
//!
//! Please read the [crate documentation](crate) for more information and examples.
mod lexeme;
mod lexer_builder;
mod lexer_error;
mod lexer_rule;
mod lexer_rules;
mod position;

pub use lexeme::Lexeme;
pub use lexer_builder::LexerBuilder;
pub use lexer_error::LexerError;
pub(crate) use lexer_rule::LexerRule;
pub use lexer_rules::LexerRules;
pub use position::Position;
use std::collections::LinkedList;
use std::rc::Rc;

/// Core implementation of the algorithm.
///
/// [Lexer] is exposed so you can use its type and traits,
/// but normally you just use [lex()].
///
/// Please read the [crate documentation](crate) for more information and examples.
pub struct Lexer<'a> {
    input:                 &'a str,
    current_byte_index:    usize,
    /// The current match length.
    ///
    /// You can use this field to go back a few bytes in the input
    /// (for instance to simulate a "trailing context" pattern in Flex).
    pub current_match_len: usize,
    current_rule_name:     &'a str,
    position:              Position,
    states_stack:          LinkedList<&'a str>,
    marks:                 LinkedList<Position>,
}

/// Return type of a lexer rule action.
pub enum NextLexeme {
    /// We encountered an error.
    Error(LexerError),
    /// We returned part of a [Lexeme].
    Lexeme {
        /// Kind of the [Lexeme].
        kind: String,
        /// Raw contents of the [Lexeme].
        raw:  String,
    },
    /// Instructs the [Lexer] that the current [Lexeme] has been skipped.
    Skip,
    /// Instructs the [Lexer] that we reached the end of the input.
    Finished,
}

impl<'a> Lexer<'a> {
    fn next_lexeme(&mut self, rules: &'a LexerRules) -> NextLexeme {
        let state = self.states_stack.back().unwrap();
        let input_len = self.input.len();

        if self.current_byte_index < input_len
            || (self.current_byte_index == input_len && *state != "DEFAULT")
        {
            let mut matches_: LinkedList<(usize, usize)> = LinkedList::new();
            let input = &self.input[self.current_byte_index..];
            let active_rules = match rules.rules.get(*state) {
                Some(rules) => rules.as_slice(),
                None => &[],
            };

            for (rule_index, rule) in active_rules.iter().enumerate() {
                let matcher = &rule.matcher;

                if let Some(len) = matcher(input) {
                    matches_.push_back((len, rule_index));
                }
            }

            if matches_.is_empty() {
                let active_rule_names: Vec<String> = active_rules
                    .iter()
                    .map(|rule| format!("{:?}", rule.name))
                    .collect();

                return NextLexeme::Error(LexerError {
                    byte_index:   self.current_byte_index,
                    match_len:    None,
                    message:      format!(
                        "Expecting one of the following {} lexemes: {}",
                        active_rules.len(),
                        active_rule_names.join(", ")
                    ),
                    position:     self.position.clone(),
                    states_stack: self
                        .states_stack
                        .iter()
                        .map(|state| state.to_string())
                        .collect(),
                    marks: self
                        .marks
                        .iter()
                        .map(|mark| mark.clone())
                        .collect(),
                });
            }

            // Pick matches with the same maximum length
            // and take the first rule declared.
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
            self.current_rule_name = &active_rules[rule_index].name;

            active_rules[rule_index].action.clone()(self)
        } else {
            NextLexeme::Finished
        }
    }

    /// Return the current match contents.
    pub fn matched(&self) -> &str {
        &self.input[self.current_byte_index..][..self.current_match_len]
    }

    /// Instructs the [Lexer] that we want to include [Lexer::matched()]
    /// in the final [Lexeme]s.
    pub fn take(&mut self) -> NextLexeme {
        self.take_and_map(|matched| matched.to_string())
    }

    /// As [Lexer::take()]
    /// but applying `function` over [Lexer::matched()] first.
    pub fn take_and_map(&mut self, function: fn(&str) -> String) -> NextLexeme {
        let matched = self.matched().to_string();
        self.position.consume(&matched);
        self.current_byte_index += self.current_match_len;

        let kind = self.current_rule_name.to_string();
        let raw = function(&matched);

        NextLexeme::Lexeme { kind, raw }
    }

    /// Instructs the [Lexer] that we don't want to include [Lexer::matched()]
    /// in the final [Lexeme]s.
    pub fn skip(&mut self) -> NextLexeme {
        let matched = self.matched().to_string();
        self.position.consume(&matched);
        self.current_byte_index += self.current_match_len;

        NextLexeme::Skip
    }

    /// Instructs the [Lexer] that we want to include [Lexer::matched()]
    /// in the final [Lexeme]s
    /// and that we want the current input position
    /// to be set to the start of this [Lexeme],
    /// so that it's matched again.
    ///
    /// This can be useful after a [Lexer::push_state()] for instance.
    pub fn take_and_retry(&mut self) -> NextLexeme {
        self.take_and_map_and_retry(|matched| matched.to_string())
    }

    /// As [Lexer::take_and_retry()]
    /// but applying `function` over [Lexer::matched()] first.
    pub fn take_and_map_and_retry(
        &mut self,
        function: fn(&str) -> String,
    ) -> NextLexeme {
        let matched = self.matched().to_string();

        let kind = self.current_rule_name.to_string();
        let raw = function(&matched);

        NextLexeme::Lexeme { kind, raw }
    }

    /// Instructs the [Lexer] that we don't want to include [Lexer::matched()]
    /// in the final [Lexeme]s
    /// and that we want the current input position
    /// to be set to the start of this [Lexeme],
    /// so that it's matched again.
    ///
    /// This can be useful after a [Lexer::push_state()] for instance.
    pub fn skip_and_retry(&mut self) -> NextLexeme {
        NextLexeme::Skip
    }

    /// Returns the current state of the [Lexer].
    pub fn current_state(&mut self) -> &str {
        self.states_stack.back().unwrap()
    }

    /// Goes back to the previous [Lexer] state.
    pub fn pop_state(&mut self) {
        self.states_stack.pop_back();
    }

    /// Pushes a new state into the [Lexer] stack.
    pub fn push_state(&mut self, state: &'a str) {
        self.states_stack.push_back(state);
    }

    /// Tells the [Lexer] that we found an error.
    pub fn error(&mut self, message: &str) -> NextLexeme {
        NextLexeme::Error(LexerError {
            message:      message.to_string(),
            byte_index:   self.current_byte_index,
            match_len:    Some(self.current_match_len),
            position:     self.position.clone(),
            states_stack: self
                .states_stack
                .iter()
                .map(|state| state.to_string())
                .collect(),
            marks: self
                .marks
                .iter()
                .map(|mark| mark.clone())
                .collect(),
        })
    }

    /// Adds a new position to the [Lexer]'s marks stack.
    pub fn push_mark(&mut self) {
        self.marks.push_back(self.position.clone());
    }

    /// Removes the last position from the [Lexer]'s marks stack.
    pub fn pop_mark(&mut self) {
        self.marks.pop_back();
    }
}

/// Perform lexical analysis of the given input according to the provided rules.
pub fn lex(
    rules: &LexerRules,
    input: &str,
) -> Result<Vec<Rc<Lexeme>>, LexerError> {
    let mut lexer = Lexer {
        input,
        current_byte_index: 0,
        current_match_len: 0,
        current_rule_name: "",
        position: Position { column: 1, line: 1 },
        states_stack: LinkedList::new(),
        marks: LinkedList::new(),
    };

    lexer.push_state("DEFAULT");

    let mut lexemes = LinkedList::new();

    loop {
        let position = lexer.position.clone();

        match lexer.next_lexeme(rules) {
            NextLexeme::Error(error) => {
                return Err(error);
            }
            NextLexeme::Lexeme { kind, raw } => {
                lexemes.push_back(Rc::new(Lexeme { kind, position, raw }))
            }
            NextLexeme::Skip => {}
            NextLexeme::Finished => {
                break;
            }
        }
    }

    Ok(lexemes.into_iter().collect())
}
