use std::borrow::Cow;

#[derive(Debug)]
pub struct Lexeme<'content> {
    pub symbol_index: usize,
    pub byte_index: usize,
    pub content: Cow<'content, str>,
}
