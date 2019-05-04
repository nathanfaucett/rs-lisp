use alloc::collections::LinkedList;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ptr;
use core::slice::{Iter, IterMut};
use alloc::vec::{self, IntoIter};

use gc::{Gc, Trace};

use super::{add_kind_method, new_bool, new_isize, nil_value, Kind, List, Object, Scope, Value};

#[derive(Clone, Eq, PartialEq)]
pub struct Vec(vec::Vec<Gc<Value>>);

impl Trace for Vec {
    #[inline]
    fn mark(&mut self) {
        for v in self.0.iter_mut() {
            v.mark();
        }
    }
}

impl fmt::Debug for Vec {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();

        for value in self.0.iter() {
            list.entry(value);
        }

        list.finish()
    }
}

impl Hash for Vec {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl From<vec::Vec<Gc<Value>>> for Vec {
    #[inline]
    fn from(vec: vec::Vec<Gc<Value>>) -> Self {
        Vec(vec)
    }
}

impl IntoIterator for Vec {
    type Item = Gc<Value>;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Vec {
    type Item = &'a Gc<Value>;
    type IntoIter = Iter<'a, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Vec {
    type Item = &'a mut Gc<Value>;
    type IntoIter = IterMut<'a, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Vec {
    #[inline]
    pub fn new() -> Self {
        Vec(vec::Vec::new())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&Gc<Value>> {
        self.0.get(index)
    }
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Gc<Value>> {
        self.0.get_mut(index)
    }

    #[inline]
    pub fn push(&mut self, value: Gc<Value>) -> &mut Self {
        self.0.push(value);
        self
    }

    #[inline]
    pub fn push_front(&mut self, value: Gc<Value>) -> &mut Self {
        self.0.insert(0, value);
        self
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Gc<Value>> {
        self.0.pop()
    }

    #[inline]
    pub fn front(&self) -> Option<&Gc<Value>> {
        self.0.first()
    }
    #[inline]
    pub fn back(&self) -> Option<&Gc<Value>> {
        self.0.last()
    }

    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut Gc<Value>> {
        self.0.first_mut()
    }
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut Gc<Value>> {
        self.0.last_mut()
    }

    #[inline]
    pub fn append(&mut self, list: &mut Vec) -> &mut Self {
        self.0.append(&mut list.0);
        self
    }
    #[inline]
    pub fn to_list(&self) -> LinkedList<Gc<Value>> {
        self.0
            .iter()
            .map(Clone::clone)
            .collect::<LinkedList<Gc<Value>>>()
    }

    #[inline]
    pub fn iter(&self) -> Iter<Gc<Value>> {
        self.0.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Gc<Value>> {
        self.0.iter_mut()
    }

    #[inline]
    pub(crate) fn init(scope: &Gc<Object<Scope>>, vec_kind: &mut Gc<Object<Kind>>) {
        add_kind_method(scope, vec_kind, "is_empty", vec_is_empty);
        add_kind_method(scope, vec_kind, "len", vec_len);
        add_kind_method(scope, vec_kind, "nth", vec_nth);
    }
}

#[inline]
pub fn vec_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<Value> {
    let vec = args
        .front()
        .expect("Vec is nil")
        .downcast_ref::<Object<Vec>>()
        .expect("Failed to downcast to Vec");

    new_bool(&scope, vec.is_empty()).into_value()
}

#[inline]
pub fn vec_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<Value> {
    let vec = args
        .front()
        .expect("Vec is nil")
        .downcast_ref::<Object<Vec>>()
        .expect("Failed to downcast to Vec");

    new_isize(&scope, vec.len() as isize).into_value()
}

#[inline]
pub fn vec_nth(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let vec = args
        .pop_front()
        .expect("Vec is nil")
        .downcast::<Object<Vec>>()
        .expect("Failed to downcast to Vec");
    let nth = args
        .pop_front()
        .expect("nth is nil")
        .downcast::<Object<isize>>()
        .expect("Failed to downcast to USize");

    vec.get(*nth.value() as usize)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(&scope).into_value())
}