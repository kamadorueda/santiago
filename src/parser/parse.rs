// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Grammar;
use crate::grammar::GrammarRule;
use crate::grammar::ProductionKind;
use crate::grammar::START_RULE_NAME;
use crate::lexer::Lexeme;
use crate::parser::tree::build;
use crate::parser::ParseError;
use crate::parser::ParserColumn;
use crate::parser::ParserState;
use crate::parser::Tree;
use std::collections::HashSet;
use std::rc::Rc;

fn predict<AST>(
    columns: &mut Vec<ParserColumn<AST>>,
    column_index: usize,
    rule: &GrammarRule<AST>,
) {
    for production in &rule.productions {
        if column_index + 1 < columns.len()
            && !production.target_lexemes.borrow().is_empty()
            && !production
                .target_lexemes
                .borrow()
                .contains(&columns[column_index + 1].kind)
        {
            continue;
        }

        let new_state = ParserState {
            rule_name:    rule.name.clone(),
            production:   production.clone(),
            dot_index:    0,
            start_column: column_index,
            end_column:   usize::MAX,
        };
        columns[column_index].add(new_state);
    }
}

fn scan<AST>(
    columns: &mut Vec<ParserColumn<AST>>,
    column_index: usize,
    state_index: usize,
) {
    let state = &columns[column_index].states[state_index];
    let new_state = ParserState {
        rule_name:    state.rule_name.clone(),
        production:   state.production.clone(),
        start_column: state.start_column,
        end_column:   usize::MAX,
        dot_index:    state.dot_index + 1,
    };
    columns[column_index + 1].add(new_state);
}

fn complete<AST>(
    columns: &mut Vec<ParserColumn<AST>>,
    column_index: usize,
    state_index: usize,
) {
    let state_name = &columns[column_index].states[state_index].rule_name;
    let state_start_column =
        columns[column_index].states[state_index].start_column;

    let indexes: Vec<usize> = columns[state_start_column]
        .states
        .iter()
        .enumerate()
        .filter_map(|(index, st)| {
            if let ProductionKind::Rules = st.production.kind {
                match st.next_symbol() {
                    Some(rule_name) => {
                        if *rule_name == **state_name {
                            Some(index)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect();

    for index in indexes {
        let st = &columns[state_start_column].states[index];
        let new_state = ParserState {
            rule_name:    st.rule_name.clone(),
            production:   st.production.clone(),
            start_column: st.start_column,
            end_column:   usize::MAX,
            dot_index:    st.dot_index + 1,
        };
        columns[column_index].add(new_state);
    }
}

/// Parse the provided [Lexeme]s with the given [Grammar].
///
/// Return all possible Parse Trees.
pub fn parse<AST>(
    grammar: &Grammar<AST>,
    lexemes: &[Lexeme],
) -> Result<Vec<Rc<Tree<AST>>>, ParseError<AST>> {
    let mut columns: Vec<ParserColumn<AST>> = earley(grammar, lexemes);

    let mut parent = None;
    for state in &columns.last().unwrap().states {
        if *state.rule_name == START_RULE_NAME && state.completed() {
            parent = Some(state.clone());
            break;
        }
    }

    if let Some(state) = parent {
        for column in columns.iter_mut() {
            column.states = column
                .states
                .iter()
                .filter(|state| state.completed())
                .cloned()
                .collect();
        }

        Ok(build(grammar, lexemes, &columns, &state))
    } else if lexemes.is_empty() {
        Err(ParseError { at: None, states: columns[0].states.clone() })
    } else {
        let column = columns
            .iter()
            .rev()
            .find(|column| !column.states.is_empty())
            .unwrap();

        Err(ParseError {
            at:     lexemes.get(column.index.overflowing_sub(1).0).cloned(),
            states: column.states.clone(),
        })
    }
}

/// Parse the provided [Lexemes](Lexeme) with the given [Grammar]
/// and the [Earley algorithm](https://en.wikipedia.org/wiki/Earley_parser).
pub fn earley<AST>(
    grammar: &Grammar<AST>,
    lexemes: &[Lexeme],
) -> Vec<ParserColumn<AST>> {
    let mut columns: Vec<ParserColumn<AST>> = (0..=lexemes.len())
        .map(|index| {
            if index == 0 {
                ParserColumn {
                    index,
                    kind: '^'.to_string(),
                    states: vec![],
                    unique: HashSet::new(),
                }
            } else {
                ParserColumn {
                    index,
                    kind: lexemes[index - 1].kind.clone(),
                    states: Vec::new(),
                    unique: HashSet::new(),
                }
            }
        })
        .collect();

    let rule_name = Rc::new(START_RULE_NAME.to_string());
    columns[0].add(ParserState {
        production: grammar.rules.get(&rule_name).unwrap().productions[0]
            .clone(),
        rule_name,
        start_column: 0,
        end_column: usize::MAX,
        dot_index: 0,
    });

    for column_index in 0..columns.len() {
        let mut state_index = 0;
        let mut state_len = columns[column_index].states.len();
        let mut predicted_names = HashSet::new();

        while state_index < state_len {
            let state = &columns[column_index].states[state_index];

            if columns[column_index].states[state_index].completed() {
                complete(&mut columns, column_index, state_index);
            } else {
                match state.production.kind {
                    ProductionKind::Rules => {
                        let rule_name = state.next_symbol().unwrap().clone();
                        if !predicted_names.contains(&rule_name) {
                            let rule = grammar.rules.get(&rule_name).unwrap();
                            predicted_names.insert(rule_name);
                            predict(&mut columns, column_index, rule);
                        }
                    }
                    ProductionKind::Lexemes => {
                        if column_index + 1 < columns.len()
                            && *state.next_symbol().unwrap()
                                == columns[column_index + 1].kind
                        {
                            scan(&mut columns, column_index, state_index);
                        }
                    }
                }
            }

            state_index += 1;
            state_len = columns[column_index].states.len();
        }
    }

    // println!();
    // println!("Columns:");
    // for (column_index, column) in columns.iter().enumerate() {
    //     println!("  {column_index}");
    //     for state in &column.states {
    //         println!("    {state}");
    //     }
    // }

    columns
}
