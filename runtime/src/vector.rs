use std::collections::LinkedList;
use std::fmt;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

use gc::Gc;

use super::Value;

#[derive(Eq, Hash)]
pub struct Vector(Vec<Gc<Value>>);

impl fmt::Debug for Vector {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut list = f.debug_list();

        for value in self.0.iter() {
            list.entry(value);
        }

        list.finish()
    }
}

impl PartialEq for Vector {
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

impl Clone for Vector {
    #[inline]
    fn clone(&self) -> Self {
        Vector(self.0.iter().map(Clone::clone).collect::<Vec<Gc<Value>>>())
    }
}

impl IntoIterator for Vector {
    type Item = Gc<Value>;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Vector {
    type Item = &'a Gc<Value>;
    type IntoIter = Iter<'a, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Vector {
    type Item = &'a mut Gc<Value>;
    type IntoIter = IterMut<'a, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Vector {
    #[inline]
    pub fn new() -> Self {
        Vector(Vec::new())
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
    pub fn push(&mut self, value: Gc<Value>) -> &mut Self {
        self.0.push(value);
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
    pub fn append(&mut self, list: &mut Vector) -> &mut Self {
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
}
