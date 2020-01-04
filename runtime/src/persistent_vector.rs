use alloc::vec::{IntoIter, Vec};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr;
use core::slice::Iter;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, Kind, List, Object,
  Scope, Value,
};

#[derive(Clone, Eq, PartialEq, PartialOrd)]
pub struct PersistentVector(Vec<Gc<dyn Value>>);

impl Trace for PersistentVector {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for v in self.0.iter_mut() {
      v.trace(marked);
    }
  }
}

impl fmt::Debug for PersistentVector {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut list = f.debug_list();

    for value in self.0.iter() {
      list.entry(value);
    }

    list.finish()
  }
}

impl Hash for PersistentVector {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl From<Vec<Gc<dyn Value>>> for PersistentVector {
  #[inline]
  fn from(vec: Vec<Gc<dyn Value>>) -> Self {
    PersistentVector(vec)
  }
}

impl IntoIterator for PersistentVector {
  type Item = Gc<dyn Value>;
  type IntoIter = IntoIter<Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a PersistentVector {
  type Item = &'a Gc<dyn Value>;
  type IntoIter = Iter<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Deref for PersistentVector {
  type Target = Vec<Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl PersistentVector {
  #[inline]
  pub fn new() -> Self {
    PersistentVector(Vec::new())
  }

  #[inline]
  pub fn insert(&self, index: usize, value: Gc<dyn Value>) -> Self {
    let mut new_persistent_vector = self.0.clone();
    new_persistent_vector.insert(index, value);
    Self::from(new_persistent_vector)
  }
  #[inline]
  pub fn push_front(&self, value: Gc<dyn Value>) -> Self {
    let mut new_persistent_vector = self.0.clone();
    new_persistent_vector.insert(0, value);
    Self::from(new_persistent_vector)
  }
  #[inline]
  pub fn push_back(&self, value: Gc<dyn Value>) -> Self {
    let mut new_persistent_vector = self.0.clone();
    new_persistent_vector.push(value);
    Self::from(new_persistent_vector)
  }

  #[inline]
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let persistent_vector_kind = new_kind::<PersistentVector>(scope.clone(), "PersistentVector");
    scope.set(
      "PersistentVector",
      persistent_vector_kind.clone().into_value(),
    );
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope.clone(),
      "persistent_vector.is_empty",
      vec!["persistent_vector"],
      persistent_vector_is_empty,
    );
    add_external_function(
      scope.clone(),
      "persistent_vector.len",
      vec!["persistent_vector"],
      persistent_vector_len,
    );
    add_external_function(
      scope.clone(),
      "persistent_vector.nth",
      vec!["persistent_vector", "index"],
      persistent_vector_nth,
    );
    add_external_function(
      scope.clone(),
      "persistent_vector.get",
      vec!["persistent_vector", "index"],
      persistent_vector_nth,
    );
    add_external_function(
      scope.clone(),
      "persistent_vector.push_front",
      vec!["persistent_vector", "...args"],
      persistent_vector_push_front,
    );
    add_external_function(
      scope.clone(),
      "persistent_vector.push_back",
      vec!["persistent_vector", "...args"],
      persistent_vector_push_back,
    );
    add_external_function(
      scope,
      "persistent_vector.insert",
      vec!["persistent_vector", "index", "value"],
      persistent_vector_insert,
    );
  }
}

#[inline]
pub fn persistent_vector_is_empty(
  scope: Gc<Object<Scope>>,
  args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let persistent_vector = args
    .front()
    .expect("PersistentVector is nil")
    .downcast_ref::<Object<PersistentVector>>()
    .expect("Failed to downcast to PersistentVector");

  new_bool(scope, persistent_vector.is_empty()).into_value()
}

#[inline]
pub fn persistent_vector_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let persistent_vector = args
    .front()
    .expect("PersistentVector is nil")
    .downcast_ref::<Object<PersistentVector>>()
    .expect("Failed to downcast to PersistentVector");

  new_usize(scope, persistent_vector.len()).into_value()
}

#[inline]
pub fn persistent_vector_nth(
  scope: Gc<Object<Scope>>,
  mut args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let persistent_vector = args
    .pop_front()
    .expect("PersistentVector is nil")
    .downcast::<Object<PersistentVector>>()
    .expect("Failed to downcast to PersistentVector");
  let nth = args
    .pop_front()
    .expect("nth is nil")
    .downcast::<Object<usize>>()
    .expect("Failed to downcast to USize");

  persistent_vector
    .get(*nth.value())
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn persistent_vector_push_front(
  scope: Gc<Object<Scope>>,
  mut args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let mut persistent_vector = args
    .pop_front()
    .expect("PersistentVector is nil")
    .downcast::<Object<PersistentVector>>()
    .expect("Failed to downcast argument to PersistentVector")
    .value()
    .clone();

  for value in args.iter() {
    persistent_vector = persistent_vector.push_front(value.clone());
  }

  new_persistent_vector_from(scope, persistent_vector).into_value()
}

#[inline]
pub fn persistent_vector_push_back(
  scope: Gc<Object<Scope>>,
  mut args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let mut persistent_vector = args
    .pop_front()
    .expect("PersistentVector is nil")
    .downcast::<Object<PersistentVector>>()
    .expect("Failed to downcast argument to PersistentVector")
    .value()
    .clone();

  for value in args.iter() {
    persistent_vector = persistent_vector.push_back(value.clone());
  }

  new_persistent_vector_from(scope, persistent_vector).into_value()
}

#[inline]
pub fn persistent_vector_insert(
  scope: Gc<Object<Scope>>,
  mut args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let persistent_vector = args
    .pop_front()
    .expect("PersistentVector is nil")
    .downcast::<Object<PersistentVector>>()
    .expect("Failed to downcast argument to PersistentVector");
  let index = args
    .pop_front()
    .expect("index is nil")
    .downcast::<Object<usize>>()
    .expect("Failed to downcast argument to usize");
  let value = args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  new_persistent_vector_from(scope, persistent_vector.insert(*index.value(), value)).into_value()
}

#[inline]
pub fn persistent_vector_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("PersistentVector")
      .expect("failed to get PersistentVector Kind")
  }
}

#[inline]
pub fn new_persistent_vector(scope: Gc<Object<Scope>>) -> Gc<Object<PersistentVector>> {
  new_persistent_vector_from(scope, PersistentVector::new())
}

#[inline]
pub fn new_persistent_vector_from(
  scope: Gc<Object<Scope>>,
  persistent_vector: PersistentVector,
) -> Gc<Object<PersistentVector>> {
  new_object(
    scope.clone(),
    Object::new(persistent_vector_kind(scope), persistent_vector),
  )
}
