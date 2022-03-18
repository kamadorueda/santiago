use crate::{
    grammar::{rule::Rule, symbol::Symbol},
    lex::{lexeme::Lexeme, position::Position},
    parse::{column::Column, state::State},
};

#[derive(Clone, Debug, Hash)]
pub enum Forest {
    Leaf { kind: String, position: Position },
    Node { kind: String, leaves: Vec<Forest> },
    Nodes { options: Vec<Forest> },
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn recurse(depth: usize, forest: &Forest) -> String {
            match forest {
                Forest::Leaf { kind, position } => {
                    format!("{}{kind} {position}\n", "  ".repeat(depth + 1),)
                }
                Forest::Node { kind, leaves } => {
                    let mut result = String::new();
                    result += &format!("{}{kind}\n", "  ".repeat(depth));
                    for leaf in leaves {
                        result += &recurse(
                            depth
                                + match leaf {
                                    Forest::Leaf { .. } => 0,
                                    Forest::Node { .. } => 1,
                                    Forest::Nodes { .. } => 1,
                                },
                            leaf,
                        );
                    }

                    result
                }
                Forest::Nodes { options } => {
                    let mut result = String::new();
                    result += &format!("{}nodes\n", "  ".repeat(depth));
                    for option in options {
                        result += &recurse(
                            depth
                                + match option {
                                    Forest::Leaf { .. } => 0,
                                    Forest::Node { .. } => 1,
                                    Forest::Nodes { .. } => 1,
                                },
                            &option,
                        );
                    }

                    result
                }
            }
        }

        write!(f, "{}", &recurse(0, self))
    }
}

pub(crate) fn build_trees(
    rules: &[Rule],
    lexemes: &[Lexeme],
    columns: &Vec<Column>,
    state: &State,
) -> Vec<Forest> {
    return build_trees_helper(
        rules,
        lexemes,
        columns,
        vec![],
        state,
        state.production.terms.len().overflowing_sub(1).0,
        state.end_column,
    );
}

fn build_trees_helper(
    rules: &[Rule],
    lexemes: &[Lexeme],
    columns: &Vec<Column>,

    leaves: Vec<Forest>,
    state: &State,
    symbol_index: usize,
    end_column: usize,
) -> Vec<Forest> {
    if symbol_index == usize::MAX {
        return vec![Forest::Node { kind: state.name.clone(), leaves }];
    }

    let mut forests = Vec::new();
    match &state.production.terms[symbol_index] {
        Symbol::Lexeme(raw) => {
            let mut leaves_extended = vec![Forest::Leaf {
                kind:     raw.clone(),
                position: lexemes[end_column - 1].position.clone(),
            }];
            leaves_extended.append(&mut leaves.clone());

            for node in build_trees_helper(
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

                let alternatives = build_trees(rules, lexemes, columns, st);

                for alternative in alternatives {
                    let mut leaves_extended = vec![alternative];
                    leaves_extended.append(&mut leaves.clone());

                    for node in build_trees_helper(
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

    return forests;
}
