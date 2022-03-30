// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Associativity;
use crate::grammar::Grammar;
use crate::grammar::Symbol;
use crate::lexer::Lexeme;
use crate::parser::ParserColumn;
use crate::parser::ParserState;
use std::collections::HashMap;
use std::rc::Rc;

/// Representation of an AST
#[derive(Clone, Debug, Hash)]
pub enum Tree {
    /// Leaf nodes of the tree, containing a [Lexeme].
    Leaf(Lexeme),
    /// Group of many [Tree::Leaf].
    Node {
        /// Name of the grammar rule that produced this node.
        kind:   String,
        /// Children of this Node.
        leaves: Vec<Rc<Tree>>,
    },
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn recurse(
            f: &mut std::fmt::Formatter<'_>,
            depth: usize,
            tree: &Tree,
        ) -> std::fmt::Result {
            match tree {
                Tree::Leaf(lexeme) => {
                    writeln!(f, "{}{lexeme}", "  ".repeat(depth + 1))
                }
                Tree::Node { kind, leaves } => {
                    let result = writeln!(f, "{}{kind}", "  ".repeat(depth));

                    for leaf in leaves {
                        recurse(
                            f,
                            depth
                                + match **leaf {
                                    Tree::Leaf { .. } => 0,
                                    Tree::Node { .. } => 1,
                                },
                            leaf,
                        )?;
                    }

                    result
                }
            }
        }

        recurse(f, 0, self)
    }
}

pub(crate) fn build(
    grammar: &Grammar,
    lexemes: &[Lexeme],
    columns: &[ParserColumn],
    state: &ParserState,
) -> Vec<Rc<Tree>> {
    let mut cache: HashMap<u64, Rc<Vec<Rc<Tree>>>> = HashMap::new();

    for column in columns.iter() {
        for state_partial in &column.states {
            build_forest(&mut cache, grammar, lexemes, columns, state_partial);
        }
    }

    (*cache.remove(&state.hash_me()).unwrap()).clone()
}

fn build_forest(
    cache: &mut HashMap<u64, Rc<Vec<Rc<Tree>>>>,
    grammar: &Grammar,
    lexemes: &[Lexeme],
    columns: &[ParserColumn],
    state: &ParserState,
) -> Rc<Vec<Rc<Tree>>> {
    let key = state.hash_me();
    match cache.get(&key) {
        Some(forest) => forest.clone(),
        None => {
            let forest = Rc::new(build_forest_helper(
                cache,
                grammar,
                lexemes,
                columns,
                vec![],
                state,
                state.production.symbols.len().overflowing_sub(1).0,
                state.end_column,
            ));

            cache.insert(key, forest.clone());

            forest
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_forest_helper(
    cache: &mut HashMap<u64, Rc<Vec<Rc<Tree>>>>,
    grammar: &Grammar,
    lexemes: &[Lexeme],
    columns: &[ParserColumn],

    leaves: Vec<Rc<Tree>>,
    state: &ParserState,
    symbol_index: usize,
    end_column: usize,
) -> Vec<Rc<Tree>> {
    if symbol_index == usize::MAX {
        if leaves.len() == 1 && matches!(*leaves[0], Tree::Node { .. }) {
            return leaves;
        }

        return vec![Rc::new(Tree::Node {
            kind: (*state.name).clone(),
            leaves,
        })];
    }

    match &state.production.symbols[symbol_index] {
        Symbol::Lexeme(_) => {
            let lexeme = &lexemes[end_column - 1];
            let mut leaves = leaves;
            let mut leaves_extended = vec![Rc::new(Tree::Leaf(lexeme.clone()))];
            leaves_extended.append(&mut leaves);

            build_forest_helper(
                cache,
                grammar,
                lexemes,
                columns,
                leaves_extended,
                state,
                symbol_index.overflowing_sub(1).0,
                state.end_column - 1,
            )
        }
        Symbol::Rule(name) => {
            let mut forest = Vec::new();

            for state_partial in columns[end_column]
                .states
                .iter()
                .take_while(|state_partial| *state_partial != state)
                .filter(|state_partial| {
                    *state_partial.name == *name
                        && (symbol_index > 0
                            || state_partial.start_column == state.start_column)
                        && satisfies_disambiguation(
                            grammar,
                            state_partial,
                            state,
                        )
                })
            {
                for alternative in (*build_forest(
                    cache,
                    grammar,
                    lexemes,
                    columns,
                    state_partial,
                ))
                .clone()
                {
                    let mut leaves_extended = vec![alternative];
                    leaves_extended.append(&mut leaves.clone());

                    forest.append(&mut build_forest_helper(
                        cache,
                        grammar,
                        lexemes,
                        columns,
                        leaves_extended,
                        state,
                        symbol_index.overflowing_sub(1).0,
                        state_partial.start_column,
                    ));
                }
            }

            forest
        }
    }
}

fn satisfies_disambiguation(
    grammar: &Grammar,
    state_partial: &ParserState,
    state: &ParserState,
) -> bool {
    if let (Some(partial_index), Some(index)) = (
        get_disambiguation(grammar, state_partial),
        get_disambiguation(grammar, state),
    ) {
        if let (Symbol::Rule(name_partial), Symbol::Rule(name)) = (
            &state_partial.production.symbols[partial_index],
            &state.production.symbols[index],
        ) {
            if let (Some(rule_partial), Some(rule)) =
                (grammar.rules.get(name_partial), grammar.rules.get(name))
            {
                if let (Some(disambiguation_partial), Some(disambiguation)) =
                    (&rule_partial.disambiguation, &rule.disambiguation)
                {
                    if disambiguation_partial.precedence
                        < disambiguation.precedence
                    {
                        return false;
                    }

                    if disambiguation_partial.precedence
                        == disambiguation.precedence
                    {
                        if state_partial.end_column == state.end_column
                            && disambiguation_partial.associativity
                                == Associativity::Left
                        {
                            return false;
                        }

                        if state_partial.start_column == state.start_column
                            && disambiguation.associativity
                                == Associativity::Right
                        {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

fn get_disambiguation(grammar: &Grammar, state: &ParserState) -> Option<usize> {
    let mut disambiguations =
        state.production.symbols.iter().enumerate().filter_map(
            |(index, symbol)| {
                if let Symbol::Rule(name) = symbol {
                    if let Some(rule) = grammar.rules.get(name) {
                        if let Some(_) = &rule.disambiguation {
                            return Some(index);
                        }
                    }
                }

                None
            },
        );

    disambiguations.next()
}
