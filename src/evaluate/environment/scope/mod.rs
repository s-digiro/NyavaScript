use crate::s_expression::SExpressionRef as SXRef;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Scope: Debug {
    fn contains_key(&self, key: &str) -> bool;
    fn entries(&self) -> Vec<(& str, & SXRef)>;
    fn get(&self, key: &str) -> Option<& SXRef>;
    fn keys(&self) -> Vec<& str>;
    fn insert(&mut self, key: String, val: SXRef);
    fn remove(&mut self, key: &str) -> Option<SXRef>;
    fn vals(& self) -> Vec<& SXRef>;
}

type HashScope = HashMap<String, SXRef>;

impl Scope for HashScope {
    fn contains_key(& self, key: &str) -> bool {
        HashMap::contains_key(self, key)
    }

    fn entries(& self) -> Vec<(& str, & SXRef)> {
        HashMap::iter(self).map(|(k, v)| (k.as_str(), v)).collect()
    }

    fn get(& self, key: &str) -> Option<& SXRef> {
        HashMap::get(self, key)
    }

    fn keys(& self) -> Vec<& str> {
        HashMap::keys(self).map(|s| s.as_str()).collect()
    }

    fn insert(& mut self, key: String, val: SXRef) {
        HashMap::insert(self, key, val);
    }

    fn remove(& mut self, key: &str) -> Option<SXRef> {
        HashMap::remove(self, key)
    }

    fn vals(& self) -> Vec<& SXRef> {
        HashMap::values(self).collect()
    }
}
