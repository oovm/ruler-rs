use std::fmt::Debug;
use std::str::FromStr;
use unicode_ident::is_xid_continue;
use ustr::Ustr;


pub struct Symbol {
    intern: Ustr,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Symbol({})", self.intern)
    }
}

pub enum SymbolError {
    Empty,
    Invalid,
}

impl FromStr for Symbol {
    type Err = SymbolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(SymbolError::Empty);
        }
        if !s.chars().next().unwrap().is_xid_continue() {
            return Err(SymbolError::Invalid);
        }
        for c in s.chars() {
            if !c.is_xid_continue() {
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

fn test() {
    let _ = Symbol::new("test");
}