use crate::grammar::Associativity;

/// Internal representation of precedence and [Associativity].
///
/// [Disambiguation] is exposed so you can use its type and traits
/// but normally you create a [Disambiguation]
/// by using a [GrammarBuilder](crate::grammar::GrammarBuilder).
#[derive(Clone)]
pub struct Disambiguation {
    pub(crate) associativity: Associativity,
    pub(crate) precedence:    usize,
}
