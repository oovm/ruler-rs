use std::str::FromStr;
use super::*;

use pex::{ParseResult, ParseState, StopBecause};
use pex::helpers::{make_from_str, whitespace};
use crate::{RulerError, RulerResult, Variable};

impl FromStr for Variable {
    type Err = RulerError;

    fn from_str(s: &str) -> RulerResult<Self> {
        let state = ParseState::new(s.trim_end()).skip(whitespace);
        Ok(make_from_str(state, Self::parse)?)
    }
}

impl From<StopBecause> for RulerError {
    fn from(value: StopBecause) -> Self {
        RulerError::syntax_error(value, 0, 0)
    }
}

impl Variable {
    /// Parse new symbol from parse state
    pub fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('?')?;
        let (state, sym) = state.skip(whitespace).match_fn(Symbol::parse)?;
        state.finish(Self::new(sym))
    }
}