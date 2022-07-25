mod unbound_fn_call_error;
pub use unbound_fn_call_error::UnboundFnCallError;

use std::fmt;
use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnboundFnCall(UnboundFnCallError),
}

impl Error {
    pub fn unbound_fn_call_error(err: UnboundFnCallError) -> Self {
        Self::UnboundFnCall(err)
    }
}

impl std::error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnboundFnCall(x) => write!(f, "{}", x),
        }
    }
}

impl From<UnboundFnCallError> for Error {
    fn from(err: UnboundFnCallError) -> Self {
        Self::unbound_fn_call_error(err)
    }
}
