// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::GrammarRule;
use crate::grammar::Symbol;
use crate::lexer::Lexeme;

use crate::parser::ParserColumn;
use crate::parser::ParserState;

/// Representation of an AST
#[derive(Clone, Debug, Hash)]
pub enum Tree {
    /// Leaf nodes of the tree, containing a [Lexeme].
    Leaf(Lexeme),
    /// Group of many [Tree::Leaf].
    Node {
        /// Name of the [GrammarRule] that produced this node.
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

pub(crate) fn build_forest(
    rules: &[GrammarRule],
    lexemes: &[Lexeme],
    columns: &[ParserColumn],
    state: &ParserState,
) -> Vec<Tree> {
    build_forest_helper(
        rules,
        lexemes,
        columns,
        vec![],
        state,
        state.production.symbols.len().overflowing_sub(1).0,
        state.end_column,
    )
}

fn build_forest_helper(
    rules: &[GrammarRule],
    lexemes: &[Lexeme],
    columns: &[ParserColumn],

    leaves: Vec<Tree>,
    state: &ParserState,
    symbol_index: usize,
    end_column: usize,
) -> Vec<Tree> {
    if symbol_index == usize::MAX {
        return vec![Tree::Node { kind: state.name.clone(), leaves }];
    }

    let mut forests = Vec::new();
    match &state.production.symbols[symbol_index] {
        Symbol::Lexeme(_) => {
            let lexeme = &lexemes[end_column - 1];
            let mut leaves = leaves;
            let mut leaves_extended = vec![Tree::Leaf(lexeme.clone())];
            leaves_extended.append(&mut leaves);

            for node in build_forest_helper(
                rules,
                lexemes,
                columns,
                leaves_extended,
                state,
                symbol_index.overflowing_sub(1).0,
                state.end_column - 1,
            ) {
                forests.push(node);
            }
        }
        Symbol::Rule(name) => {
            for st in &columns[end_column].states {
                if st == state {
                    break;
                }

                if st.name != *name
                    || (symbol_index == 0
                        && st.start_column != state.start_column)
                {
                    continue;
                }

                let alternatives = build_forest(rules, lexemes, columns, st);

                for alternative in alternatives {
                    let mut leaves_extended = vec![alternative];
                    leaves_extended.append(&mut leaves.clone());

                    for node in build_forest_helper(
                        rules,
                        lexemes,
                        columns,
                        leaves_extended,
                        state,
                        symbol_index.overflowing_sub(1).0,
                        st.start_column,
                    ) {
                        forests.push(node);
                    }
                }
            }
        }
    }

    forests
}
