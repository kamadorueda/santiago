/// Ways in which repeated uses of rules with the same precedence nest.
#[derive(Clone, Eq, PartialEq)]
pub enum Associativity {
    /// Specifies left associativity: `(x op y) op z`.
    Left,
    /// Specifies right associativity: `x op (y op z)`.
    Right,
    /// Specifies that `x op y op z` is considered a syntax error.
    None,
}
