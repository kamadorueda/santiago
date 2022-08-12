use quote::ToTokens;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ParserAction {
    /// Nothing else to do
    Finish,
    /// Reduce the given production
    Reduce(usize),
    /// Shift to the given state
    Shift(usize),
}

impl ToTokens for ParserAction {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        use quote::__private::*;
        parse(tokens, &format!("{self:?}"));
    }
}
