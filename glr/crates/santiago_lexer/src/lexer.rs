use crate::cursor_movement::CursorMovement;
use crate::lex::Cursor;
use crate::matched_input::MatchedInput;

pub type LexerAction = for<'eph, 'input> fn(
    &'eph mut Cursor,
    MatchedInput<'input>,
) -> CursorMovement<'input>;

pub struct Lexer {
    pub get_instruction: fn(&str) -> usize,
}
