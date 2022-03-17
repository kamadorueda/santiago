<h1 align="center">:leopard: Santiago</h2>

<p align="center">A lexing and parsing toolkit for Rust</p>

## Features

- ✔️ **Crab friendly** :crab:

  It's written in [Rust](https://www.rust-lang.org/),
  with zero dependencies,
  maximum portability.

- ✔️ **Human friendly**

  Built with a focus on ergonomics,
  defining a grammar is closely the same to its
  [Backus–Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).

- ✔️ **Powerful**

  Santiago can parse all [context-free languages](https://en.wikipedia.org/wiki/Context-free_grammar),
  including [ambiguous](https://en.wikipedia.org/wiki/Ambiguous_grammar)
  and [recursive](https://en.wikipedia.org/wiki/Recursive_grammar) grammars.

- ✔️ **Cutting-edge**

  Santiago uses the [Earley algorithm](https://en.wikipedia.org/wiki/Earley_parser).
  Its time and space performance is close to the known theoretical minimum.

## Getting started

Just checkout the examples:

- [calculator](./examples/calculator.rs)

You can run the examples by cloning this project and executing:

```sh
/santiago $ cargo run --example calculator
```

## Alternatives

Santiago aims to be an alternative to
[GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison),
[Yacc](https://en.wikipedia.org/wiki/Yacc) and
[Flex](<https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)>).

Sadly those long standing tools do not offer
[Rust](https://www.rust-lang.org/) compatibility.

<!--
Parsing takes (theoretical worst case):

- Linear time for [deterministic context-free grammars](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar).
- Quadratic time for [unambiguous-grammars](https://en.wikipedia.org/wiki/Ambiguous_grammar).
- Cubic time in the general case.

In practice the theoretical worst case is just theoretical, and performance is normally linear. -->

## Short term goals

In order:

1. Propagate position counters from the lexer to the parser.

1. Enforce resolving ambiguities in the input grammar.

1. Implement a grammar builder, so that defining a grammar is not that verbose

1. Implement a Flex-like interface for the lexer,
   so that you can really
   do complex lexing beyond 'char-by-char'

1. Release `1.0.0`.
