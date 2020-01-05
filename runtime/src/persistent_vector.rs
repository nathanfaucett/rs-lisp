use alloc::vec::{IntoIter, Vec};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::iter::FromIterator;
use core::ops::Deref;
use core::ptr;
use core::slice::Iter;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, scope_get_with_kind,
  scope_set, Kind, Map, Object, PersistentScope, Value,
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

impl<'a> FromIterator<&'a Gc<dyn Value>> for PersistentVector {
  #[inline]
  fn from_iter<I: IntoIterator<Item = &'a Gc<dyn Value>>>(iter: I) -> Self {
    let mut persistent_vector = PersistentVector::new();

    for value in iter {
      persistent_vector = persistent_vector.push(value.clone());
    }

    persistent_vector
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
  pub fn push(&self, value: Gc<dyn Value>) -> Self {
    self.push_back(value)
  }

  #[inline]
  pub fn pop_front(&self) -> Self {
    let mut new_persistent_vector = self.0.clone();
    new_persistent_vector.remove(0);
    Self::from(new_persistent_vector)
  }
  #[inline]
  pub fn pop_back(&self) -> Self {
    let mut new_persistent_vector = self.0.clone();
    new_persistent_vector.pop();
    Self::from(new_persistent_vector)
  }

  #[inline]
  pub fn front(&self) -> Option<&Gc<dyn Value>> {
    self.0.get(0)
  }
  #[inline]
  pub fn front_mut(&mut self) -> Option<&mut Gc<dyn Value>> {
    self.0.get_mut(0)
  }

  #[inline]
  pub fn back(&self) -> Option<&Gc<dyn Value>> {
    if self.is_empty() {
      None
    } else {
      let index = self.0.len() - 1;
      self.0.get(index)
    }
  }
  #[inline]
  pub fn back_mut(&mut self) -> Option<&mut Gc<dyn Value>> {
    if self.is_empty() {
      None
    } else {
      let index = self.0.len() - 1;
      self.0.get_mut(index)
    }
  }

  #[inline]
  pub(crate) fn init_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let persistent_vector_kind = new_kind::<PersistentVector>(scope, "PersistentVector");
    scope_set(
      scope,
      "PersistentVector",
      persistent_vector_kind.clone().into_value(),
    )
  }

  #[inline]
  pub(crate) fn init_scope(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let mut new_scope = add_external_function(
      scope,
      "persistent_vector.is_empty",
      vec!["persistent_vector"],
      persistent_vector_is_empty,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_vector.len",
      vec!["persistent_vector"],
      persistent_vector_len,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_vector.nth",
      vec!["persistent_vector", "index"],
      persistent_vector_nth,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_vector.get",
      vec!["persistent_vector", "index"],
      persistent_vector_nth,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_vector.push_front",
      vec!["persistent_vector", "...args"],
      persistent_vector_push_front,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_vector.push_back",
      vec!["persistent_vector", "...args"],
      persistent_vector_push_back,
    );
    add_external_function(
      &new_scope,
      "persistent_vector.insert",
      vec!["persistent_vector", "index", "value"],
      persistent_vector_insert,
    )
  }
}

#[inline]
pub fn persistent_vector_is_empty(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_vector = args
    .front()
    .expect("PersistentVector is nil")
    .downcast_ref::<Object<PersistentVector>>()
    .expect("Failed to downcast to PersistentVector");

  new_bool(scope, persistent_vector.is_empty()).into_value()
}

#[inline]
pub fn persistent_vector_len(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_vector = args
    .front()
    .expect("PersistentVector is nil")
    .downcast_ref::<Object<PersistentVector>>()
    .expect("Failed to downcast to PersistentVector");

  new_usize(scope, persistent_vector.len()).into_value()
}

#[inline]
pub fn persistent_vector_nth(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_vector_value = args.front().expect("PersistentVector is nil");
  let persistent_vector = persistent_vector_value
    .downcast_ref::<Object<PersistentVector>>()
    .expect("Failed to downcast to PersistentVector");
  let nth_value = args.get(1).expect("nth is nil");
  let nth = nth_value
    .downcast_ref::<Object<usize>>()
    .expect("Failed to downcast to USize");

  persistent_vector
    .get(*nth.value())
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn persistent_vector_push_front(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut persistent_vector = args
    .front()
    .expect("PersistentVector is nil")
    .downcast_ref::<Object<PersistentVector>>()
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
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut persistent_vector = args
    .front()
    .expect("PersistentVector is nil")
    .downcast_ref::<Object<PersistentVector>>()
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
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_vector_value = args.front().expect("PersistentVector is nil");
  let persistent_vector = persistent_vector_value
    .downcast_ref::<Object<PersistentVector>>()
    .expect("Failed to downcast argument to PersistentVector");
  let index_value = args.get(1).expect("index is nil");
  let index = index_value
    .downcast_ref::<Object<usize>>()
    .expect("Failed to downcast argument to usize");
  let value = args
    .get(2)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  new_persistent_vector_from(scope, persistent_vector.insert(*index.value(), value)).into_value()
}

#[inline]
pub fn persistent_vector_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "PersistentVector")
    .expect("failed to get PersistentVector Kind")
}

#[inline]
pub fn new_persistent_vector(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentVector>> {
  new_persistent_vector_from(scope, PersistentVector::new())
}

#[inline]
pub fn new_persistent_vector_from(
  scope: &Gc<Object<PersistentScope>>,
  persistent_vector: PersistentVector,
) -> Gc<Object<PersistentVector>> {
  new_object(
    scope,
    Object::new(persistent_vector_kind(scope).clone(), persistent_vector),
  )
}

#[inline]
pub fn new_persistent_vector_from_with_meta(
  scope: &Gc<Object<PersistentScope>>,
  persistent_vector: PersistentVector,
  meta: Gc<Object<Map>>,
) -> Gc<Object<PersistentVector>> {
  new_object(
    scope,
    Object::new_with_meta(
      persistent_vector_kind(scope).clone(),
      persistent_vector,
      meta,
    ),
  )
}
