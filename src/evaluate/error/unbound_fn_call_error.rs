use crate::s_expression::SExpressionRef as SXRef;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct UnboundFnCallError {
    fn_name: SXRef,
}

impl UnboundFnCallError {
    pub fn new(fn_name: SXRef) -> UnboundFnCallError {
        UnboundFnCallError {
            fn_name,
        }
    }
}

impl std::error::Error for UnboundFnCallError { }

impl fmt::Display for UnboundFnCallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not call non-function '{}' as a function", self.fn_name)
    }
}
