use std::fmt::{Debug, Display, Formatter};
use crate::Symbol;

#[cfg(feature = "parse")]
mod parser;

#[derive(Copy, Clone)]
pub struct Variable {
    symbol: Symbol,
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable({})", self.symbol)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.symbol)
    }
}

impl Variable {
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
        }
    }
}