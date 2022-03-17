use crate::lex::{lexeme::Lexeme, position::Position};
use std::collections::LinkedList;

pub struct Lexer<'a> {
    input:       &'a str,
    input_index: usize,
}

impl<'a> Lexer<'a> {
    fn next_lexeme(&mut self) -> Option<(String, String)> {
        match &self.input[self.input_index..].chars().next() {
            Some(raw) => {
                let raw = raw.to_string();
                let kind = raw.clone();
                self.input_index += 1;

                Some((kind, raw))
            }
            None => None,
        }
    }

    pub fn lex(input: &str) -> Vec<Lexeme> {
        let mut lexemes = LinkedList::new();
        let mut lexer = Lexer { input, input_index: 0 };

        let mut column: usize = 1;
        let mut line: usize = 1;

        loop {
            let position = Position { column, index: lexer.input_index, line };

            match lexer.next_lexeme() {
                Some((kind, raw)) => {
                    for char in
                        lexer.input[position.index..lexer.input_index].chars()
                    {
                        if char == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                    }

                    lexemes.push_back(Lexeme { kind, position, raw })
                }
                _ => break,
            }
        }

        lexemes.into_iter().collect()
    }
}
