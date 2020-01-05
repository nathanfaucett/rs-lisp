use alloc::collections::linked_list::{IntoIter, Iter};
use alloc::collections::LinkedList;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::iter::FromIterator;
use core::ops::Deref;
use core::ptr;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_isize, new_kind, new_object, nil_value, scope_get_with_kind,
  scope_set, Kind, Map, Object, PersistentScope, PersistentVector, Value,
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

impl<'a> FromIterator<&'a Gc<dyn Value>> for PersistentList {
  #[inline]
  fn from_iter<I: IntoIterator<Item = &'a Gc<dyn Value>>>(iter: I) -> Self {
    let mut persistent_list = PersistentList::new();

    for value in iter {
      persistent_list = persistent_list.push_back(value.clone());
    }

    persistent_list
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
  pub fn pop_front(&self) -> Self {
    let mut new_persistent_list = self.0.clone();
    new_persistent_list.pop_front();
    Self::from(new_persistent_list)
  }
  #[inline]
  pub fn pop_back(&self) -> Self {
    let mut new_persistent_list = self.0.clone();
    new_persistent_list.pop_back();
    Self::from(new_persistent_list)
  }

  #[inline]
  pub(crate) fn init_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let persistent_list_kind = new_kind::<PersistentList>(scope, "PersistentList");
    scope_set(
      scope,
      "PersistentList",
      persistent_list_kind.clone().into_value(),
    )
  }

  #[inline]
  pub(crate) fn init_scope(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let mut new_scope = add_external_function(
      scope,
      "persistent_list.is_empty",
      vec!["persistent_list"],
      persistent_list_is_empty,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_list.len",
      vec!["persistent_list"],
      persistent_list_len,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_list.nth",
      vec!["persistent_list", "index"],
      persistent_list_nth,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_list.get",
      vec!["persistent_list", "index"],
      persistent_list_nth,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_list.push_front",
      vec!["persistent_list", "...args"],
      persistent_list_push_front,
    );
    add_external_function(
      &new_scope,
      "persistent_list.push_back",
      vec!["persistent_list", "...args"],
      persistent_list_push_back,
    )
  }
}

#[inline]
pub fn persistent_list_is_empty(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_list = args
    .front()
    .expect("PersistentList is nil")
    .downcast_ref::<Object<PersistentList>>()
    .expect("Failed to downcast to PersistentList");

  new_bool(scope, persistent_list.is_empty()).into_value()
}

#[inline]
pub fn persistent_list_len(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_list = args
    .front()
    .expect("PersistentList is nil")
    .downcast_ref::<Object<PersistentList>>()
    .expect("Failed to downcast to PersistentList");

  new_isize(scope, persistent_list.len() as isize).into_value()
}
#[inline]
pub fn persistent_list_nth(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_list_value = args.front().expect("PersistentList is nil").clone();
  let persistent_list = persistent_list_value
    .downcast_ref::<Object<PersistentList>>()
    .expect("Failed to downcast to PersistentList");
  let nth_value = args.get(1).expect("nth is nil");
  let nth = nth_value
    .downcast_ref::<Object<isize>>()
    .expect("Failed to downcast to USize");

  persistent_list
    .iter()
    .nth(*nth.value() as usize)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn persistent_list_push_front(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut persistent_list = args
    .front()
    .expect("PersistentList is nil")
    .downcast_ref::<Object<PersistentList>>()
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
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let mut persistent_list = args
    .front()
    .expect("PersistentList is nil")
    .downcast_ref::<Object<PersistentList>>()
    .expect("Failed to downcast argument to PersistentList")
    .value()
    .clone();

  for value in args.iter() {
    persistent_list = persistent_list.push_back(value.clone());
  }

  new_persistent_list_from(scope, persistent_list).into_value()
}

#[inline]
pub fn persistent_list_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "PersistentList").expect("failed to get PersistentList Kind")
}

#[inline]
pub fn new_persistent_list(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentList>> {
  new_persistent_list_from(scope, PersistentList::new())
}

#[inline]
pub fn new_persistent_list_from(
  scope: &Gc<Object<PersistentScope>>,
  persistent_list: PersistentList,
) -> Gc<Object<PersistentList>> {
  new_object(
    scope,
    Object::new(persistent_list_kind(scope).clone(), persistent_list.clone()),
  )
}

#[inline]
pub fn new_persistent_list_from_with_meta(
  scope: &Gc<Object<PersistentScope>>,
  persistent_list: PersistentList,
  meta: Gc<Object<Map>>,
) -> Gc<Object<PersistentList>> {
  new_object(
    scope,
    Object::new_with_meta(
      persistent_list_kind(scope).clone(),
      persistent_list.clone(),
      meta,
    ),
  )
}
