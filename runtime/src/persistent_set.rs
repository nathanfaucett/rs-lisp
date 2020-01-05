use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr;

use gc::{Gc, Trace};
use hashbrown::hash_set::{IntoIter, Iter};
use hashbrown::HashSet;

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, scope_get_with_kind,
  scope_set, Kind, Object, PersistentScope, PersistentVector, Value,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PersistentSet(HashSet<Gc<dyn Value>>);

impl PartialOrd for PersistentSet {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Trace for PersistentSet {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for v in self.0.iter() {
      unsafe {
        v.unsafe_as_mut().trace(marked);
      }
    }
  }
}

impl From<HashSet<Gc<dyn Value>>> for PersistentSet {
  #[inline]
  fn from(set: HashSet<Gc<dyn Value>>) -> Self {
    PersistentSet(set)
  }
}

impl Hash for PersistentSet {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for PersistentSet {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('{')?;
    let mut index = self.0.len();

    for value in self.0.iter() {
      write!(f, "{:?}", value)?;

      index -= 1;
      if index != 0 {
        write!(f, ", ")?;
      }
    }

    f.write_char('}')
  }
}

impl IntoIterator for PersistentSet {
  type Item = Gc<dyn Value>;
  type IntoIter = IntoIter<Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a PersistentSet {
  type Item = &'a Gc<dyn Value>;
  type IntoIter = Iter<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Deref for PersistentSet {
  type Target = HashSet<Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl PersistentSet {
  #[inline]
  pub fn new() -> Self {
    PersistentSet(HashSet::default())
  }

  #[inline]
  pub fn add(&self, value: Gc<dyn Value>) -> Self {
    let mut new_persistent_set = self.0.clone();
    new_persistent_set.insert(value);
    Self::from(new_persistent_set)
  }

  #[inline]
  pub fn has(&self, value: &Gc<dyn Value>) -> bool {
    self.0.contains(value)
  }

  #[inline]
  pub fn remove(&self, value: &Gc<dyn Value>) -> Self {
    let mut new_persistent_set = self.0.clone();
    new_persistent_set.remove(value);
    Self::from(new_persistent_set)
  }

  #[inline]
  pub(crate) fn init_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let persistent_set_kind = new_kind::<PersistentSet>(scope, "PersistentSet");
    scope_set(scope, "PersistentSet", persistent_set_kind.into_value())
  }

  #[inline]
  pub(crate) fn init_scope(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let mut new_scope = add_external_function(
      scope,
      "persistent_set.is_empty",
      vec!["persistent_set"],
      persistent_set_is_empty,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_set.len",
      vec!["persistent_set"],
      persistent_set_len,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_set.get",
      vec!["persistent_set", "key"],
      persistent_set_get,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_set.remove",
      vec!["persistent_set", "key"],
      persistent_set_remove,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_set.has",
      vec!["persistent_set", "key"],
      persistent_set_has,
    );
    add_external_function(
      &new_scope,
      "persistent_set.add",
      vec!["persistent_set", "value"],
      persistent_set_add,
    )
  }
}

#[inline]
pub fn persistent_set_is_empty(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_set = args
    .front()
    .expect("PersistentSet is nil")
    .downcast_ref::<Object<PersistentSet>>()
    .expect("Failed to downcast to PersistentSet");

  new_bool(scope, persistent_set.is_empty()).into_value()
}

#[inline]
pub fn persistent_set_len(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_set = args
    .front()
    .expect("PersistentSet is nil")
    .downcast_ref::<Object<PersistentSet>>()
    .expect("Failed to downcast to PersistentSet");

  new_usize(scope, persistent_set.len()).into_value()
}

#[inline]
pub fn persistent_set_has(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_set_value = args.front().expect("PersistentSet is nil").clone();
  let persistent_set = persistent_set_value
    .downcast_ref::<Object<PersistentSet>>()
    .expect("Failed to downcast to PersistentSet");
  let key = args
    .get(1)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  new_bool(scope, persistent_set.has(&key)).into_value()
}

#[inline]
pub fn persistent_set_get(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_set_value = args.front().expect("PersistentSet is nil");
  let persistent_set = persistent_set_value
    .downcast_ref::<Object<PersistentSet>>()
    .expect("Failed to downcast to PersistentSet");
  let value = args
    .get(1)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  persistent_set
    .get(&value)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn persistent_set_remove(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_set_value = args.front().expect("PersistentSet is nil");
  let persistent_set = persistent_set_value
    .downcast_ref::<Object<PersistentSet>>()
    .expect("Failed to downcast to PersistentSet");
  let value = args
    .get(1)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  new_persistent_set_from(scope, persistent_set.remove(&value)).into_value()
}

#[inline]
pub fn persistent_set_add(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_set_value = args.front().expect("PersistentSet is nil");
  let persistent_set = persistent_set_value
    .downcast_ref::<Object<PersistentSet>>()
    .expect("Failed to downcast to PersistentSet");
  let value = args
    .get(1)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  new_persistent_set_from(scope, persistent_set.add(value)).into_value()
}

#[inline]
pub fn persistent_set_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "PersistentSet").expect("failed to get PersistentSet Kind")
}
#[inline]
pub fn new_persistent_set(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentSet>> {
  new_persistent_set_from(scope, PersistentSet::new())
}
#[inline]
pub fn new_persistent_set_from(
  scope: &Gc<Object<PersistentScope>>,
  persistent_set: PersistentSet,
) -> Gc<Object<PersistentSet>> {
  new_object(
    scope,
    Object::new(persistent_set_kind(scope).clone(), persistent_set),
  )
}
