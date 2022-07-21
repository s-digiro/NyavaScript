use super::*;

pub struct ListIter {
    pub current: SExpressionRef,
}

impl Iterator for ListIter {
    type Item = SExpressionRef;

    fn next(&mut self) -> Option<Self::Item> {
        match *self.current {
            SExpression::ConsCell(_) => {
                let ret = util::car(&self.current);

                self.current = util::cdr(&self.current);
                Some(ret)
            }
            _ => None,
        }
    }
}
