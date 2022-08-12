use santiago_lexer::lexeme::Lexeme;
use santiago_types::production::Production;

use crate::parser_action::ParserAction;
use crate::parser_instruction::ParserInstruction;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Tree {
    Node { symbol_index: usize, children: Box<[usize]> },
    Lexeme(usize),
}

#[derive(Debug)]
struct GraphLikeStack<T> {
    vertices: Vec<T>,
    predecessors: Vec<Vec<usize>>,
}

impl<LabelType> GraphLikeStack<LabelType> {
    fn new() -> Self {
        Self { vertices: Vec::new(), predecessors: Vec::new() }
    }

    fn add_vertex(&mut self, data: LabelType) -> usize {
        let vertex_index = self.vertices.len();

        self.vertices.push(data);
        self.predecessors.push(Vec::new());

        vertex_index
    }

    fn add_predecessor(
        &mut self,
        from_vertex_index: usize,
        to_vertex_index: usize,
    ) {
        self.predecessors[from_vertex_index].push(to_vertex_index);
    }

    fn label_of(&self, vertex_index: usize) -> &LabelType {
        &self.vertices[vertex_index]
    }

    fn get_predecessors(&self, vertex_index: usize) -> &[usize] {
        &self.predecessors[vertex_index]
    }

    fn get_predecessors_at_distance(
        &self,
        vertex_index: usize,
        distance: usize,
    ) -> Vec<usize> {
        let mut vertex_indexes = Vec::from([vertex_index]);

        for _ in 0..distance {
            for vertex_index in vertex_indexes.split_off(0) {
                vertex_indexes.extend(self.get_predecessors(vertex_index));
                vertex_indexes.sort_unstable();
                vertex_indexes.dedup();
            }
        }

        vertex_indexes
    }
}

#[test]
fn graph_like_stack() {
    let mut g = GraphLikeStack::new();

    // 0 - 1 - 2
    //   \   \
    //    \    3
    // 4 - 5 - 6
    for data in ["a", "b", "c", "d", "e", "f", "g"] {
        g.add_vertex(data);
    }
    for (to, from) in [(0, 1), (0, 5), (1, 2), (1, 3), (4, 5), (5, 6)] {
        g.add_predecessor(from, to);
    }

    assert_eq!(g.get_predecessors_at_distance(3, 1), &[1]);
    assert_eq!(g.get_predecessors_at_distance(3, 2), &[0]);
    assert_eq!(g.get_predecessors_at_distance(6, 1), &[5]);
    assert_eq!(g.get_predecessors_at_distance(6, 2), &[0, 4]);
}

pub fn parse(
    productions: &'static [Production],
    parser_instructions: &'static [ParserInstruction],
    lexemes: &[Lexeme],
) -> Result<(), ()> {
    let lexemes_len = lexemes.len();

    let mut accepted = false;

    // state -> symbol_index
    let mut gamma: GraphLikeStack<usize> = GraphLikeStack::new();
    let mut created_vertex_indexes: Vec<Vec<usize>> =
        vec![Vec::new(); lexemes_len];

    created_vertex_indexes[0].push(gamma.add_vertex(0));

    for lexeme_index in 0..lexemes_len {
        println!("lexeme_index={lexeme_index}");
        let mut active_vertex_indexes =
            created_vertex_indexes[lexeme_index].clone();

        // Q
        let mut pending_to_shift: Vec<(usize, usize)> = Vec::new();
        // R
        let mut pending_to_reduce: Vec<(usize, usize, usize)> = Vec::new();

        loop {
            println!("loop");
            println!("  created_vertex_indexes={created_vertex_indexes:?}");
            println!("  active_vertex_indexes={active_vertex_indexes:?}");
            println!("  pending_to_shift={pending_to_shift:?}");
            println!("  pending_to_reduce={pending_to_reduce:?}");
            println!("  gamma={gamma:?}");

            if let Some(vertex_index) = active_vertex_indexes.pop() {
                let state: usize = *gamma.label_of(vertex_index);
                let symbol_index: usize = lexemes[lexeme_index].symbol_index;

                // ACTOR
                parser_instructions[state][symbol_index].into_iter().for_each(
                    |parser_action| {
                        match parser_action {
                            ParserAction::Finish => {
                                accepted = true;
                            },
                            ParserAction::Shift(next_state) => {
                                pending_to_shift
                                    .push((vertex_index, *next_state));
                            },
                            ParserAction::Reduce(production_index) => {
                                gamma
                                    .get_predecessors(vertex_index)
                                    .into_iter()
                                    .for_each(|vertex_index_predecessor| {
                                        pending_to_reduce.push((
                                            vertex_index,
                                            *vertex_index_predecessor,
                                            *production_index,
                                        ));
                                    });
                            },
                        }
                    },
                );
            } else if let Some((
                vertex_index,
                vertex_index_predecessor,
                production_index,
            )) = pending_to_reduce.pop()
            {
                let production: &Production = &productions[production_index];
                let production_from: usize = production.from;
                let production_len: usize = production.to.len();

                // REDUCER
                for w_vertex_index in gamma.get_predecessors_at_distance(
                    vertex_index_predecessor,
                    2 * production_len - 1,
                ) {
                    let w_state = *gamma.label_of(w_vertex_index);

                    for parser_action in
                        parser_instructions[w_state][production_from]
                    {
                        let next_state = match parser_action {
                            ParserAction::Shift(next_state) => *next_state,
                            _ => unreachable!(),
                        };

                        if let Some(u_vertex_index) = created_vertex_indexes
                            [lexeme_index]
                            .iter()
                            .find(|created_vertex_index| {
                                *gamma.label_of(**created_vertex_index)
                                    == next_state
                            })
                            .map(|created_vertex_index| *created_vertex_index)
                        {
                            if !gamma
                                .get_predecessors_at_distance(u_vertex_index, 2)
                                .iter()
                                .any(|u_vertex_index_predecessor| {
                                    *u_vertex_index_predecessor
                                        == w_vertex_index
                                })
                            {
                                let z_vertex_index =
                                    gamma.add_vertex(production_from);
                                gamma.add_predecessor(
                                    u_vertex_index,
                                    z_vertex_index,
                                );
                                gamma.add_predecessor(
                                    z_vertex_index,
                                    w_vertex_index,
                                );
                            }
                        } else {
                            let z_vertex_index =
                                gamma.add_vertex(production_from);
                            let u_vertex_index = gamma.add_vertex(next_state);
                            gamma.add_predecessor(
                                u_vertex_index,
                                z_vertex_index,
                            );
                            gamma.add_predecessor(
                                z_vertex_index,
                                w_vertex_index,
                            );

                            created_vertex_indexes[lexeme_index]
                                .push(u_vertex_index);
                            active_vertex_indexes.push(u_vertex_index);
                        }
                    }
                }
            } else {
                break;
            }
        }

        // SHIFTER

        println!("active_vertex_indexes={active_vertex_indexes:?}");
        println!("pending_to_shift={pending_to_shift:?}");
        println!("pending_to_reduce={pending_to_reduce:?}");
        println!("gamma={gamma:?}");
        for s in {
            let mut states: Vec<usize> =
                pending_to_shift.iter().map(|(_, s)| *s).collect();
            states.sort_unstable();
            states.dedup();
            states
        } {
            println!("  s={s}");
            let w_vertex_index = gamma.add_vertex(s);
            created_vertex_indexes[lexeme_index + 1].push(w_vertex_index);

            for (vertex_index, _) in
                pending_to_shift.iter().filter(|(_, s2)| *s2 == s)
            {
                let x_vertex_index =
                    gamma.add_vertex(lexemes[lexeme_index].symbol_index);

                gamma.add_predecessor(w_vertex_index, x_vertex_index);
                gamma.add_predecessor(x_vertex_index, *vertex_index);
            }
        }
        println!("gamma={gamma:?}");
    }

    println!("accepted={accepted}");
    return Ok(());

    // let mut forest: Vec<Tree> = (0..lexemes_len).map(Tree::Lexeme).collect();

    // let mut inputs: Vec<usize> = (0..lexemes_len).rev().collect();
    // let mut outputs: Vec<usize> = Vec::new();

    // let mut states: Vec<usize> = Vec::from([0]);

    // loop {
    //     let state = *states.last().unwrap();
    //     let input = *inputs.last().unwrap();

    //     let symbol_index = match &forest[input] {
    //         Tree::Lexeme(lexeme_index) =>
    // lexemes[*lexeme_index].symbol_index,         Tree::Node {
    // symbol_index, .. } => *symbol_index,     };

    //     let options = parser_instructions[state][symbol_index];

    //     println!("---");
    //     println!("inputs = {inputs:?}");
    //     println!("outputs = {outputs:?}");
    //     println!("states = {states:?}");
    //     println!("symbol_index = {symbol_index:?}");
    //     println!("options = {options:?}");

    //     if options.is_empty() {
    //         return Err(());
    //     }

    //     if options.len() >= 2 {
    //         panic!("Multiple options for automaton to follow");
    //     }

    //     let option = &options[0];

    //     match option {
    //         ParserAction::Finish => {
    //             if states.len() == 2 {
    //                 break;
    //             } else {
    //                 return Err(());
    //             }
    //         },
    //         ParserAction::Shift(next_state) => {
    //             outputs.push(inputs.pop().unwrap());
    //             states.push(*next_state);
    //         },
    //         ParserAction::Reduce(production_index) => {
    //             let production = &productions[*production_index];
    //             let production_to_len = production.to.len();

    //             inputs.push(
    //                 forest.sorted_insert(Tree::Node {
    //                     symbol_index: production.from,
    //                     children: outputs
    //                         .split_off(outputs.len() - production_to_len)
    //                         .into_boxed_slice(),
    //                 }),
    //             );

    //             states.truncate(states.len() - production_to_len);
    //         },
    //     }
    // }

    // println!("forest = {forest:?}");
    // println!("outputs = {outputs:?}");

    // Ok(())
}
