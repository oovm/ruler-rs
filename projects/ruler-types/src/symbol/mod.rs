use std::fmt::{Debug, Display};
use std::str::FromStr;
use ustr::Ustr;
use crate::RulerError;

#[cfg(feature = "parse")]
mod parser;

#[derive(Copy, Clone)]
pub struct Symbol {
    intern: Ustr,
}

impl<T> From<T> for Symbol where T: AsRef<str> {
    fn from(value: T) -> Self {
        Self::new(value.as_ref())
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Symbol({})", self.intern)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.intern)
    }
}


impl Symbol {
    /// Create a new symbol from given name.
    #[track_caller]
    pub fn new(intern: &str) -> Self {
        if cfg!(debug_assertions) && cfg!(feature = "parse") {
            match Self::from_str(intern) {
                Ok(o) => { o }
                Err(e) => panic!("{e}")
            }
        } else {
            Self::new_unchecked(intern)
        }
    }
    pub(crate) fn new_unchecked(intern: &str) -> Self {
        Self {
            intern: Ustr::from(intern),
        }
    }
}

#[test]
fn test() {
    let t1 = Symbol::new("test");
    let t2 = Symbol::new("test2");
    println!("{:?}", t1);
}