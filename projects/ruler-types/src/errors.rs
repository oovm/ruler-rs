use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub type RulerResult<T> = std::result::Result<T, RulerError>;


#[derive(Debug, Clone)]
pub struct RulerError {
    kind: Box<RuleErrorKind>,
}

#[derive(Debug, Clone)]
pub enum RuleErrorKind {
    SyntaxError {
        message: String,
        range: Range<usize>,
    },
    CustomError {
        message: String,
    },
}

impl Error for RulerError {}

impl Display for RulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for RuleErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleErrorKind::SyntaxError { message, range: offset } => {
                write!(f, "SymbolError({offset}): {}", message)
            }
            RuleErrorKind::CustomError { message } => {
                f.write_str(message)
            }
        }
    }
}

impl RulerError {
    pub fn syntax_error<S>(message: S, start: usize, end: usize) -> Self
        where
            S: ToString
    {
        Self {
            kind: Box::new(RuleErrorKind::SyntaxError {
                message: message.into(),
                range: start,
            }),
        }
    }
    pub fn custom_error<S>(message: S) -> Self
        where
            S: ToString
    {
        Self {
            kind: Box::new(RuleErrorKind::CustomError {
                message: message.into(),
            }),
        }
    }
}