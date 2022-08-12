use crate::parser_action::ParserAction;

pub type ParserInstruction = &'static [&'static [ParserAction]];
