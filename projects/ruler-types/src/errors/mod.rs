use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;

/// Result type for Ruler.
pub type RulerResult<T> = Result<T, RulerError>;


/// Error type for Ruler.
#[derive(Debug, Clone)]
pub struct RulerError {
    kind: Box<RuleErrorKind>,
}

/// Error kind for Ruler.
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
            RuleErrorKind::SyntaxError { message, range } => {
                write!(f, "SymbolError: {} at {range:?}", message)
            }
            RuleErrorKind::CustomError { message } => {
                f.write_str(message)
            }
        }
    }
}

impl RulerError {
    /// Create a new syntax error.
    pub fn syntax_error<S>(message: S, start: usize, end: usize) -> Self
        where
            S: ToString
    {
        Self {
            kind: Box::new(RuleErrorKind::SyntaxError {
                message: message.to_string(),
                range: Range {
                    start,
                    end,
                },
            }),
        }
    }
    /// Create a new custom error.
    pub fn custom_error<S>(message: S) -> Self
        where
            S: ToString
    {
        Self {
            kind: Box::new(RuleErrorKind::CustomError {
                message: message.to_string(),
            }),
        }
    }
}