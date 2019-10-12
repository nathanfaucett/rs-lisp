use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::{Gc, Trace};
use hashbrown::hash_map::{IntoIter, Iter, IterMut, Keys, Values, ValuesMut};
use hashbrown::HashMap;

use super::{add_external_function, new_bool, new_usize, nil_value, List, Object, Scope, Value};

#[derive(Clone, PartialEq, Eq)]
pub struct Map(HashMap<Gc<dyn Value>, Gc<dyn Value>>);

impl Trace for Map {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for (k, v) in self.0.iter_mut() {
      unsafe {
        k.unsafe_as_mut().trace(marked);
      }
      v.trace(marked);
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
  type Item = (Gc<dyn Value>, Gc<dyn Value>);
  type IntoIter = IntoIter<Gc<dyn Value>, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Map {
  type Item = (&'a Gc<dyn Value>, &'a Gc<dyn Value>);
  type IntoIter = Iter<'a, Gc<dyn Value>, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'a> IntoIterator for &'a mut Map {
  type Item = (&'a Gc<dyn Value>, &'a mut Gc<dyn Value>);
  type IntoIter = IterMut<'a, Gc<dyn Value>, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

impl Map {
  #[inline]
  pub fn new() -> Self {
    Map(HashMap::default())
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
  pub fn set(&mut self, key: Gc<dyn Value>, value: Gc<dyn Value>) -> &mut Self {
    self.0.insert(key, value);
    self
  }

  #[inline]
  pub fn remove(&mut self, key: &Gc<dyn Value>) -> Option<Gc<dyn Value>> {
    self.0.remove(key)
  }

  #[inline]
  pub fn has(&self, key: &Gc<dyn Value>) -> bool {
    self.0.contains_key(key)
  }
  #[inline]
  pub fn get(&self, key: &Gc<dyn Value>) -> Option<&Gc<dyn Value>> {
    self.0.get(key)
  }
  #[inline]
  pub fn get_mut(&mut self, key: &Gc<dyn Value>) -> Option<&mut Gc<dyn Value>> {
    self.0.get_mut(key)
  }

  #[inline]
  pub fn keys(&self) -> Keys<Gc<dyn Value>, Gc<dyn Value>> {
    self.0.keys()
  }
  #[inline]
  pub fn values(&self) -> Values<Gc<dyn Value>, Gc<dyn Value>> {
    self.0.values()
  }
  #[inline]
  pub fn values_mut(&mut self) -> ValuesMut<Gc<dyn Value>, Gc<dyn Value>> {
    self.0.values_mut()
  }

  #[inline]
  pub fn iter(&self) -> Iter<Gc<dyn Value>, Gc<dyn Value>> {
    self.0.iter()
  }
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut<Gc<dyn Value>, Gc<dyn Value>> {
    self.0.iter_mut()
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(scope.clone(), "map.is_empty", vec!["map"], map_is_empty);
    add_external_function(scope.clone(), "map.len", vec!["map"], map_len);
    add_external_function(scope.clone(), "map.get", vec!["map", "key"], map_get);
    add_external_function(scope.clone(), "map.remove", vec!["map", "key"], map_remove);
    add_external_function(scope.clone(), "map.has", vec!["map", "key"], map_has);
    add_external_function(scope, "map.set", vec!["map", "key", "value"], map_set);
  }
}

#[inline]
pub fn map_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let map = args
    .front()
    .expect("Map is nil")
    .downcast_ref::<Object<Map>>()
    .expect("Failed to downcast to Map");

  new_bool(scope, map.is_empty()).into_value()
}

#[inline]
pub fn map_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let map = args
    .front()
    .expect("Map is nil")
    .downcast_ref::<Object<Map>>()
    .expect("Failed to downcast to Map");

  new_usize(scope, map.len()).into_value()
}

#[inline]
pub fn map_has(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let map = mut_args
    .pop_front()
    .expect("Map is nil")
    .downcast::<Object<Map>>()
    .expect("Failed to downcast to Map");
  let key = mut_args.pop_front().expect("key is nil");

  new_bool(scope, map.has(&key)).into_value()
}

#[inline]
pub fn map_get(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let map = mut_args
    .pop_front()
    .expect("Map is nil")
    .downcast::<Object<Map>>()
    .expect("Failed to downcast to Map");
  let key = mut_args.pop_front().expect("key is nil");

  map
    .get(&key)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn map_remove(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let mut map = mut_args
    .pop_front()
    .expect("Map is nil")
    .downcast::<Object<Map>>()
    .expect("Failed to downcast to Map");
  let key = mut_args.pop_front().expect("key is nil");

  map
    .remove(&key)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn map_set(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let mut map = mut_args
    .pop_front()
    .expect("Map is nil")
    .downcast::<Object<Map>>()
    .expect("Failed to downcast to Map");
  let key = mut_args.pop_front().expect("key is nil");
  let value = mut_args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope).into_value());

  map.set(key, value);
  map.into_value()
}
