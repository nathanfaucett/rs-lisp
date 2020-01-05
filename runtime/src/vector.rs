use alloc::vec::{IntoIter, Vec};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::iter::FromIterator;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice::{Iter, IterMut};

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, scope_get_with_kind,
  scope_set, Kind, Object, PersistentScope, PersistentVector, Value,
};

#[derive(Clone, Eq, PartialEq, PartialOrd)]
pub struct Vector(Vec<Gc<dyn Value>>);

impl Trace for Vector {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for v in self.0.iter_mut() {
      v.trace(marked);
    }
  }
}

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

impl Hash for Vector {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl From<Vec<Gc<dyn Value>>> for Vector {
  #[inline]
  fn from(vector: Vec<Gc<dyn Value>>) -> Self {
    Vector(vector)
  }
}

impl IntoIterator for Vector {
  type Item = Gc<dyn Value>;
  type IntoIter = IntoIter<Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Vector {
  type Item = &'a Gc<dyn Value>;
  type IntoIter = Iter<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'a> IntoIterator for &'a mut Vector {
  type Item = &'a mut Gc<dyn Value>;
  type IntoIter = IterMut<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

impl<'a> FromIterator<&'a Gc<dyn Value>> for Vector {
  #[inline]
  fn from_iter<I: IntoIterator<Item = &'a Gc<dyn Value>>>(iter: I) -> Self {
    let mut vector = Vector::new();

    for value in iter {
      vector.push(value.clone());
    }

    vector
  }
}

impl Deref for Vector {
  type Target = Vec<Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Vector {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Vector {
  #[inline]
  pub fn new() -> Self {
    Vector(Vec::new())
  }

  #[inline]
  pub(crate) fn init_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let vector_kind = new_kind::<Vector>(scope, "Vector");
    scope_set(scope, "Vector", vector_kind.clone().into_value())
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
  pub(crate) fn init_scope(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let mut new_scope =
      add_external_function(scope, "vector.is_empty", vec!["vector"], vector_is_empty);
    new_scope = add_external_function(&new_scope, "vector.len", vec!["vector"], vector_len);
    new_scope = add_external_function(
      &new_scope,
      "vector.nth",
      vec!["vector", "index"],
      vector_nth,
    );
    new_scope = add_external_function(
      &new_scope,
      "vector.get",
      vec!["vector", "index"],
      vector_nth,
    );
    new_scope = add_external_function(
      &new_scope,
      "vector.push_front",
      vec!["vector", "...args"],
      vector_push_front,
    );
    new_scope = add_external_function(
      &new_scope,
      "vector.push_back",
      vec!["vector", "...args"],
      vector_push_back,
    );
    add_external_function(
      &new_scope,
      "vector.insert",
      vec!["vector", "index", "value"],
      vector_insert,
    )
  }
}

#[inline]
pub fn vector_is_empty(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let vector = args
    .front()
    .expect("Vector is nil")
    .downcast_ref::<Object<Vector>>()
    .expect("Failed to downcast to Vector");

  new_bool(scope, vector.is_empty()).into_value()
}

#[inline]
pub fn vector_len(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let vector = args
    .front()
    .expect("Vector is nil")
    .downcast_ref::<Object<Vector>>()
    .expect("Failed to downcast to Vector");

  new_usize(scope, vector.len()).into_value()
}

#[inline]
pub fn vector_nth(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let vector_value = args.get(0).expect("Vector is nil");
  let vector = vector_value
    .downcast_ref::<Object<Vector>>()
    .expect("Failed to downcast to Vector");
  let nth_value = args.get(1).expect("nth is nil");
  let nth = nth_value
    .downcast_ref::<Object<usize>>()
    .expect("Failed to downcast to USize");

  vector
    .get(*nth.value())
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn vector_push_front(
  _scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut vector_value = args.front().expect("Vector is nil").clone();
  let vector = vector_value
    .downcast_mut::<Object<Vector>>()
    .expect("Failed to downcast argument to Vector");

  for value in args.iter() {
    vector.insert(0, value.clone());
  }

  vector_value
}

#[inline]
pub fn vector_push_back(
  _scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut vector_value = args.front().expect("Vector is nil").clone();
  let vector = vector_value
    .downcast_mut::<Object<Vector>>()
    .expect("Failed to downcast argument to Vector");

  for value in args.iter() {
    vector.push(value.clone());
  }

  vector_value
}

#[inline]
pub fn vector_insert(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut vector_value = args.front().expect("Vector is nil").clone();
  let vector = vector_value
    .downcast_mut::<Object<Vector>>()
    .expect("Failed to downcast argument to Vector");
  let index_value = args.get(1).expect("index is nil");
  let index = index_value
    .downcast_ref::<Object<usize>>()
    .expect("Failed to downcast argument to usize");
  let value = args
    .get(2)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  vector.insert(*index.value(), value);
  vector_value
}

#[inline]
pub fn vector_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Vector").expect("failed to get Vector Kind")
}

#[inline]
pub fn new_vector(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<Vector>> {
  new_vector_from(scope, Vector::new())
}

#[inline]
pub fn new_vector_from(scope: &Gc<Object<PersistentScope>>, vector: Vector) -> Gc<Object<Vector>> {
  new_object(
    scope,
    Object::new(vector_kind(scope).clone(), vector.clone()),
  )
}
