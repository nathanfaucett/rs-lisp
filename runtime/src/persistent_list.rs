use alloc::collections::linked_list::{IntoIter, Iter};
use alloc::collections::LinkedList;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_isize, new_kind, new_object, nil_value, Kind, List, Object,
  Scope, Value,
};

#[derive(Clone, PartialEq, PartialOrd, Eq)]
pub struct PersistentList(LinkedList<Gc<dyn Value>>);

impl Hash for PersistentList {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for PersistentList {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('(')?;
    let mut index = self.len();

    for value in self.0.iter() {
      write!(f, "{:?}", value)?;

      index -= 1;
      if index != 0 {
        write!(f, ", ")?;
      }
    }

    f.write_char(')')
  }
}

impl IntoIterator for PersistentList {
  type Item = Gc<dyn Value>;
  type IntoIter = IntoIter<Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a PersistentList {
  type Item = &'a Gc<dyn Value>;
  type IntoIter = Iter<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Trace for PersistentList {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for v in self.0.iter_mut() {
      v.trace(marked);
    }
  }
}

impl From<LinkedList<Gc<dyn Value>>> for PersistentList {
  #[inline]
  fn from(list: LinkedList<Gc<dyn Value>>) -> Self {
    PersistentList(list)
  }
}

impl Deref for PersistentList {
  type Target = LinkedList<Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl PersistentList {
  #[inline]
  pub fn new() -> Self {
    PersistentList(LinkedList::new())
  }

  #[inline]
  pub fn push_front(&self, value: Gc<dyn Value>) -> Self {
    let mut new_persistent_list = self.0.clone();
    new_persistent_list.push_front(value);
    Self::from(new_persistent_list)
  }
  #[inline]
  pub fn push_back(&self, value: Gc<dyn Value>) -> Self {
    let mut new_persistent_list = self.0.clone();
    new_persistent_list.push_back(value);
    Self::from(new_persistent_list)
  }

  #[inline]
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let persistent_list_kind = new_kind::<PersistentList>(scope.clone(), "PersistentList");
    scope.set("PersistentList", persistent_list_kind.clone().into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope.clone(),
      "persistent_list.is_empty",
      vec!["persistent_list"],
      persistent_list_is_empty,
    );
    add_external_function(
      scope.clone(),
      "persistent_list.len",
      vec!["persistent_list"],
      persistent_list_len,
    );
    add_external_function(
      scope.clone(),
      "persistent_list.nth",
      vec!["persistent_list", "index"],
      persistent_list_nth,
    );
    add_external_function(
      scope.clone(),
      "persistent_list.get",
      vec!["persistent_list", "index"],
      persistent_list_nth,
    );
    add_external_function(
      scope.clone(),
      "persistent_list.push_front",
      vec!["persistent_list", "...args"],
      persistent_list_push_front,
    );
    add_external_function(
      scope,
      "persistent_list.push_back",
      vec!["persistent_list", "...args"],
      persistent_list_push_back,
    );
  }
}

#[inline]
pub fn persistent_list_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let persistent_list = args
    .front()
    .expect("PersistentList is nil")
    .downcast_ref::<Object<PersistentList>>()
    .expect("Failed to downcast to PersistentList");

  new_bool(scope, persistent_list.is_empty()).into_value()
}

#[inline]
pub fn persistent_list_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let persistent_list = args
    .front()
    .expect("PersistentList is nil")
    .downcast_ref::<Object<PersistentList>>()
    .expect("Failed to downcast to PersistentList");

  new_isize(scope, persistent_list.len() as isize).into_value()
}
#[inline]
pub fn persistent_list_nth(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let persistent_list = args
    .pop_front()
    .expect("PersistentList is nil")
    .downcast::<Object<PersistentList>>()
    .expect("Failed to downcast to PersistentList");
  let nth = args
    .pop_front()
    .expect("nth is nil")
    .downcast::<Object<isize>>()
    .expect("Failed to downcast to USize");

  persistent_list
    .iter()
    .nth(*nth.value() as usize)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn persistent_list_push_front(
  scope: Gc<Object<Scope>>,
  mut args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let mut persistent_list = args
    .pop_front()
    .expect("PersistentList is nil")
    .downcast::<Object<PersistentList>>()
    .expect("Failed to downcast argument to PersistentList")
    .value()
    .clone();

  for value in args.iter() {
    persistent_list = persistent_list.push_front(value.clone());
  }

  new_persistent_list_from(scope, persistent_list).into_value()
}

#[inline]
pub fn persistent_list_push_back(
  scope: Gc<Object<Scope>>,
  mut args: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let mut persistent_list = args
    .pop_front()
    .expect("PersistentList is nil")
    .downcast::<Object<PersistentList>>()
    .expect("Failed to downcast argument to PersistentList")
    .value()
    .clone();

  for value in args.iter() {
    persistent_list = persistent_list.push_back(value.clone());
  }

  new_persistent_list_from(scope, persistent_list).into_value()
}

#[inline]
pub fn persistent_list_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("PersistentList")
      .expect("failed to get PersistentList Kind")
  }
}

#[inline]
pub fn new_persistent_list(scope: Gc<Object<Scope>>) -> Gc<Object<PersistentList>> {
  new_persistent_list_from(scope, PersistentList::new())
}

#[inline]
pub fn new_persistent_list_from(
  scope: Gc<Object<Scope>>,
  persistent_list: PersistentList,
) -> Gc<Object<PersistentList>> {
  new_object(
    scope.clone(),
    Object::new(persistent_list_kind(scope), persistent_list.clone()),
  )
}
