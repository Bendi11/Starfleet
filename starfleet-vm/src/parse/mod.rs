#![doc = concat!("Module defining a parser for the arc programming language, whose grammar is below in EBNF notation\n\n", include_str!("../../../doc/arc/grammar.md"))]

use self::lex::{CodeLoc, Lexer};
use thiserror::Error;

mod lex; 

/// The structure that parses a stream of tokens from a lexer into an abstract syntax tree
#[derive(Debug)]
pub struct Parser<'src> {
    /// The token stream from the original file
    toks: Lexer<'src>,
}

pub type ParseRes<T> = Result<T, ParseErr>;

impl<'src> Parser<'src> {

    /// Create a new `Parser` from the given source string
    pub fn new(source: &'src str) -> Self {
        Self {
            toks: Lexer::new(source)
        }
    }


    fn typename(&mut self) -> ParseRes<>
}

/// All errors that can occur when parsing a stream of tokens to an abstract syntax tree
#[derive(Clone, Debug, Error)]
pub enum ParseErr {
    #[error("[{}]: Bad typename: {}", .0, .1)]
    BadType(CodeLoc, String),


}