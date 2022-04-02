<!--
SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>

SPDX-License-Identifier: GPL-3.0-only
-->

<h1 align="center">🐆 Santiago</h2>

<p align="center">A lexing and parsing toolkit for Rust</p>

<p align="center">
  <a href="https://buildkite.com/kamadorueda/santiago">
    <img
      alt="CI/CD"
      src="https://badge.buildkite.com/4b931515838b1cf833c90ef188b455f4fbb336f2b416fec20c.svg?branch=main"
    >
    </img>
  </a>
  <a href="https://docs.rs/santiago">
    <img
      alt="Documentation"
      src="https://img.shields.io/docsrs/santiago?color=brightgreen"
    >
    </img>
  </a>
  <a href="https://crates.io/crates/santiago">
    <img
      alt="Version"
      src="https://img.shields.io/crates/v/santiago?color=brightgreen"
    >
    </img>
  </a>
  <a href="https://spdx.org/licenses/GPL-3.0-only.html">
    <img
      alt="License"
      src="https://img.shields.io/crates/l/santiago?color=brightgreen"
    >
    </img>
  </a>
  <!-- <a href="https://crates.io/crates/santiago">
    <img
      alt="Downloads"
      src="https://img.shields.io/crates/d/santiago"
    >
    </img>
  </a> -->

</p>

Santiago provides you:

- A library for defining any
  [context-free grammar](https://en.wikipedia.org/wiki/Context-free_grammar).
- A [Lexical analysis](https://en.wikipedia.org/wiki/Lexical_analysis) module.
- And facilities for building interpreters or compilers of the language.

With Santiago you have everything that is needed
to build your own programming language!

## Features

- ✔️ **Fast** 🦀

  It's written in [Rust](https://www.rust-lang.org/),
  with zero dependencies and
  maximum portability in mind.

- ✔️ **Easy to use**

  Defining a grammar is closely the same to its
  [Backus–Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).

  Creating a lexer is a matter of mapping some strings.

  Error messages contain useful information.

- ✔️ **Powerful**

  Santiago can parse all [context-free languages](https://en.wikipedia.org/wiki/Context-free_grammar) without exceptions.
  <!--
    It performs:

    - Linear time and space lexing.
    - Linear time and space parsing of
      [deterministic grammars](https://en.wikipedia.org/wiki/Deterministic_context-free_grammar).
    - Linear space and quadratic time of
      [unambiguous grammars](https://en.wikipedia.org/wiki/Unambiguous_grammar).
    - Linear space and cubic time of
      highly [ambiguous grammars](https://en.wikipedia.org/wiki/Ambiguous_grammar).
  -->

- ✔️ **Reliable**

  High coverage, battle tested.

- ✔️ **Compatible**

  Santiago is inspired and aims to be an alternative to
  [GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison),
  [Yacc](https://en.wikipedia.org/wiki/Yacc) and
  [Flex](<https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)>),
  which are amazing tools,
  but not compatible with rust.

## Getting started

Just read the [docs](https://docs.rs/santiago),
we have plenty of examples over there,
plus detailed explanation of each component.

Alternatively,
you can checkout more examples
in the [tests](https://github.com/kamadorueda/santiago/tree/main/tests)
folder.

We hope you find Santiago useful!

And don’t forget to give us a star ⭐
