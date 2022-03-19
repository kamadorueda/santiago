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

## Features

- ✔️ **Crab friendly** 🦀

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

- ✔️ **Modern**

  Santiago is inspired and aims to be an alternative to
  [GNU Bison](https://en.wikipedia.org/wiki/GNU_Bison),
  [Yacc](https://en.wikipedia.org/wiki/Yacc) and
  [Flex](<https://en.wikipedia.org/wiki/Flex_(lexical_analyser_generator)>).

## Getting started

Just checkout the examples:

- [calculator](./examples/calculator.rs)

You can run the examples by cloning this project and executing:

```sh
/santiago $ cargo run --example $name
```

## Short term goals

In order:

1. Enforce resolving ambiguities in the input grammar.

1. Release `1.0.0`.
