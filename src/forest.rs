use crate::{column::Column, rule::Rule, state::State};

#[derive(Clone, Debug, Hash)]
pub enum Forest {
    Leaf { kind: String },
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
    columns: &Vec<Column>,
    state: &State,
) -> Vec<Forest> {
    return build_trees_helper(
        rules,
        columns,
        vec![],
        state,
        state.production.rules().len().overflowing_sub(1).0,
        state.end_column,
    );
}

fn build_trees_helper(
    rules: &[Rule],
    columns: &Vec<Column>,

    leaves: Vec<Forest>,
    state: &State,
    symbol_index: usize,
    end_column: usize,
) -> Vec<Forest> {
    if symbol_index == usize::MAX {
        return vec![Forest::Node { kind: state.name.clone(), leaves }];
    }

    // println!("build_trees_helper:");
    // println!("  state: {state}");
    // println!("  symbol_index: {symbol_index}");
    // println!("  end_column: {end_column}");

    let symbol = &state.production.rules()[symbol_index];
    // println!("  symbol: {symbol}");
    let mut outputs = Vec::new();

    for st in &columns[end_column].states {
        // name:         String,
        // production:   Production,
        // dot_index:    usize,
        // start_column: usize,
        // end_column:   usize,
        if st == state {
            break;
        }

        if !st.completed()
            || st.name != *symbol
            || (symbol_index == 0 && st.start_column != state.start_column)
        {
            continue;
        }
        // println!("  loop: {st}");

        let forests = build_trees(rules, columns, st);
        if forests.len() >= 2 {
            for forest in &forests {
                println!("  forest: {forest}");
            }
        }
        for forest in forests {
            // println!("  forest: {forest:?}");
            let mut x = vec![forest];
            x.append(&mut leaves.clone());

            let trees = build_trees_helper(
                rules,
                columns,
                x,
                state,
                symbol_index.overflowing_sub(1).0,
                st.start_column,
            );

            for node in trees {
                outputs.push(node);
            }
        }
        // println!("  forests-end")
    }

    return outputs;
}
