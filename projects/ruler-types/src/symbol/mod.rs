use std::fmt::{Debug, Display};
use std::str::FromStr;
use ustr::Ustr;
use crate::RulerError;


pub struct Symbol {
    intern: Ustr,
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


impl FromStr for Symbol {
    type Err = RulerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) => {
                if is_xid_start(c) {
                    // ok
                } else {
                    return Err(RulerError::Invalid);
                }
            }
            _ => return Err(RulerError::syntax_error("Empty symbol",0, 1)),
        }
        for c in s.chars() {
            if !is_xid_continue(c) {
                return Err(SymbolError::Invalid);
            }
        }
        Ok(Self {
            intern: Ustr::from(s),
        })
    }
}

impl Symbol {
    pub fn new(intern: &str) -> Self {
        if cfg!(debug_assertions) {
            Self::from_str(intern).unwrap()
        } else {
            Self {
                intern: Ustr::from(intern),
            }
        }
    }
}

#[test]
fn test() {
    let t1 = Symbol::new("test");
    let t2 = Symbol::new("test2");
    println!("{:?}", t1);
}