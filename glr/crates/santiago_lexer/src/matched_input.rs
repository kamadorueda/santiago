use std::borrow::Cow;

pub struct MatchedInput<'input> {
    pub id: usize,
    pub content: Cow<'input, str>,
    pub len: usize,
}
