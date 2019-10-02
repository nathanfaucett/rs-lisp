use alloc::string::String;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::ptr;

use super::{Object, Value};
use gc::{Gc, Trace};
use hashbrown::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scope {
    parent: Option<Gc<Object<Scope>>>,
    map: HashMap<String, Gc<dyn Value>>,
}

impl Hash for Scope {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state);
    }
}

impl Scope {
    #[inline(always)]
    pub fn new(parent: Option<Gc<Object<Scope>>>) -> Self {
        Scope {
            parent: parent,
            map: HashMap::default(),
        }
    }

    #[inline]
    pub fn get(&self, ident: &str) -> Option<&Gc<dyn Value>> {
        if let Some(value) = self.map.get(ident) {
            return Some(value);
        }
        if let Some(ref parent) = self.parent {
            return parent.get(ident);
        }
        None
    }
    #[inline]
    pub fn get_mut(&mut self, ident: &str) -> Option<&mut Gc<dyn Value>> {
        if let Some(value) = self.map.get_mut(ident) {
            return Some(value);
        }
        if let Some(ref mut parent) = self.parent {
            return parent.get_mut(ident);
        }
        None
    }

    #[inline]
    pub unsafe fn get_with_type<T>(&self, ident: &str) -> Option<Gc<Object<T>>>
    where
        T: 'static + Hash + Debug + PartialEq + Trace,
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
    pub fn set(&mut self, ident: &str, value: Gc<dyn Value>) -> bool {
        if let Some(scope) = self.scope_with_mut(ident) {
            scope.map.insert(ident.into(), value);
            return false;
        }
        self.map.insert(ident.into(), value);
        true
    }
}

impl Trace for Scope {
    #[inline]
    fn mark(&mut self) {
        if let Some(parent) = &mut self.parent {
            parent.mark();
        }

        for (_, v) in self.map.iter_mut() {
            v.mark();
        }
    }
}
