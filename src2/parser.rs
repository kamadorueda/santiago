use crate::{
    forest::build_forest,
    lexeme::Lexeme,
    parse_state::ParseState,
    rule::Rule,
    symbol::Symbol,
};
use linked_hash_set::LinkedHashSet;

pub fn parse(grammar: &[Rule], lexemes: &[Lexeme]) -> Result<(), ()> {
    let mut states: Vec<LinkedHashSet<ParseState>> =
        (0..=lexemes.len()).map(|_| LinkedHashSet::new()).collect();

    states[0].insert(ParseState {
        rule:             grammar[0].clone(),
        dot_index:        0,
        end_lexeme_index: 0,
    });

    for lexeme_index in 0..=lexemes.len() {
        let mut lexeme_state_index = 0;
        let mut lexeme_state_len = states[lexeme_index].len();

        while lexeme_state_index < lexeme_state_len {
            let state = states[lexeme_index]
                .iter()
                .nth(lexeme_state_index)
                .unwrap()
                .clone();

            if state.completed() {
                complete(&mut states, &state, lexeme_index);
            } else if matches!(
                state.rule.to[state.dot_index],
                Symbol::NonTerminal(_)
            ) {
                predict(&mut states, &state, lexeme_index, grammar);
            } else {
                scan(&mut states, &state, lexemes, lexeme_index);
            }

            lexeme_state_index += 1;
            lexeme_state_len = states[lexeme_index].len();
        }
    }

    for (index, state) in states.iter().enumerate() {
        eprintln!("{index}");
        for s in state {
            eprintln!("  {s}");
        }
    }

    let forest = build_forest(grammar, &states);
    println!("Forest");
    println!("{:?}", forest);
    // for state in states.last().unwrap() {
    //     if state.rule == grammar[0] && state.completed() {
    //     }
    // }

    Err(())
}

fn complete(
    states: &mut Vec<LinkedHashSet<ParseState>>,
    state: &ParseState,
    lexeme_index: usize,
) {
    let matches: Vec<ParseState> = states[state.end_lexeme_index]
        .iter()
        .filter(|match_| {
            match_.dot_index < match_.rule.to.len()
                && match_.rule.to[match_.dot_index] == state.rule.from
        })
        .cloned()
        .collect();

    for match_ in matches {
        states[lexeme_index].insert_if_absent(match_.advance());
    }
}

fn predict(
    states: &mut Vec<LinkedHashSet<ParseState>>,
    state: &ParseState,
    lexeme_index: usize,
    grammar: &[Rule],
) {
    for rule in grammar {
        if rule.from == state.rule.to[state.dot_index] {
            states[lexeme_index].insert_if_absent(ParseState {
                rule:             rule.clone(),
                dot_index:        0,
                end_lexeme_index: lexeme_index,
            });
        }
    }
}

fn scan(
    states: &mut Vec<LinkedHashSet<ParseState>>,
    state: &ParseState,
    lexemes: &[Lexeme],
    lexeme_index: usize,
) {
    match &state.rule.to[state.dot_index] {
        Symbol::Terminal(lexeme_kind) => {
            if lexeme_index < lexemes.len()
                && lexemes[lexeme_index].kind == *lexeme_kind
            {
                states[lexeme_index + 1].insert_if_absent(ParseState {
                    dot_index: state.dot_index + 1,
                    end_lexeme_index: lexeme_index + 1,
                    ..state.clone()
                });
            };
        }
        _ => unreachable!(),
    }
}
