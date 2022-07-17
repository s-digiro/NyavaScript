use super::ConsCell;
use crate::expression::{ ValRef, List };
use crate::expression::list::ListIter;

impl<'a> IntoIterator for &'a ConsCell {
    type Item = ValRef;
    type IntoIter = ConsCellIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ConsCellIter {
            current: Current::Amp(self),
        }
    }
}

enum Current<'a> {
    Amp(&'a ConsCell),
    Ref(ListIter),
}

pub struct ConsCellIter<'a> {
    current: Current<'a>,
}

impl<'a> ConsCellIter<'a> {
    pub fn new(cc: &'a ConsCell) -> ConsCellIter<'a> {
        ConsCellIter {
            current: Current::Amp(cc),
        }
    }
}

impl<'a> Iterator for ConsCellIter<'a> {
    type Item = ValRef;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            Current::Amp(e) => {
                let ret = ValRef::clone(&e.car);

                self.current = Current::Ref(List::iter(&e.cdr));

                Some(ret)
            },
            Current::Ref(i) => i.next(),
        }
    }
}
