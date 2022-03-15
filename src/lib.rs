use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

const START: &str = "Γ";

fn hash(state: &State) -> u64 {
    let mut hasher = DefaultHasher::new();
    state.hash(&mut hasher);
    hasher.finish()
}

#[derive(Clone, PartialEq)]
enum Symbol {
    Char(char),
    Rule(String),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Char(char) => write!(f, "{char:?}"),
            Symbol::Rule(rule) => write!(f, "{rule}"),
        }
    }
}

impl std::hash::Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Symbol::Char(char) => char.hash(state),
            Symbol::Rule(rule) => rule.hash(state),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Production {
    terms: Vec<Symbol>,
}

impl std::hash::Hash for Production {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.terms.hash(state);
    }
}

impl std::fmt::Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.terms
                .iter()
                .map(Symbol::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Production {
    fn rules(&self) -> Vec<String> {
        self.terms
            .iter()
            .filter_map(|symbol| match symbol {
                Symbol::Char(_) => None,
                Symbol::Rule(name) => Some(name.clone()),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct Rule {
    name:        String,
    productions: Vec<Production>,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} := {}",
            self.name,
            self.productions
                .iter()
                .map(Production::to_string)
                .collect::<Vec<String>>()
                .join(" | ")
        )
    }
}

impl std::hash::Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone)]
struct State {
    name:         String,
    production:   Production,
    dot_index:    usize,
    start_column: usize,
    end_column:   usize,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut terms: Vec<String> =
            self.production.terms.iter().map(Symbol::to_string).collect();
        terms.insert(self.dot_index, "•".to_string());

        write!(
            f,
            "{} := {} [{}-{}]",
            self.name,
            terms.join(" "),
            self.start_column,
            self.end_column,
        )
    }
}

impl std::hash::Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dot_index.hash(state);
        self.name.hash(state);
        self.production.hash(state);
        self.start_column.hash(state);
    }
}

impl State {
    fn completed(&self) -> bool {
        self.dot_index >= self.production.terms.len()
    }

    fn next_term(&self) -> Option<Symbol> {
        if self.completed() {
            None
        } else {
            Some(self.production.terms[self.dot_index].clone())
        }
    }
}

struct Column {
    index:  usize,
    token:  char,
    states: Vec<State>,
    unique: HashSet<u64>,
}

impl Column {
    fn add(&mut self, state: State) {
        let mut state = state;
        let digest = hash(&state);

        if !self.unique.contains(&digest) {
            self.unique.insert(digest);
            state.end_column = self.index;
            self.states.push(state);
        }
    }
}

fn predict(column: &mut Column, rule: &Rule) {
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

fn scan(column: &mut Column, state: &State, token: char) {
    if token == column.token {
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
            let term = st.next_term();
            if let Some(Symbol::Rule(name)) = term {
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

pub fn parse(rules: &[Rule], lexemes: &[char]) -> Result<Vec<Forest>, String> {
    let mut columns: Vec<Column> = (0..=lexemes.len())
        .map(|index| {
            if index == 0 {
                Column {
                    index,
                    token: '^',
                    states: vec![],
                    unique: HashSet::new(),
                }
            } else {
                Column {
                    index,
                    token: lexemes[index - 1],
                    states: Vec::new(),
                    unique: HashSet::new(),
                }
            }
        })
        .collect();

    columns[0].states.push(State {
        name:         START.to_string(),
        production:   Production {
            terms: vec![Symbol::Rule(rules[0].name.clone())],
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
                match state.next_term().unwrap() {
                    Symbol::Rule(name) => {
                        let rule = rules
                            .iter()
                            .find(|rule| rule.name == name)
                            .unwrap();
                        predict(&mut columns[column_index], &rule);
                    }
                    Symbol::Char(char) => {
                        if column_index + 1 < columns.len() {
                            scan(&mut columns[column_index + 1], &state, char);
                        }
                    }
                }
            }

            state_index += 1;
            state_len = columns[column_index].states.len();
        }
    }

    println!();
    println!("Columns:");
    for (column_index, column) in columns.iter().enumerate() {
        println!("  {column_index}");
        for state in &column.states {
            println!("    {state}");
        }
    }

    for state in &columns.last().unwrap().states {
        if state.name == START && state.completed() {
            return Ok(build_trees(rules, &columns, state));
        }
    }

    Err(String::new())
}

#[derive(Clone, Debug)]
pub enum Forest {
    Leaf { kind: String },
    Node { kind: String, leaves: Vec<Forest> },
    Nodes { options: Vec<Forest> },
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn recurse(depth: usize, forest: &Forest) -> String {
            match forest {
                Forest::Leaf { kind } => {
                    format!("{}{kind}\n", "  ".repeat(depth + 1),)
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

fn build_trees(
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

    println!("build_trees_helper:");
    println!("  state: {state}");
    println!("  symbol_index: {symbol_index}");
    println!("  end_column: {end_column}");

    let symbol = &state.production.rules()[symbol_index];
    println!("  symbol: {symbol}");
    let mut outputs = Vec::new();

    for st in &columns[end_column].states {
        // name:         String,
        // production:   Production,
        // dot_index:    usize,
        // start_column: usize,
        // end_column:   usize,
        if st.name == state.name
            && st.production == state.production
            && st.dot_index == state.dot_index
            && st.start_column == state.start_column
            && st.end_column == state.end_column
        {
            break;
        }

        if !st.completed() {
            continue;
        }

        if st.name != *symbol {
            continue;
        }

        if symbol_index == 0 && st.start_column != state.start_column {
            continue;
        }
        println!("  loop: {st}");

        let forests = build_trees(rules, columns, st);
        for forest in forests {
            println!("  forest: {forest:?}");
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
        println!("  forests-end")
    }

    return outputs;
}

pub fn test() {
    // S := S + S | T
    // T := 1 | 2 | 3
    let grammar = &[
        Rule {
            name:        "S".to_string(),
            productions: vec![
                Production {
                    terms: vec![
                        Symbol::Rule("S".to_string()),
                        Symbol::Rule("Plus".to_string()),
                        Symbol::Rule("S".to_string()),
                    ],
                },
                Production { terms: vec![Symbol::Rule("T".to_string())] },
            ],
        },
        Rule {
            name:        "T".to_string(),
            productions: vec![
                Production { terms: vec![Symbol::Char('1')] },
                Production { terms: vec![Symbol::Char('2')] },
                Production { terms: vec![Symbol::Char('3')] },
            ],
        },
        Rule {
            name:        "Plus".to_string(),
            productions: vec![Production { terms: vec![Symbol::Char('+')] }],
        },
    ];

    println!("Grammar:");
    for rule in grammar {
        println!("  {rule}");
    }

    let input: Vec<char> = "1+2+3".chars().collect();

    println!();
    println!("input: {input:?}");

    println!();
    println!("Forest:");
    let result = parse(grammar, &input);
    match result {
        Ok(forests) => {
            for forest in forests {
                println!("{forest}");
            }
        }
        Err(_) => println!("{result:?}"),
    }
}
