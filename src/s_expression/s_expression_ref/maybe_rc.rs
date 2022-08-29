use std::rc::Rc;
use std::boxed::Box;
use std::ops::Deref;

#[derive(Clone, PartialEq, Debug)]
pub enum MaybeRc<T> {
    Rc(Rc<T>),
    Owned(Box<T>),
}

impl<T> MaybeRc<T> {
    pub fn rc(val: T) -> Self {
        Self::Rc(Rc::new(val))
    }

    pub fn owned(val: T) -> Self {
        Self::Owned(Box::new(val))
    }
}

impl<T> Deref for MaybeRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Rc(rc) => &*rc,
            Self::Owned(o) => o,
        }
    }
}
