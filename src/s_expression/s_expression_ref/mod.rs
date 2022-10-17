mod iter;
pub use iter::*;

mod maybe_rc;
pub use maybe_rc::MaybeRc;

#[cfg(test)]
mod test;

use super::*;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct SExpressionRef(pub MaybeRc<SExpression>);

impl Deref for SExpressionRef {
    type Target = SExpression;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl SExpressionRef {
    pub fn new(sx: SExpression) -> Self {
        match sx {
            SExpression::Function(_) => Self(MaybeRc::rc(sx)),
            _ => Self(MaybeRc::owned(sx)),
        }
    }

    pub fn clone(sx: &Self) -> Self {
        Self(MaybeRc::clone(&sx.0))
    }

    pub fn cons_cell(c: ConsCell) -> Self {
        Self::new(SExpression::ConsCell(c))
    }

    pub fn function(function: Function) -> Self {
        Self::new(SExpression::Function(function))
    }

    pub fn iter(&self) -> ListIter {
        ListIter {
            current: Self::clone(self),
        }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn r#macro(m: Macro) -> Self {
        Self::new(SExpression::Macro(m))
    }

    pub fn number(n: isize) -> Self {
        Self::new(SExpression::Number(n))
    }

    pub fn nil() -> Self {
        Self::new(SExpression::Nil)
    }

    pub fn quote(v: Self) -> Self {
        Self::new(SExpression::Quote(v))
    }

    pub fn string(s: String) -> Self {
        Self::new(SExpression::String(s))
    }

    pub fn symbol(s: String) -> Self {
        Self::new(SExpression::Symbol(s))
    }
}

impl std::fmt::Display for SExpressionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self.0)
    }
}

impl From<ConsCell> for SExpressionRef {
    fn from(c: ConsCell) -> Self {
        Self::cons_cell(c)
    }
}

impl From<Function> for SExpressionRef {
    fn from(f: Function) -> Self {
        Self::function(f)
    }
}

impl From<RustFunction> for SExpressionRef {
    fn from(f: RustFunction) -> Self {
        Self::function(f.into())
    }
}

impl From<LispFunction> for SExpressionRef {
    fn from(l: LispFunction) -> Self {
        Self::function(l.into())
    }
}

impl From<Macro> for SExpressionRef {
    fn from(f: Macro) -> Self {
        Self::r#macro(f)
    }
}

impl From<LispMacro> for SExpressionRef {
    fn from(f: LispMacro) -> Self {
        Self::r#macro(f.into())
    }
}

impl From<SExpression> for SExpressionRef {
    fn from(sx: SExpression) -> Self {
        Self::new(sx)
    }
}

impl From<RustMacro> for SExpressionRef {
    fn from(f: RustMacro) -> Self {
        Self::r#macro(f.into())
    }
}

impl From<LabelFunction> for SExpressionRef {
    fn from(f: LabelFunction) -> Self {
        Self::function(f.into())
    }
}

impl From<Vec<Self>> for SExpressionRef {
    fn from(v: Vec<Self>) -> Self {
        let mut ret = Self::nil();

        for e in v.into_iter().rev() {
            ret = util::cons(&e, &ret)
        }

        ret
    }
}

impl From<ListIter> for SExpressionRef {
    fn from(v: ListIter) -> Self {
        let mut ret = Self::nil();

        let veced: Vec<SExpressionRef> = v.collect();

        for e in veced.iter().rev() {
            ret = util::cons(&e, &ret)
        }

        ret
    }

}

impl FromIterator<SExpressionRef> for SExpressionRef {
    fn from_iter<I: IntoIterator<Item=SExpressionRef>>(iter: I) -> Self {
        let mut temp = Self::nil();

        for e in iter {
            temp = util::cons(&e, &temp)
        }

        let mut ret = Self::nil();

        for e in temp.iter() {
            ret = util::cons(&e, &ret)
        }

        ret
    }
}
