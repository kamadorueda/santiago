use std::borrow::Cow;

use regex::Regex;

use crate::cursor_movement::CursorMovement;
use crate::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::lexer::LexerAction;
use crate::lexer_instruction::LexerInstruction;
use crate::matched_input::MatchedInput;

pub struct Cursor<'input> {
    lexer: &'static Lexer,
    lexer_actions: &'static [LexerAction],
    lexer_instructions: &'static [LexerInstruction],
    lexer_symbol_indexes: &'static [usize],
    lexer_matchers: &'static [Regex],
    input: &'input str,
    instructions_stack: Vec<usize>,
    byte_index: usize,
}

impl<'input> Cursor<'input> {
    fn next<'eph>(&'eph mut self) -> CursorMovement<'input> {
        let state = self.current_state();
        let input = self.input;
        let input_len = input.len();
        let byte_index = self.byte_index;

        if byte_index < input_len || (byte_index == input_len && state != 0) {
            let input_remaining = &input[byte_index..];

            let maybe_captures =
                self.lexer_matchers[state].captures(input_remaining);

            let lengths_and_indexes: Vec<(usize, usize)> =
                if let Some(captures) = maybe_captures {
                    captures
                        .iter()
                        .skip(1)
                        .enumerate()
                        .filter_map(|(index, match_)| {
                            match_.map(|match_| (match_.end(), index))
                        })
                        .collect()
                } else {
                    // let active_rule_names: Vec<String> = active_rules
                    //     .iter()
                    //     .map(|rule| format!("{:?}", rule.name))
                    //     .collect();

                    // return CursorMovement::Error(LexerError {
                    //     byte_index: self.current_byte_index,
                    //     match_len: None,
                    //     message: format!(
                    //         "Expecting one of the following {} lexemes: {}",
                    //         active_rules.len(),
                    //         active_rule_names.join(", ")
                    //     ),
                    //     position: self.position.clone(),
                    //     states_stack: self
                    //         .states_stack
                    //         .iter()
                    //         .map(|state| state.to_string())
                    //         .collect(),
                    // });
                    return CursorMovement::Error;
                };

            // Pick matches with the same maximum length
            // and take the first rule declared.
            let max_len = *lengths_and_indexes
                .iter()
                .max_by(|(len1, _), (len2, _)| len1.cmp(len2))
                .map(|(len, _)| len)
                .unwrap();

            let (len, index): (usize, usize) = lengths_and_indexes
                .into_iter()
                .filter(|(len, _)| len == &max_len)
                .min_by(|(_, index1), (_, index2)| index1.cmp(index2))
                .unwrap();

            let index = self.lexer_instructions[state].ids[index];
            let action = self.lexer_actions[index];

            action(self, MatchedInput {
                id: index,
                content: Cow::from(&input_remaining[..len]),
                len,
            })
        } else {
            CursorMovement::Stop
        }
    }

    /// Instructs the [Lexer] that we want to include [Lexer::matched()]
    /// in the final [Lexeme]s.
    pub fn take<'eph, 'matched>(
        &'eph mut self,
        matched: MatchedInput<'matched>,
    ) -> CursorMovement<'matched> {
        let movement = CursorMovement::Lexeme(Lexeme {
            symbol_index: self.lexer_symbol_indexes[matched.id],
            byte_index: self.byte_index,
            content: matched.content,
        });

        self.byte_index += matched.len;

        movement
    }

    /// As [Lexer::take()]
    /// but applying `function` over [Lexer::matched()] first.
    // pub fn take_and_map<'eph>(
    //     &'eph mut self,
    //     function: fn(&str) -> String,
    // ) -> CursorMovement<'input> {
    //     let matched = self.matched().to_owned();

    //     let start_byte_index = self.byte_index;
    //     self.byte_index += self.current_match_len;

    //     CursorMovement::Lexeme(Lexeme {
    //         id: self.current_index,
    //         byte_index: start_byte_index,
    //         content: Cow::from(function(&matched)),
    //     })
    // }

    /// Instructs the [Lexer] that we don't want to include [Lexer::matched()]
    /// in the final [Lexeme]s.
    pub fn skip<'eph, 'matched>(
        &'eph mut self,
        matched: MatchedInput<'matched>,
    ) -> CursorMovement<'matched> {
        self.byte_index += matched.len;

        CursorMovement::Skip
    }

    /// Instructs the [Lexer] that we want to include [Lexer::matched()]
    /// in the final [Lexeme]s
    /// and that we want the current input position
    /// to be set to the start of this [Lexeme],
    /// so that it's matched again.
    ///
    /// This can be useful after a [Lexer::push_state()] for instance.
    // pub fn take_and_retry(&mut self) -> CursorMovement {
    //     self.take_and_map_and_retry(identity)
    // }

    /// As [Lexer::take_and_retry()]
    /// but applying `function` over [Lexer::matched()] first.
    // pub fn take_and_map_and_retry(
    //     &mut self,
    //     function: fn(Cow<'input, str>) -> Cow<'input, str>,
    // ) -> CursorMovement {
    //     CursorMovement::Lexeme(Lexeme {
    //         id: self.current_index,
    //         byte_index: self.byte_index,
    //         content: function(self.matched()),
    //     })
    // }

    /// Instructs the [Lexer] that we don't want to include [Lexer::matched()]
    /// in the final [Lexeme]s
    /// and that we want the current input position
    /// to be set to the start of this [Lexeme],
    /// so that it's matched again.
    ///
    /// This can be useful after a [Lexer::push_state()] for instance.
    // pub fn skip_and_retry(&mut self) -> CursorMovement {
    //     CursorMovement::Skip
    // }

    /// Returns the current state of the [Lexer].
    pub fn current_state(&mut self) -> usize {
        *self.instructions_stack.last().unwrap()
    }

    /// Goes back to the previous [Lexer] state.
    pub fn pop_state(&mut self) {
        self.instructions_stack.pop();
    }

    /// Pushes a new state into the [Lexer] stack.
    pub fn push_state(&mut self, state: &str) {
        self.instructions_stack.push((self.lexer.get_instruction)(state));
    }
}

pub fn lex<'input>(
    lexer: &'static Lexer,
    lexer_actions: &'static [LexerAction],
    lexer_instructions: &'static [LexerInstruction],
    lexer_matchers: &'static [Regex],
    lexer_symbol_indexes: &'static [usize],
    input: &'input str,
) -> Vec<Lexeme<'input>> {
    let mut lexemes = Vec::new();

    let mut cursor = Cursor {
        lexer,
        lexer_actions,
        lexer_instructions,
        lexer_matchers,
        lexer_symbol_indexes,
        input,

        byte_index: 0,
        instructions_stack: Vec::from([0]),
    };

    loop {
        match cursor.next() {
            CursorMovement::Error => {
                panic!();
            },
            CursorMovement::Lexeme(lexeme) => {
                lexemes.push(lexeme);
            },
            CursorMovement::Skip => {},
            CursorMovement::Stop => {
                break;
            },
        }
    }

    lexemes.push(Lexeme {
        symbol_index: *lexer_symbol_indexes.last().unwrap(),
        byte_index: input.len(),
        content: Cow::default(),
    });

    lexemes
}
