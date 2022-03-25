// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Associativity;
use crate::grammar::Grammar;
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

pub(crate) struct BuildForest<'a> {
    build_forest_helper: BuildForestHelper<'a>,
}

impl<'a> BuildForest<'a> {
    pub(crate) fn new(state: &'a ParserState) -> BuildForest<'a> {
        BuildForest {
            build_forest_helper: BuildForestHelper::new(
                vec![],
                state,
                state.production.symbols.len().overflowing_sub(1).0,
                state.end_column,
            ),
        }
    }

    pub(crate) fn next(
        &mut self,
        grammar: &'a Grammar,
        lexemes: &'a [Lexeme],
        columns: &'a [ParserColumn],
    ) -> Option<Vec<Tree>> {
        self.build_forest_helper.next(grammar, lexemes, columns)
    }
}

pub(crate) struct BuildForestHelper<'a> {
    leaves:       Vec<Tree>,
    state:        &'a ParserState,
    symbol_index: usize,
    end_column:   usize,
    finished:     bool,
}

impl<'a> BuildForestHelper<'a> {
    pub(crate) fn new(
        leaves: Vec<Tree>,
        state: &'a ParserState,
        symbol_index: usize,
        end_column: usize,
    ) -> BuildForestHelper<'a> {
        BuildForestHelper {
            leaves,
            state,
            symbol_index,
            end_column,
            finished: false,
        }
    }

    pub(crate) fn next(
        &mut self,
        grammar: &'a Grammar,
        lexemes: &'a [Lexeme],
        columns: &'a [ParserColumn],
    ) -> Option<Vec<Tree>> {
        println!(
            "BuildForestHelper {:?} {} {} {}",
            self.leaves.len(),
            self.state,
            self.symbol_index,
            self.end_column
        );
        if self.finished {
            return None;
        }
        self.finished = true;

        if self.symbol_index == usize::MAX {
            if self.leaves.len() == 1
                && matches!(self.leaves[0], Tree::Node { .. })
            {
                return Some(self.leaves.clone());
            }

            return Some(vec![Tree::Node {
                kind:   self.state.name.clone(),
                leaves: self.leaves.clone(),
            }]);
        }

        let mut forest = Vec::new();

        match &self.state.production.symbols[self.symbol_index] {
            Symbol::Lexeme(_) => {
                let lexeme = &lexemes[self.end_column - 1];
                let mut leaves = self.leaves.clone();
                let mut leaves_extended = vec![Tree::Leaf(lexeme.clone())];
                leaves_extended.append(&mut leaves);

                let mut build_forest_helper = BuildForestHelper::new(
                    leaves_extended,
                    self.state,
                    self.symbol_index.overflowing_sub(1).0,
                    self.state.end_column - 1,
                );

                while let Some(mut trees) =
                    build_forest_helper.next(grammar, lexemes, columns)
                {
                    forest.append(&mut trees);
                }
            }
            Symbol::Rule(name) => {
                for state_partial in columns[self.end_column]
                    .states
                    .iter()
                    .take_while(|state_partial| *state_partial != self.state)
                    .filter(|state_partial| {
                        state_partial.name == *name
                            && (self.symbol_index > 0
                                || state_partial.start_column
                                    == self.state.start_column)
                            && satisfies_disambiguation(
                                grammar,
                                state_partial,
                                self.state,
                            )
                    })
                {
                    let mut build_forest = BuildForest::new(state_partial);

                    while let Some(alternatives) =
                        build_forest.next(grammar, lexemes, columns)
                    {
                        for alternative in alternatives {
                            let mut leaves_extended = vec![alternative];
                            leaves_extended.append(&mut self.leaves.clone());

                            let mut build_forest_helper =
                                BuildForestHelper::new(
                                    leaves_extended,
                                    self.state,
                                    self.symbol_index.overflowing_sub(1).0,
                                    state_partial.start_column,
                                );

                            while let Some(mut trees) = build_forest_helper
                                .next(grammar, lexemes, columns)
                            {
                                forest.append(&mut trees);
                            }
                        }
                    }
                }
            }
        }

        Some(forest)
    }
}

// pub(crate) fn build_forest(
//     grammar: &Grammar,
//     lexemes: &[Lexeme],
//     columns: &[ParserColumn],
//     state: &ParserState,
// ) -> Vec<Tree> {
//     build_forest_helper(
//         grammar,
//         lexemes,
//         columns,
//         vec![],
//         state,
//         state.production.symbols.len().overflowing_sub(1).0,
//         state.end_column,
//     )
// }

// fn build_forest_helper(
//     grammar: &Grammar,
//     lexemes: &[Lexeme],
//     columns: &[ParserColumn],

//     leaves: Vec<Tree>,
//     state: &ParserState,
//     symbol_index: usize,
//     end_column: usize,
// ) -> Vec<Tree> {
//     if symbol_index == usize::MAX {
//         if leaves.len() == 1 && matches!(leaves[0], Tree::Node { .. }) {
//             return leaves;
//         }

//         return vec![Tree::Node { kind: state.name.clone(), leaves }];
//     }

//     let mut forest = Vec::new();

//     match &state.production.symbols[symbol_index] {
//         Symbol::Lexeme(_) => {
//             let lexeme = &lexemes[end_column - 1];
//             let mut leaves = leaves;
//             let mut leaves_extended = vec![Tree::Leaf(lexeme.clone())];
//             leaves_extended.append(&mut leaves);

//             for tree in build_forest_helper(
//                 grammar,
//                 lexemes,
//                 columns,
//                 leaves_extended,
//                 state,
//                 symbol_index.overflowing_sub(1).0,
//                 state.end_column - 1,
//             ) {
//                 forest.push(tree);
//             }
//         }
//         Symbol::Rule(name) => {
//             for state_partial in columns[end_column]
//                 .states
//                 .iter()
//                 .take_while(|state_partial| *state_partial != state)
//                 .filter(|state_partial| {
//                     state_partial.name == *name
//                         && (symbol_index > 0
//                             || state_partial.start_column == state.start_column)
//                         && satisfies_disambiguation(
//                             grammar,
//                             state_partial,
//                             state,
//                         )
//                 })
//             {
//                 for alternative in
//                     build_forest(grammar, lexemes, columns, state_partial)
//                 {
//                     let mut leaves_extended = vec![alternative];
//                     leaves_extended.append(&mut leaves.clone());

//                     for tree in build_forest_helper(
//                         grammar,
//                         lexemes,
//                         columns,
//                         leaves_extended,
//                         state,
//                         symbol_index.overflowing_sub(1).0,
//                         state_partial.start_column,
//                     ) {
//                         forest.push(tree);
//                     }
//                 }
//             }
//         }
//     }

//     forest
// }

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
