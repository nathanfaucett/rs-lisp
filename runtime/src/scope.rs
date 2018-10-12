use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{Object, Value};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scope {
    parent: Option<Gc<Object<Scope>>>,
    map: HashMap<String, Gc<Value>>,
}

impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
    }
}

impl Scope {
    #[inline(always)]
    pub fn new_root() -> Self {
        Self::new(None)
    }

    #[inline(always)]
    pub fn new(parent: Option<Gc<Object<Scope>>>) -> Self {
        Scope {
            parent: parent,
            map: HashMap::new(),
        }
    }

    #[inline]
    pub fn get(&self, ident: &str) -> Option<&Gc<Value>> {
        if let Some(value) = self.map.get(ident) {
            return Some(value);
        }
        if let Some(ref parent) = self.parent {
            return parent.get(ident);
        }
        None
    }
    #[inline]
    pub fn get_mut(&mut self, ident: &str) -> Option<&mut Gc<Value>> {
        if let Some(value) = self.map.get_mut(ident) {
            return Some(value);
        }
        if let Some(ref mut parent) = self.parent {
            return parent.get_mut(ident);
        }
        None
    }

    #[inline]
    pub unsafe fn get_type<T>(&self, ident: &str) -> Option<Gc<Object<T>>>
    where
        T: 'static + Hash + Debug + PartialEq,
    {
        self.get(ident)
            .map(|value| value.clone().into_object_unchecked())
    }

    #[inline]
    pub fn scope_with(&self, ident: &str) -> Option<&Scope> {
        if self.map.contains_key(ident) {
            return Some(self);
        }
        if let Some(ref parent) = self.parent {
            return parent.scope_with(ident);
        }
        None
    }
    #[inline]
    pub fn scope_with_mut(&mut self, ident: &str) -> Option<&mut Scope> {
        if self.map.contains_key(ident) {
            return Some(self);
        }
        if let Some(ref mut parent) = self.parent {
            return parent.scope_with_mut(ident);
        }
        None
    }

    #[inline]
    pub fn has(&self, ident: &str) -> bool {
        if self.map.contains_key(ident) {
            return true;
        }
        if let Some(ref parent) = self.parent {
            return parent.has(ident);
        }
        false
    }

    #[inline]
    pub fn set(&mut self, ident: &str, value: Gc<Value>) -> bool {
        if let Some(scope) = self.scope_with_mut(ident) {
            scope.map.insert(ident.into(), value);
            return false;
        }
        self.map.insert(ident.into(), value);
        true
    }
}
