use super::{
    SExpression as SX,
    SExpressionRef as SXRef,
};

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
        let mut s = "(".to_owned();

        let mut current = self;

        loop {
            s.push_str(&format!("{}", current.car));

            match &*current.cdr {
                SX::Nil => break,
                SX::ConsCell(c) => {
                    s.push(' ');
                    current = c;
                },
                x => {
                    s.push_str(&format!(" . {}", x));
                    break
                }
            }
        }

        s.push(')');

        write!(f, "{}", s)
    }
}
