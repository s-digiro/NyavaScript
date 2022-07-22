use super::SExpressionRef as SXRef;

mod iter;
use iter::ConsCellIter;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct ConsCell {
    pub car: SXRef,
    pub cdr: SXRef,
}

impl ConsCell {
    pub fn new(car: SXRef, cdr: SXRef) -> ConsCell {
        ConsCell {
            car,
            cdr,
        }
    }

    pub fn iter(&self) -> ConsCellIter {
        ConsCellIter::new(self)
    }
}

impl std::fmt::Display for ConsCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for e in self.iter() {
            write!(f, "{} ", e)?;
        }
        write!(f, ")")
    }
}
