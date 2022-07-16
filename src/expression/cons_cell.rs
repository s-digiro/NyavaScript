use super::{ ExRef, List, list::ListIter };

#[derive(Debug, PartialEq)]
pub struct ConsCell {
    pub car: ExRef,
    pub cdr: ExRef,
}

impl ConsCell {
    pub fn nil() -> ExRef {
        ExRef::nil()
    }

    pub fn new(car: ExRef, cdr: ExRef) -> ExRef {
        ExRef::cons_cell(
            ConsCell {
                car,
                cdr,
            }
        )
    }

    pub fn iter(&self) -> ConsCellIter {
        ConsCellIter {
            current: Current::Amp(self),
        }
    }
}

impl<'a> IntoIterator for &'a ConsCell {
    type Item = ExRef;
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

impl<'a> Iterator for ConsCellIter<'a> {
    type Item = ExRef;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            Current::Amp(e) => {
                let ret = ExRef::clone(&e.car);

                self.current = Current::Ref(List::iter(&e.cdr));

                Some(ret)
            },
            Current::Ref(i) => i.next(),
        }
    }
}
