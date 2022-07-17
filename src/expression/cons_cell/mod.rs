use super::ValRef;

mod iter;
use iter::ConsCellIter;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct ConsCell {
    pub car: ValRef,
    pub cdr: ValRef,
}

impl ConsCell {
    pub fn nil() -> ValRef {
        ValRef::nil()
    }

    pub fn new(car: ValRef, cdr: ValRef) -> ConsCell {
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
