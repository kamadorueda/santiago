// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::parser::state::State;
use std::collections::HashSet;

pub(crate) struct Column {
    pub(crate) index:  usize,
    pub(crate) kind:   String,
    pub(crate) states: Vec<State>,
    pub(crate) unique: HashSet<u64>,
}

impl Column {
    pub(crate) fn add(&mut self, state: State) {
        let mut state = state;
        let digest = state.hash_me();

        if !self.unique.contains(&digest) {
            self.unique.insert(digest);
            state.end_column = self.index;
            self.states.push(state);
        }
    }
}
