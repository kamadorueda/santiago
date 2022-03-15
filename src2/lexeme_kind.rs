#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LexemeKind {
    Asterisk, // *
    Hyphen,   // -
    Int,      //
    Plus,     // +
    Slash,    // /
}
