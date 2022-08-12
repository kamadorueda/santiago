pub mod lexer {
    pub use santiago_lexer::*;
}

pub mod macros {
    use std::any::Any;

    use santiago_lexer::lexeme::Lexeme;
    pub use santiago_macros::*;
    use santiago_parser::parser_action::ParserAction;
    use santiago_parser::parser_instruction::ParserInstruction;
    use santiago_types::production::Production;

    pub fn debug(
        symbols: &[&str],
        productions: &[Production],
        parser_instructions: &[ParserInstruction],
        value: &dyn Any,
    ) -> String {
        if let Some(symbols) = value.downcast_ref::<&[&str]>() {
            symbols
                .iter()
                .enumerate()
                .map(|(index, symbol)| format!("{index} | {symbol:?}\n"))
                .collect()
        } else if let Some(productions) = value.downcast_ref::<&[Production]>()
        {
            productions
                .iter()
                .enumerate()
                .map(|(index, production)| {
                    let from = symbols[production.from];
                    let to: String = production
                        .to
                        .iter()
                        .map(|symbol_index| symbols[*symbol_index])
                        .map(|symbol| format!("{symbol:?}"))
                        .collect::<Vec<String>>()
                        .join(" ");

                    format!("{index} | {from:?} -> {to}\n")
                })
                .collect()
        } else if let Some(parser_instructions) =
            value.downcast_ref::<&[ParserInstruction]>()
        {
            parser_instructions
                .iter()
                .enumerate()
                .flat_map(|(state, instruction)| {
                    instruction[1..]
                        .iter()
                        .zip(&symbols[1..])
                        .filter(|(parser_actions, _)| {
                            !parser_actions.is_empty()
                        })
                        .map(move |(parser_actions, symbol)| {
                            let actions = parser_actions
                                .iter()
                                .map(|parser_action| {
                                    debug(
                                        symbols,
                                        productions,
                                        parser_instructions,
                                        parser_action,
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join(" or ");

                            (state, actions, symbol)
                        })
                })
                .map(|(state, actions, symbol)| {
                    format!("{state} | {symbol:?} -> {actions}\n")
                })
                .collect()
        } else if let Some(lexeme) = value.downcast_ref::<Lexeme>() {
            let Lexeme { symbol_index, content, .. } = lexeme;

            let symbol = symbols[*symbol_index];

            if content.is_empty() {
                format!("{symbol:?}")
            } else if symbol == content {
                format!("{content:?}")
            } else {
                format!("{symbol:?}({content:?})")
            }
        } else if let Some(lexemes) = value.downcast_ref::<Vec<Lexeme>>() {
            lexemes
                .iter()
                .map(|l| debug(symbols, productions, parser_instructions, l))
                .enumerate()
                .map(|(index, lexeme)| format!("{index} | {lexeme}\n"))
                .collect()
        } else if let Some(parser_action) = value.downcast_ref::<ParserAction>()
        {
            format!("{parser_action:?}")
        } else {
            format!("{:?}", value.type_id())
        }
    }
}

pub mod parser {
    pub use santiago_parser::*;
}

pub mod types {
    pub use santiago_types::*;
}

pub mod vendored {
    pub use lazy_static;
    pub use regex;
}
