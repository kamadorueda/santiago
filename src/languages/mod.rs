// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

//! Built-in lexers and parsers for different languages.
//!
//! Please read the [crate documentation](crate) for more information and examples.

#[cfg(feature = "language_calculator")]
pub mod calculator;

#[cfg(feature = "language_nix")]
pub mod nix;
