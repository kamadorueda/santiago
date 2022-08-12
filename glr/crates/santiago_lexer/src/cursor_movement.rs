use crate::lexeme::Lexeme;

pub enum CursorMovement<'input> {
    Error,
    Lexeme(Lexeme<'input>),
    Skip,
    Stop,
}
