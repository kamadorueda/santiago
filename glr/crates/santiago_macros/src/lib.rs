use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use santiago_parser::parser_action::ParserAction;
use santiago_sorted_vec::SortedVec;
use santiago_types::symbol::ID_EOF;
use santiago_types::symbol::ID_START;
use syn::parse_macro_input;
use syn::Block;
use syn::Expr;

#[proc_macro]
pub fn build(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);

    let (user_defined_lexer_rules, user_defined_productions): (
        Vec<UserDefinedLexerRule>,
        Vec<UserDefinedProduction>,
    ) = parse_user_input(input);

    if user_defined_productions.is_empty() {
        panic!("Expected one or more productions, but got zero.");
    }

    let lexer_ids: Vec<&str> =
        user_defined_lexer_rules.iter().map(|r| r.id.as_str()).collect();
    let symbols: Vec<SemiSymbol> =
        identify_symbols(&lexer_ids, &user_defined_productions);
    let productions: Vec<SemiProduction> =
        identify_productions(&symbols, &user_defined_productions);
    let items = identify_items(&productions);
    let parser_instructions =
        identify_parser_instructions(&symbols, &productions, &items);

    let mut lexer_ids: Vec<usize> = user_defined_lexer_rules
        .iter()
        .map(|r| {
            symbols
                .iter()
                .position(|s| s.id == r.id)
                .unwrap_or_else(|| panic!("{:?}", r.id.as_str()))
        })
        .collect();
    lexer_ids.push(symbols.len() - 1);
    let lexer_actions: Vec<Option<&Block>> =
        user_defined_lexer_rules.iter().map(|r| r.action.as_ref()).collect();
    let mut lexer_states: Vec<&String> = user_defined_lexer_rules
        .iter()
        .flat_map(|r| &r.states)
        .filter(|state| *state != "DEFAULT")
        .collect();
    let default = "DEFAULT".to_string();
    lexer_states.sort_unstable();
    lexer_states.dedup();
    lexer_states.insert(0, &default);

    let lexer_instructions: Vec<(Vec<usize>, Vec<&str>)> = lexer_states
        .iter()
        .map(|state| {
            user_defined_lexer_rules
                .iter()
                .enumerate()
                .filter(|(_, r)| r.states.contains(state))
                .map(|(index, r)| (index, r.regex.as_str()))
                .unzip()
        })
        .collect();

    let symbols: Vec<_> =
        symbols.iter().map(|SemiSymbol { id, .. }| id).collect();
    let productions: Vec<_> = productions
        .iter()
        .map(|SemiProduction { from, to, .. }| {
            quote! {
                Production {
                    from: #from,
                    to: &[#(#to),*],
                }
            }
        })
        .collect();
    let parser_instructions: Vec<_> = parser_instructions
        .iter()
        .map(|symbols| {
            let symbols = symbols.iter().map(|actions| {
                quote! {
                    &[#(#actions),*]
                }
            });

            quote! {
                &[#(#symbols),*]
            }
        })
        .collect();
    let lexer_actions: Vec<_> = lexer_actions
        .iter()
        .map(|block| {
            block.map_or_else(
                || quote!(|current, matched| { current.take(matched) }),
                |block| quote!(|current, matched| #block),
            )
        })
        .collect();
    let lexer_state_to_index: Vec<_> = lexer_states
        .iter()
        .enumerate()
        .map(|(index, state)| quote!( #state => #index ))
        .collect();
    let lexer_matchers: Vec<_> = lexer_instructions
        .iter()
        .map(|(_, regexes)| {
            let regexes: Vec<String> =
                regexes.iter().map(|regex| format!("({regex})")).collect();
            let regex = regexes.join("|");
            let regex = format!(r"\A(?:{regex})");

            quote!(Regex::new(#regex).unwrap())
        })
        .collect();
    let lexer_instructions: Vec<_> = lexer_instructions
        .iter()
        .enumerate()
        .map(|(index, (ids, _))| {
            quote!(
                LexerInstruction {
                    ids: &[#(#ids),*],
                    matcher: #index,
                }
            )
        })
        .collect();

    let output = TokenStream::from(quote! {
        extern crate santiago;

        use std::any::Any;

        use santiago::lexer::lexeme::Lexeme;
        use santiago::lexer::lexer::Lexer;
        use santiago::lexer::lexer::LexerAction;
        use santiago::lexer::lexer_instruction::LexerInstruction;
        use santiago::parser::parse::Tree;
        use santiago::parser::parser_action::ParserAction::Finish;
        use santiago::parser::parser_action::ParserAction::Reduce;
        use santiago::parser::parser_action::ParserAction::Shift;
        use santiago::parser::parser_instruction::ParserInstruction;
        use santiago::types::production::Production;
        use santiago::vendored::lazy_static::lazy_static;
        use santiago::vendored::regex::Regex;

        pub const SYMBOLS: &'static [&'static str] = &[ #(#symbols),* ];
        pub const PRODUCTIONS: &'static [Production] = &[ #(#productions),* ];
        pub const LEXER_SYMBOL_INDEXES: &'static [usize] = &[ #(#lexer_ids),* ];
        pub const LEXER_ACTIONS: &'static [LexerAction] = &[ #(#lexer_actions),* ];
        pub const LEXER_INSTRUCTIONS: &'static [LexerInstruction] = &[ #(#lexer_instructions),* ];
        pub const PARSER_INSTRUCTIONS: &'static [ParserInstruction] = &[ #(#parser_instructions),* ];

        pub const LEXER: &'static Lexer = &Lexer {
            get_instruction: |state| match state {
                #( #lexer_state_to_index, )*
                _ => unreachable!(),
            },
        };

        lazy_static!(
            static ref LEXER_MATCHERS: &'static [Regex] =
                Vec::from([ #(#lexer_matchers),* ]).leak();
        );

        pub fn debug(value: &dyn Any) -> String {
            santiago::macros::debug(
                SYMBOLS,
                PRODUCTIONS,
                PARSER_INSTRUCTIONS,
                value,
            )
        }

        pub fn lex(input: &str) -> Vec<Lexeme> {
            santiago::lexer::lex::lex(
                LEXER,
                LEXER_ACTIONS,
                LEXER_INSTRUCTIONS,
                &LEXER_MATCHERS,
                LEXER_SYMBOL_INDEXES,
                input,
            )
        }

        pub fn parse(lexemes: &[Lexeme]) -> Result<(), ()> {
            santiago::parser::parse::parse(PRODUCTIONS, PARSER_INSTRUCTIONS, lexemes)
        }
    });

    output
}

struct UserDefinedProduction {
    from: String,
    to: Vec<String>,
}

struct UserDefinedLexerRule {
    states: Vec<String>,
    id: String,
    regex: String,
    action: Option<Block>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SemiSymbol {
    id: String,
    is_terminal: bool,
}

struct SemiProduction {
    from: usize,
    to: Vec<usize>,
}

#[derive(Debug)]
struct Item {
    production_index: usize,
    dot_index: usize,
    states: Vec<usize>,
}

fn identify_parser_instructions(
    symbols: &[SemiSymbol],
    productions: &[SemiProduction],
    items: &[Item],
) -> Vec<Vec<Vec<ParserAction>>> {
    let mut instructions = Vec::new();

    let mut current_state = 0;

    loop {
        let current_state_items: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.states.contains(&current_state))
            .map(|(index, _)| index)
            .collect();

        if current_state_items.is_empty() {
            break;
        }

        let mut table: Vec<Vec<ParserAction>> =
            (0..symbols.len()).map(|_| Vec::new()).collect();

        for item_index in current_state_items {
            let item = &items[item_index];
            let production = &productions[item.production_index];

            if let Some(symbol_index) = production.to.get(item.dot_index) {
                for state in &items[item_index + 1].states {
                    table[*symbol_index]
                        .sorted_insert(ParserAction::Shift(*state));
                }
            } else if !production.to.is_empty() {
                symbols
                    .iter()
                    .enumerate()
                    .filter(|(_, symbol)| symbol.is_terminal)
                    .for_each(|(symbol_index, symbol)| {
                        if production.from == 0 {
                            if symbol.id == ID_EOF {
                                table[symbol_index]
                                    .sorted_insert(ParserAction::Finish);
                            }
                        } else {
                            table[symbol_index].sorted_insert(
                                ParserAction::Reduce(item.production_index),
                            );
                        }
                    });
            }
        }

        instructions.push(table);

        current_state += 1;
    }

    instructions
}

fn identify_items(productions: &[SemiProduction]) -> Vec<Item> {
    let mut items: Vec<Item> = productions
        .iter()
        .enumerate()
        .flat_map(|(production_index, production)| {
            (0..=production.to.len()).map(move |dot_index| {
                Item { production_index, dot_index, states: Vec::new() }
            })
        })
        .collect();

    items[0].states.sorted_insert(0);

    let mut current_state = 0;
    let mut max_state = 0;

    while items.iter().any(|item| item.states.is_empty()) {
        let mut old_expected_symbols;
        let mut expected_symbols = Vec::new();

        while {
            old_expected_symbols = expected_symbols;
            expected_symbols = items
                .iter()
                .filter(|item| item.states.contains(&current_state))
                .filter_map(|item| {
                    productions[item.production_index].to.get(item.dot_index)
                })
                .cloned()
                .collect();
            expected_symbols.sort_unstable();
            expected_symbols.dedup();

            expected_symbols != old_expected_symbols
        } {
            for symbol in &expected_symbols {
                items
                    .iter_mut()
                    .filter(|item| item.dot_index == 0)
                    .filter(|item| {
                        &productions[item.production_index].from == symbol
                    })
                    .for_each(|item| {
                        item.states.sorted_insert(current_state);
                    });
            }
        }

        for symbol in expected_symbols {
            let indexes: Vec<usize> = items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.states.contains(&current_state))
                .filter(|(_, item)| {
                    productions[item.production_index].to.get(item.dot_index)
                        == Some(&symbol)
                })
                .filter(|(index, _)| items[index + 1].states.is_empty())
                .map(|(index, _)| index)
                .collect();

            if !indexes.is_empty() {
                max_state += 1;

                indexes.into_iter().for_each(|index| {
                    items[index + 1].states.sorted_insert(max_state);
                });
            }
        }

        current_state += 1;
    }

    items
}

fn identify_symbols(
    lexer_ids: &[&str],
    user_defined_productions: &[UserDefinedProduction],
) -> Vec<SemiSymbol> {
    let mut symbols = Vec::new();

    let mut non_terminals = Vec::new();
    for user_defined_production in user_defined_productions {
        non_terminals.sorted_insert(user_defined_production.from.as_str());
    }

    for id in lexer_ids {
        symbols.sorted_insert(SemiSymbol {
            is_terminal: true,
            id: id.to_string(),
        });
    }

    for user_defined_production in user_defined_productions {
        for id in []
            .iter()
            .chain([&user_defined_production.from])
            .chain(&user_defined_production.to)
        {
            let id = id.clone();

            symbols.sorted_insert(SemiSymbol {
                is_terminal: !non_terminals.contains(&id.as_str()),
                id,
            });
        }
    }

    symbols
        .insert(0, SemiSymbol { id: ID_START.to_string(), is_terminal: false });
    symbols.push(SemiSymbol { id: ID_EOF.to_string(), is_terminal: true });

    symbols
}

fn identify_productions(
    symbols: &[SemiSymbol],
    user_defined_productions: &[UserDefinedProduction],
) -> Vec<SemiProduction> {
    [].iter()
        .chain(&[UserDefinedProduction {
            from: ID_START.to_string(),
            to: Vec::from([user_defined_productions[0].from.clone()]),
        }])
        .chain(user_defined_productions)
        .map(|UserDefinedProduction { from, to }| {
            SemiProduction {
                from: symbols
                    .iter()
                    .position(|symbol| &symbol.id == from)
                    .unwrap(),
                to: to
                    .iter()
                    .map(|to_id| {
                        symbols
                            .iter()
                            .position(|symbol| &symbol.id == to_id)
                            .unwrap()
                    })
                    .collect(),
            }
        })
        .collect()
}

fn parse_user_input(
    input: Expr,
) -> (Vec<UserDefinedLexerRule>, Vec<UserDefinedProduction>) {
    let mut array = match input {
        Expr::Array(array) => array,
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    [\n",
                    "        # Grammar\n",
                    "        \"foo\" = [], # empty \n",
                    "        \"foo\" = [\"bar\", ...], # or many \n",
                    "        # and more as needed\n",
                    "    ],\n",
                    "    [\n",
                    "        # Lexer\n",
                    "        ...\n",
                    "    ],\n",
                    "But got: {}",
                ),
                other.to_token_stream(),
            );
        },
    };

    if array.elems.len() != 2 {
        panic!(
            concat!(
                "Expected an expression in the form:\n",
                "    [\n",
                "        # Grammar\n",
                "        \"foo\" = [], # empty \n",
                "        \"foo\" = [\"bar\", ...], # or many \n",
                "        # and more as needed\n",
                "    ],\n",
                "    [\n",
                "        # Lexer\n",
                "        ...\n",
                "    ],\n",
                "But got: {}",
            ),
            array.to_token_stream(),
        );
    }

    (
        parse_user_input_lexer_rules(array.elems.pop().unwrap().into_value()),
        parse_user_input_productions(array.elems.pop().unwrap().into_value()),
    )
}

fn parse_user_input_lexer_rules(input: Expr) -> Vec<UserDefinedLexerRule> {
    let mut lexer_rules = Vec::new();

    let array = match input {
        Expr::Array(array) => array,
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    [\n",
                    "        # ...\n",
                    "    ]\n",
                    "But got: {}",
                ),
                other.to_token_stream(),
            );
        },
    };

    for elem in array.elems {
        let mut array = match elem {
            Expr::Array(array) => array.elems.into_iter(),
            other => {
                panic!(
                    concat!(
                        "Expected an expression in the form:\n",
                        "    [\n",
                        "        # ...\n",
                        "    ]\n",
                        "But got: {}",
                    ),
                    other.to_token_stream()
                );
            },
        };

        let states = match array.next().unwrap() {
            Expr::Array(array) => {
                array.elems.into_iter().map(expr_as_str).collect()
            },
            other => {
                panic!(
                    concat!(
                        "Expected an expression in the form:\n",
                        "    [...]\n",
                        "But got: {}",
                    ),
                    other.to_token_stream()
                );
            },
        };

        let id = array.next().map(expr_as_str).unwrap();
        let regex = array.next().map(expr_as_regex).unwrap();
        let action = array.next().map(|expr| {
            match expr {
                Expr::Block(block) => block.block,
                other => {
                    panic!(
                        concat!(
                            "Expected an expression in the form:\n",
                            "    {{\n",
                            "        # anything\n",
                            "    }}\n",
                            "But got: {}",
                        ),
                        other.to_token_stream()
                    )
                },
            }
        });

        lexer_rules.push(UserDefinedLexerRule { states, id, regex, action });
    }

    lexer_rules.sort_by(|a, b| a.id.cmp(&b.id));

    lexer_rules
}

fn parse_user_input_productions(input: Expr) -> Vec<UserDefinedProduction> {
    let mut productions = Vec::new();

    let array = match input {
        Expr::Array(array) => array,
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    [\n",
                    "        \"foo\" = [], # empty \n",
                    "        \"foo\" = [\"bar\", ...], # or many \n",
                    "        # and more as needed\n",
                    "    ]\n",
                    "But got: {}",
                ),
                other.to_token_stream(),
            );
        },
    };

    for elem in array.elems {
        let assign = match elem {
            Expr::Assign(assign) => assign,
            other => {
                panic!(
                    concat!(
                        "Expected an expression in the form:\n",
                        "    \"foo\" = [], # empty \n",
                        "Or:\n",
                        "    \"foo\" = [\"bar\", ...], # many \n",
                        "But got: {}",
                    ),
                    other.to_token_stream(),
                );
            },
        };

        let from = expr_as_str(*assign.left);
        let to = match *assign.right {
            Expr::Array(array) => array,
            other => {
                panic!(
                    concat!(
                        "Expected an expression in the form:\n",
                        "    [], # empty \n",
                        "Or:",
                        "    [\"bar\", ...], # many \n",
                        "But got: {}",
                    ),
                    other.to_token_stream(),
                );
            },
        };
        let to = to.elems.into_iter().map(expr_as_str).collect();

        productions.push(UserDefinedProduction { from, to });
    }

    productions
}

fn expr_as_str(expr: Expr) -> String {
    match expr {
        Expr::Lit(lit) => {
            match lit.lit {
                syn::Lit::Str(str) => str.value(),
                other => {
                    panic!(
                        concat!(
                            "Expected an expression in the form:\n",
                            "    \"foo\"\n",
                            "But got: {}",
                        ),
                        other.into_token_stream(),
                    );
                },
            }
        },
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    \"foo\"\n",
                    "But got: {}",
                ),
                other.into_token_stream(),
            );
        },
    }
}

fn expr_as_regex(expr: Expr) -> String {
    let call = match expr {
        Expr::Call(call) => call,
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    regex(r\"\\d\")\n",
                    "Or:\n",
                    "    literal(\"foo\")\n",
                    "But got: {}",
                ),
                other.into_token_stream(),
            );
        },
    };

    if call.args.len() != 1 {
        panic!(
            concat!(
                "Expected an expression in the form:\n",
                "    regex(r\"\\d\")\n",
                "Or:\n",
                "    literal(\"foo\")\n",
                "But got: {}",
            ),
            call.into_token_stream(),
        )
    }

    let path = match &*call.func {
        Expr::Path(ref path) => path,
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    regex\n",
                    "Or:\n",
                    "    literal\n",
                    "But got: {}",
                ),
                other.into_token_stream(),
            );
        },
    };

    let id = path.path.get_ident().unwrap().to_string();
    let arg = expr_as_str(call.args.into_iter().next().unwrap());

    match id.as_str() {
        "regex" => arg,
        "literal" => regex::escape(&arg),
        other => {
            panic!(
                concat!(
                    "Expected an expression in the form:\n",
                    "    regex\n",
                    "Or:\n",
                    "    literal\n",
                    "But got: {}",
                ),
                other.into_token_stream(),
            )
        },
    }
}
