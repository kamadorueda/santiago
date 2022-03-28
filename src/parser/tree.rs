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
        leaves: Vec<Tree>,
    },
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn recurse(depth: usize, forest: &Tree) -> String {
            match forest {
                Tree::Leaf(lexeme) => {
                    format!("{}{lexeme}\n", "  ".repeat(depth + 1),)
                }
                Tree::Node { kind, leaves } => {
                    let mut result = String::new();
                    result += &format!("{}{kind}\n", "  ".repeat(depth));
                    for leaf in leaves {
                        result += &recurse(
                            depth
                                + match leaf {
                                    Tree::Leaf { .. } => 0,
                                    Tree::Node { .. } => 1,
                                },
                            leaf,
                        );
                    }

                    result
                }
            }
        }

        write!(f, "{}", &recurse(0, self))
    }
}

pub(crate) fn build(
    grammar: &Grammar,
    lexemes: &[Lexeme],
    columns: &[ParserColumn],
    state: &ParserState,
) -> Vec<Tree> {
    let mut cache: HashMap<u64, Vec<Tree>> = HashMap::new();

    for column in columns.iter() {
        for state_partial in &column.states {
            build_forest(&mut cache, grammar, lexemes, columns, state_partial);
        }
    }

    cache.remove(&state.hash_me()).unwrap()
}

fn build_forest(
    cache: &mut HashMap<u64, Vec<Tree>>,
    grammar: &Grammar,
    lexemes: &[Lexeme],
    columns: &[ParserColumn],
    state: &ParserState,
) -> Vec<Tree> {
    let key = state.hash_me();
    match cache.get(&key) {
        Some(forest) => forest.clone(),
        None => {
            let forest = build_forest_helper(
                cache,
                grammar,
                lexemes,
                columns,
                vec![],
                state,
                state.production.symbols.len().overflowing_sub(1).0,
                state.end_column,
            );

            cache.insert(key, forest.clone());

            forest
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_forest_helper(
    cache: &mut HashMap<u64, Vec<Tree>>,
    grammar: &Grammar,
    lexemes: &[Lexeme],
    columns: &[ParserColumn],

    leaves: Vec<Tree>,
    state: &ParserState,
    symbol_index: usize,
    end_column: usize,
) -> Vec<Tree> {
    if symbol_index == usize::MAX {
        if leaves.len() == 1 && matches!(leaves[0], Tree::Node { .. }) {
            return leaves;
        }

        return vec![Tree::Node { kind: (*state.name).clone(), leaves }];
    }

    let mut forest = Vec::new();

    match &state.production.symbols[symbol_index] {
        Symbol::Lexeme(_) => {
            let lexeme = &lexemes[end_column - 1];
            let mut leaves = leaves;
            let mut leaves_extended = vec![Tree::Leaf(lexeme.clone())];
            leaves_extended.append(&mut leaves);

            for tree in build_forest_helper(
                cache,
                grammar,
                lexemes,
                columns,
                leaves_extended,
                state,
                symbol_index.overflowing_sub(1).0,
                state.end_column - 1,
            ) {
                forest.push(tree);
            }
        }
        Symbol::Rule(name) => {
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
                for alternative in build_forest(
                    cache,
                    grammar,
                    lexemes,
                    columns,
                    state_partial,
                ) {
                    let mut leaves_extended = vec![alternative];
                    leaves_extended.append(&mut leaves.clone());

                    for tree in build_forest_helper(
                        cache,
                        grammar,
                        lexemes,
                        columns,
                        leaves_extended,
                        state,
                        symbol_index.overflowing_sub(1).0,
                        state_partial.start_column,
                    ) {
                        forest.push(tree);
                    }
                }
            }
        }
    }

    forest
}

fn satisfies_disambiguation(
    grammar: &Grammar,
    state_partial: &ParserState,
    state: &ParserState,
) -> bool {
    if state_partial.production.symbols.len() == 3
        && state.production.symbols.len() == 3
    {
        if let (Symbol::Rule(name_partial), Symbol::Rule(name)) =
            (&state_partial.production.symbols[1], &state.production.symbols[1])
        {
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
