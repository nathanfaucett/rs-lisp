use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ptr;

use hashmap_core::map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut};
use hashmap_core::fnv::FnvHashMap;
use gc::{Gc, Trace};

use super::Value;

#[derive(Clone, PartialEq, Eq)]
pub struct Map(FnvHashMap<Gc<Value>, Gc<Value>>);

impl Trace for Map {
    #[inline]
    fn mark(&mut self) {
        for (k, v) in self.0.iter_mut() {
            unsafe {
                k.unsafe_as_mut().mark();
            }
            v.mark();
        }
    }
}

impl Hash for Map {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl fmt::Debug for Map {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('{')?;
        let mut index = self.len();

        for (key, value) in self.0.iter() {
            write!(f, "{:?} {:?}", key, value)?;

            index -= 1;
            if index != 0 {
                write!(f, ", ")?;
            }
        }

        f.write_char('}')
    }
}

impl IntoIterator for Map {
    type Item = (Gc<Value>, Gc<Value>);
    type IntoIter = IntoIter<Gc<Value>, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (&'a Gc<Value>, &'a Gc<Value>);
    type IntoIter = Iter<'a, Gc<Value>, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Map {
    type Item = (&'a Gc<Value>, &'a mut Gc<Value>);
    type IntoIter = IterMut<'a, Gc<Value>, Gc<Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Map {
    #[inline]
    pub fn new() -> Self {
        Map(FnvHashMap::default())
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
    pub fn set(&mut self, key: Gc<Value>, value: Gc<Value>) -> &mut Self {
        self.0.insert(key, value);
        self
    }

    #[inline]
    pub fn get(&self, key: &Gc<Value>) -> Option<&Gc<Value>> {
        self.0.get(key)
    }
    #[inline]
    pub fn get_mut(&mut self, key: &Gc<Value>) -> Option<&mut Gc<Value>> {
        self.0.get_mut(key)
    }

    #[inline]
    pub fn keys(&self) -> Keys<Gc<Value>, Gc<Value>> {
        self.0.keys()
    }
    #[inline]
    pub fn values(&self) -> Values<Gc<Value>, Gc<Value>> {
        self.0.values()
    }
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<Gc<Value>, Gc<Value>> {
        self.0.values_mut()
    }

    #[inline]
    pub fn iter(&self) -> Iter<Gc<Value>, Gc<Value>> {
        self.0.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Gc<Value>, Gc<Value>> {
        self.0.iter_mut()
    }
}
