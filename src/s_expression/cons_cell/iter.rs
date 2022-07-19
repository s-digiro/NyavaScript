use super::*;
use crate::s_expression::list::{ List, ListIter };

impl<'a> IntoIterator for &'a ConsCell {
    type Item = SXRef;
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
    type Item = SXRef;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            Current::Amp(e) => {
                let ret = SXRef::clone(&e.car);

                self.current = Current::Ref(List::iter(&e.cdr));

                Some(ret)
            },
            Current::Ref(i) => i.next(),
        }
    }
}
