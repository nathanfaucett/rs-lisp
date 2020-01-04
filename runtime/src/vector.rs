use alloc::vec::{IntoIter, Vec};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice::{Iter, IterMut};

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, Kind, List, Object,
  Scope, Value,
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
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let vector_kind = new_kind::<Vector>(scope.clone(), "Vector");
    scope.set("Vector", vector_kind.clone().into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope.clone(),
      "vector.is_empty",
      vec!["vector"],
      vector_is_empty,
    );
    add_external_function(scope.clone(), "vector.len", vec!["vector"], vector_len);
    add_external_function(
      scope.clone(),
      "vector.nth",
      vec!["vector", "index"],
      vector_nth,
    );
    add_external_function(
      scope.clone(),
      "vector.get",
      vec!["vector", "index"],
      vector_nth,
    );
    add_external_function(
      scope.clone(),
      "vector.push_front",
      vec!["vector", "...args"],
      vector_push_front,
    );
    add_external_function(
      scope.clone(),
      "vector.push_back",
      vec!["vector", "...args"],
      vector_push_back,
    );
    add_external_function(
      scope,
      "vector.insert",
      vec!["vector", "index", "value"],
      vector_insert,
    );
  }
}

#[inline]
pub fn vector_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let vector = args
    .front()
    .expect("Vector is nil")
    .downcast_ref::<Object<Vector>>()
    .expect("Failed to downcast to Vector");

  new_bool(scope, vector.is_empty()).into_value()
}

#[inline]
pub fn vector_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let vector = args
    .front()
    .expect("Vector is nil")
    .downcast_ref::<Object<Vector>>()
    .expect("Failed to downcast to Vector");

  new_usize(scope, vector.len()).into_value()
}

#[inline]
pub fn vector_nth(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let vector = args
    .pop_front()
    .expect("Vector is nil")
    .downcast::<Object<Vector>>()
    .expect("Failed to downcast to Vector");
  let nth = args
    .pop_front()
    .expect("nth is nil")
    .downcast::<Object<usize>>()
    .expect("Failed to downcast to USize");

  vector
    .get(*nth.value())
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn vector_push_front(_scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut vector = args
    .pop_front()
    .expect("Vector is nil")
    .downcast::<Object<Vector>>()
    .expect("Failed to downcast argument to Vector");

  for value in args.iter() {
    vector.insert(0, value.clone());
  }

  vector.into_value()
}

#[inline]
pub fn vector_push_back(_scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut vector = args
    .pop_front()
    .expect("Vector is nil")
    .downcast::<Object<Vector>>()
    .expect("Failed to downcast argument to Vector");

  for value in args.iter() {
    vector.push(value.clone());
  }

  vector.into_value()
}

#[inline]
pub fn vector_insert(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut vector = args
    .pop_front()
    .expect("Vector is nil")
    .downcast::<Object<Vector>>()
    .expect("Failed to downcast argument to Vector");
  let index = args
    .pop_front()
    .expect("index is nil")
    .downcast::<Object<usize>>()
    .expect("Failed to downcast argument to usize");
  let value = args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  vector.insert(*index.value(), value);
  vector.into_value()
}

#[inline]
pub fn vector_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Vector")
      .expect("failed to get Vector Kind")
  }
}

#[inline]
pub fn new_vector(scope: Gc<Object<Scope>>) -> Gc<Object<Vector>> {
  new_vector_from(scope, Vector::new())
}

#[inline]
pub fn new_vector_from(scope: Gc<Object<Scope>>, vector: Vector) -> Gc<Object<Vector>> {
  new_object(
    scope.clone(),
    Object::new(vector_kind(scope), vector.clone()),
  )
}
