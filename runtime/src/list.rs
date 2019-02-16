use std::collections::linked_list::{IntoIter, Iter, IterMut};
use std::collections::LinkedList;
use std::fmt;

use gc::Gc;

use super::{add_kind_method, new_bool, new_usize, Kind, Object, Scope, Value};

#[derive(Eq, Hash)]
pub struct List(LinkedList<Gc<Value>>);

impl fmt::Debug for List {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tuple = f.debug_tuple("");

        for value in self.0.iter() {
            tuple.field(value);
        }

        tuple.finish()
    }
}

impl PartialEq for List {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            false
        } else {
            let mut a = self.0.iter();
            let mut b = other.0.iter();

            while let Some(a_value) = a.next() {
                if Some(a_value) == b.next() {
                    return false;
                }
            }

            true
        }
    }
}

impl Clone for List {
    #[inline]
    fn clone(&self) -> Self {
        List(
            self.0
                .iter()
                .map(Clone::clone)
                .collect::<LinkedList<Gc<Value>>>(),
        )
    }
}

impl IntoIterator for List {
    type Item = Gc<Value>;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a Gc<Value>;
    type IntoIter = Iter<'a, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut List {
    type Item = &'a mut Gc<Value>;
    type IntoIter = IterMut<'a, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl List {
    #[inline]
    pub fn new() -> Self {
        List(LinkedList::new())
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
    pub fn push_front(&mut self, value: Gc<Value>) -> &mut Self {
        self.0.push_front(value);
        self
    }
    #[inline]
    pub fn push_back(&mut self, value: Gc<Value>) -> &mut Self {
        self.0.push_back(value);
        self
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<Gc<Value>> {
        self.0.pop_front()
    }
    #[inline]
    pub fn pop_back(&mut self) -> Option<Gc<Value>> {
        self.0.pop_back()
    }

    #[inline]
    pub fn front(&self) -> Option<&Gc<Value>> {
        self.0.front()
    }
    #[inline]
    pub fn back(&self) -> Option<&Gc<Value>> {
        self.0.back()
    }

    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut Gc<Value>> {
        self.0.front_mut()
    }
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut Gc<Value>> {
        self.0.back_mut()
    }

    #[inline]
    pub fn append(&mut self, list: &mut List) -> &mut Self {
        self.0.append(&mut list.0);
        self
    }
    #[inline]
    pub fn to_vec(&self) -> Vec<Gc<Value>> {
        self.0.iter().map(Clone::clone).collect::<Vec<Gc<Value>>>()
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
    pub(crate) fn init(scope: &Gc<Object<Scope>>, list_kind: &mut Gc<Object<Kind>>) {
        add_kind_method(scope, list_kind, "is_empty", list_is_empty);
        add_kind_method(scope, list_kind, "len", list_len);
    }
}

#[inline]
pub fn list_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<Value> {
    let list = args
        .front()
        .expect("List is nil")
        .downcast_ref::<Object<List>>()
        .expect("Failed to downcast to List");

    new_bool(&scope, list.is_empty()).into_value()
}

#[inline]
pub fn list_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<Value> {
    let list = args
        .front()
        .expect("List is nil")
        .downcast_ref::<Object<List>>()
        .expect("Failed to downcast to List");

    new_usize(&scope, list.len()).into_value()
}
