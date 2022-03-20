// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::grammar_rule::GrammarRule;
use crate::grammar::production::Production;
use crate::grammar::symbol::Symbol;
use crate::lexer::lexeme::Lexeme;
use crate::parser::column::Column;
use crate::parser::forest::build_forest;
use crate::parser::forest::Forest;
use crate::parser::state::State;
use std::collections::HashSet;

const START_RULE_NAME: &str = "Γ";

fn predict(column: &mut Column, rule: &GrammarRule) {
    for production in &rule.productions {
        column.add(State {
            name:         rule.name.clone(),
            production:   production.clone(),
            dot_index:    0,
            start_column: column.index,
            end_column:   usize::MAX,
        });
    }
}

fn scan(column: &mut Column, state: &State, kind: &str) {
    if kind == column.kind {
        column.add(State {
            name:         state.name.clone(),
            production:   state.production.clone(),
            start_column: state.start_column,
            end_column:   usize::MAX,
            dot_index:    state.dot_index + 1,
        });
    }
}

fn complete(columns: &mut Vec<Column>, column_index: usize, state: &State) {
    if state.completed() {
        for st in &columns[state.start_column].states.clone() {
            if let Some(Symbol::Rule(name)) = st.next_symbol() {
                if name == state.name {
                    columns[column_index].add(State {
                        name:         st.name.clone(),
                        production:   st.production.clone(),
                        start_column: st.start_column,
                        end_column:   usize::MAX,
                        dot_index:    st.dot_index + 1,
                    });
                }
            }
        }
    }
}

pub fn parse(
    rules: &[GrammarRule],
    lexemes: &[Lexeme],
) -> Result<Vec<Forest>, String> {
    let mut columns: Vec<Column> = (0..=lexemes.len())
        .map(|index| {
            if index == 0 {
                Column {
                    index,
                    kind: '^'.to_string(),
                    states: vec![],
                    unique: HashSet::new(),
                }
            } else {
                Column {
                    index,
                    kind: lexemes[index - 1].kind.clone(),
                    states: Vec::new(),
                    unique: HashSet::new(),
                }
            }
        })
        .collect();

    columns[0].states.push(State {
        name:         START_RULE_NAME.to_string(),
        production:   Production {
            symbols: vec![Symbol::Rule(rules[0].name.clone())],
        },
        start_column: 0,
        end_column:   usize::MAX,
        dot_index:    0,
    });

    for column_index in 0..columns.len() {
        let mut state_index = 0;
        let mut state_len = columns[column_index].states.len();

        while state_index < state_len {
            let state = columns[column_index].states[state_index].clone();

            if state.completed() {
                complete(&mut columns, column_index, &state);
            } else {
                match state.next_symbol().unwrap() {
                    Symbol::Rule(name) => {
                        let rule = rules
                            .iter()
                            .find(|rule| rule.name == name)
                            .unwrap();
                        predict(&mut columns[column_index], rule);
                    }
                    Symbol::Lexeme(kind) => {
                        if column_index + 1 < columns.len() {
                            scan(&mut columns[column_index + 1], &state, &kind);
                        }
                    }
                }
            }

            state_index += 1;
            state_len = columns[column_index].states.len();
        }
    }

    for column in columns.iter_mut() {
        column.states =
            column.states.iter().filter(|c| c.completed()).cloned().collect();
    }

    // println!();
    // println!("Columns:");
    // for (column_index, column) in columns.iter().enumerate() {
    //     println!("  {column_index}");
    //     for state in &column.states {
    //         println!("    {state}");
    //     }
    // }

    for state in &columns.last().unwrap().states {
        if state.name == START_RULE_NAME && state.completed() {
            return Ok(build_forest(rules, lexemes, &columns, state));
        }
    }

    Err(String::new())
}
