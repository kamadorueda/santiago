<h1 align="center">Santiago :leopard:</h2>

<p align="center">A parsing toolkit for Rust</p>

Santiago is a parsing toolkit for Rust :crab,
built with a focus on ergonomics, performance and modularity.

Santiago can parse all [context-free languages](https://en.wikipedia.org/wiki/Context-free_grammar).

To put it simply,
it's is capable of parsing
almost anything out there
including [ambiguous](https://en.wikipedia.org/wiki/Ambiguous_grammar) and [recursive](https://en.wikipedia.org/wiki/Recursive_grammar) grammars.

Santiago is a [Rust](https://www.rust-lang.org/) alternative to
[GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison),
[Yacc](https://en.wikipedia.org/wiki/Yacc) and
[Flex](<https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)>).

<!--
Parsing takes (theoretical worst case):

- Linear time for [deterministic context-free grammars](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar).
- Quadratic time for [unambiguous-grammars](https://en.wikipedia.org/wiki/Ambiguous_grammar).
- Cubic time in the general case.

In practice the theoretical worst case is just theoretical, and performance is normally linear. -->

Short term goals:

- Generalize the lexeme kinds from `char` to an `Enum`.
- Implement a lexer that turns a `&str` into an `&[Enum]`
- Generalize non terminals to another `Enum`.
  So that the AST is statically defined.
