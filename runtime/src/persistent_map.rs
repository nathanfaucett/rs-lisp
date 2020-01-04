use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr;

use gc::{Gc, Trace};
use hashbrown::hash_map::{IntoIter, Iter};
use hashbrown::HashMap;

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, Kind, List, Object,
  Scope, Value,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PersistentMap(HashMap<Gc<dyn Value>, Gc<dyn Value>>);

impl PartialOrd for PersistentMap {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Trace for PersistentMap {
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

impl From<HashMap<Gc<dyn Value>, Gc<dyn Value>>> for PersistentMap {
  #[inline]
  fn from(map: HashMap<Gc<dyn Value>, Gc<dyn Value>>) -> Self {
    PersistentMap(map)
  }
}

impl Hash for PersistentMap {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for PersistentMap {
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

impl IntoIterator for PersistentMap {
  type Item = (Gc<dyn Value>, Gc<dyn Value>);
  type IntoIter = IntoIter<Gc<dyn Value>, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a PersistentMap {
  type Item = (&'a Gc<dyn Value>, &'a Gc<dyn Value>);
  type IntoIter = Iter<'a, Gc<dyn Value>, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Deref for PersistentMap {
  type Target = HashMap<Gc<dyn Value>, Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl PersistentMap {
  #[inline]
  pub fn new() -> Self {
    PersistentMap(HashMap::default())
  }

  #[inline]
  pub fn set(&self, key: Gc<dyn Value>, value: Gc<dyn Value>) -> Self {
    let mut new_map = self.0.clone();
    new_map.insert(key, value);
    Self::from(new_map)
  }

  #[inline]
  pub fn remove(&self, key: &Gc<dyn Value>) -> Self {
    let mut new_map = self.0.clone();
    new_map.remove(key);
    Self::from(new_map)
  }

  #[inline]
  pub fn has(&self, key: &Gc<dyn Value>) -> bool {
    self.0.contains_key(key)
  }

  #[inline]
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let persistent_map_kind = new_kind::<PersistentMap>(scope.clone(), "PersistentMap");
    scope.set("PersistentMap", persistent_map_kind.into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope.clone(),
      "persistent_map.is_empty",
      vec!["persistent_map"],
      persistent_map_is_empty,
    );
    add_external_function(
      scope.clone(),
      "persistent_map.len",
      vec!["persistent_map"],
      persistent_map_len,
    );
    add_external_function(
      scope.clone(),
      "persistent_map.get",
      vec!["persistent_map", "key"],
      persistent_map_get,
    );
    add_external_function(
      scope.clone(),
      "persistent_map.remove",
      vec!["persistent_map", "key"],
      persistent_map_remove,
    );
    add_external_function(
      scope.clone(),
      "persistent_map.has",
      vec!["persistent_map", "key"],
      persistent_map_has,
    );
    add_external_function(
      scope,
      "persistent_map.set",
      vec!["persistent_map", "key", "value"],
      persistent_map_set,
    );
  }
}

#[inline]
pub fn persistent_map_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let persistent_map = args
    .front()
    .expect("PersistentMap is nil")
    .downcast_ref::<Object<PersistentMap>>()
    .expect("Failed to downcast to PersistentMap");

  new_bool(scope, persistent_map.is_empty()).into_value()
}

#[inline]
pub fn persistent_map_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let persistent_map = args
    .front()
    .expect("PersistentMap is nil")
    .downcast_ref::<Object<PersistentMap>>()
    .expect("Failed to downcast to PersistentMap");

  new_usize(scope, persistent_map.len()).into_value()
}

#[inline]
pub fn persistent_map_has(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let persistent_map = mut_args
    .pop_front()
    .expect("PersistentMap is nil")
    .downcast::<Object<PersistentMap>>()
    .expect("Failed to downcast to PersistentMap");
  let key = mut_args.pop_front().expect("key is nil");

  new_bool(scope, persistent_map.has(&key)).into_value()
}

#[inline]
pub fn persistent_map_get(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let persistent_map = mut_args
    .pop_front()
    .expect("PersistentMap is nil")
    .downcast::<Object<PersistentMap>>()
    .expect("Failed to downcast to PersistentMap");
  let key = mut_args.pop_front().expect("key is nil");

  persistent_map
    .get(&key)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn persistent_map_remove(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let persistent_map = mut_args
    .pop_front()
    .expect("PersistentMap is nil")
    .downcast::<Object<PersistentMap>>()
    .expect("Failed to downcast to PersistentMap");
  let key = mut_args.pop_front().expect("key is nil");

  new_persistent_map_from(scope, persistent_map.remove(&key)).into_value()
}

#[inline]
pub fn persistent_map_set(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let persistent_map = mut_args
    .pop_front()
    .expect("PersistentMap is nil")
    .downcast::<Object<PersistentMap>>()
    .expect("Failed to downcast to PersistentMap");
  let key = mut_args.pop_front().expect("key is nil");
  let value = mut_args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  new_persistent_map_from(scope, persistent_map.set(key, value)).into_value()
}

#[inline]
pub fn persistent_map_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("PersistentMap")
      .expect("failed to get PersistentMap Kind")
  }
}
#[inline]
pub fn new_persistent_map(scope: Gc<Object<Scope>>) -> Gc<Object<PersistentMap>> {
  new_object(
    scope.clone(),
    Object::new(persistent_map_kind(scope), PersistentMap::new()),
  )
}

#[inline]
pub fn new_persistent_map_from(
  scope: Gc<Object<Scope>>,
  persistent_map: PersistentMap,
) -> Gc<Object<PersistentMap>> {
  new_object(
    scope.clone(),
    Object::new(persistent_map_kind(scope), persistent_map),
  )
}
