// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-only

use crate::grammar::Associativity;
use crate::grammar::Grammar;
use crate::grammar::Production;
use crate::grammar::ProductionKind;
use crate::lexer::Lexeme;
use crate::parser::ParserColumn;
use crate::parser::ParserState;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;

/// Representation of an AST.
pub enum Tree<Value> {
    /// Leaf nodes of the tree, containing a [Lexeme].
    Leaf(Lexeme),
    /// Group of many [Tree::Leaf].
    Node {
        /// Name of the [GrammarRule](crate::grammar::GrammarRule) that produced this node.
        rule_name:  Rc<String>,
        /// Reference to the [Production] that produced this node.
        production: Rc<Production<Value>>,
        /// Children of this Node.
        leaves:     Vec<Rc<Tree<Value>>>,
    },
}

impl<Value> std::fmt::Debug for Tree<Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<Value> std::fmt::Display for Tree<Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = LinkedList::new();

        for tree in self.traverse_in_pre_order() {
            let indent = "  ".repeat(stack.len());

            match tree {
                Tree::Leaf(lexeme) => {
                    writeln!(f, "{indent}{lexeme}")?;
                }
                Tree::Node { rule_name, production, .. } => {
                    writeln!(f, "{indent}{rule_name} := {production}")?;
                    stack.push_back(production.symbols.len());
                }
            }

            while let Some(remaining) = stack.back_mut() {
                if *remaining == 0 {
                    stack.pop_back();
                } else {
                    *remaining -= 1;
                    break;
                };
            }
        }

        Ok(())
    }
}

impl<Value> Tree<Value> {
    /// Traverse the tree in post-order.
    ///
    /// - Recursively traverse the current node's Nth subtree.
    /// - Recursively traverse the current node's (N-1)th subtree.
    /// - ...
    /// - Visit the current node.
    pub fn traverse_in_post_order(&self) -> LinkedList<&Tree<Value>> {
        let mut todo: LinkedList<&Tree<Value>> = LinkedList::new();
        let mut ordered: LinkedList<&Tree<Value>> = LinkedList::new();

        todo.push_back(self);
        ordered.push_front(self);
        while !todo.is_empty() {
            if let Some(Tree::Node { leaves, .. }) = todo.pop_front() {
                for tree in leaves.iter().rev() {
                    ordered.push_front(tree);
                    todo.push_back(tree);
                }
            }
        }

        ordered
    }

    /// Traverse the tree in pre-order.
    ///
    /// - Visit the current node.
    /// - Recursively traverse the current node's 0 subtree.
    /// - Recursively traverse the current node's 1 subtree.
    /// - ...
    pub fn traverse_in_pre_order(&self) -> LinkedList<&Tree<Value>> {
        let mut todo: LinkedList<&Tree<Value>> = LinkedList::new();
        let mut ordered: LinkedList<&Tree<Value>> = LinkedList::new();

        todo.push_front(self);
        while !todo.is_empty() {
            ordered.push_back(todo.pop_front().unwrap());
            if let Some(Tree::Node { leaves, .. }) = ordered.back() {
                for tree in leaves.iter().rev() {
                    todo.push_front(tree);
                }
            }
        }

        ordered
    }
}

pub(crate) fn build<Value>(
    grammar: &Grammar<Value>,
    lexemes: &[Lexeme],
    columns: &[ParserColumn<Value>],
    state: &ParserState<Value>,
) -> Vec<Rc<Tree<Value>>> {
    let mut cache: HashMap<u64, Rc<Vec<Rc<Tree<Value>>>>> = HashMap::new();

    for column in columns.iter() {
        for state_partial in &column.states {
            build_forest(&mut cache, grammar, lexemes, columns, state_partial);
        }
    }

    (*cache.remove(&state.hash_me()).unwrap()).clone()
}

fn build_forest<Value>(
    cache: &mut HashMap<u64, Rc<Vec<Rc<Tree<Value>>>>>,
    grammar: &Grammar<Value>,
    lexemes: &[Lexeme],
    columns: &[ParserColumn<Value>],
    state: &ParserState<Value>,
) -> Rc<Vec<Rc<Tree<Value>>>> {
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
fn build_forest_helper<Value>(
    cache: &mut HashMap<u64, Rc<Vec<Rc<Tree<Value>>>>>,
    grammar: &Grammar<Value>,
    lexemes: &[Lexeme],
    columns: &[ParserColumn<Value>],

    leaves: Vec<Rc<Tree<Value>>>,
    state: &ParserState<Value>,
    symbol_index: usize,
    end_column: usize,
) -> Vec<Rc<Tree<Value>>> {
    if symbol_index == usize::MAX {
        if leaves.len() == 1 && matches!(*leaves[0], Tree::Node { .. }) {
            return leaves;
        }

        return vec![Rc::new(Tree::Node {
            rule_name: state.rule_name.clone(),
            production: state.production.clone(),
            leaves,
        })];
    }

    match &state.production.kind {
        ProductionKind::Lexemes => {
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
        ProductionKind::Rules => {
            let rule_name = &state.production.symbols[symbol_index];
            let mut forest = Vec::new();

            for state_partial in columns[end_column]
                .states
                .iter()
                .take_while(|state_partial| *state_partial != state)
                .filter(|state_partial| {
                    *state_partial.rule_name == *rule_name
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

fn satisfies_disambiguation<Value>(
    grammar: &Grammar<Value>,
    state_partial: &ParserState<Value>,
    state: &ParserState<Value>,
) -> bool {
    if let (Some(partial_index), Some(index)) = (
        get_disambiguation(grammar, state_partial),
        get_disambiguation(grammar, state),
    ) {
        if let (ProductionKind::Rules, ProductionKind::Rules) =
            (&state_partial.production.kind, &state.production.kind)
        {
            let name_partial = &state_partial.production.symbols[partial_index];
            let name = &state.production.symbols[index];

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
                            && matches!(
                                disambiguation_partial.associativity,
                                Associativity::Left
                            )
                        {
                            return false;
                        }

                        if state_partial.start_column == state.start_column
                            && matches!(
                                disambiguation.associativity,
                                Associativity::Right
                            )
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

fn get_disambiguation<Value>(
    grammar: &Grammar<Value>,
    state: &ParserState<Value>,
) -> Option<usize> {
    if let ProductionKind::Rules = state.production.kind {
        let mut disambiguations =
            state.production.symbols.iter().enumerate().filter_map(
                |(index, symbol)| {
                    if let Some(rule) = grammar.rules.get(symbol) {
                        if rule.disambiguation.is_some() {
                            return Some(index);
                        }
                    }

                    None
                },
            );

        disambiguations.next()
    } else {
        None
    }
}
