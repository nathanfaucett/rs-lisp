use alloc::collections::linked_list::{IntoIter, Iter, IterMut};
use alloc::collections::LinkedList;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::ptr;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_isize, new_kind, new_object, nil_value, Kind, Object, Scope,
  Value,
};

#[derive(Clone, PartialEq, PartialOrd, Eq)]
pub struct List(LinkedList<Gc<dyn Value>>);

impl Hash for List {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl From<LinkedList<Gc<dyn Value>>> for List {
  #[inline]
  fn from(list: LinkedList<Gc<dyn Value>>) -> Self {
    List(list)
  }
}

impl fmt::Debug for List {
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

impl IntoIterator for List {
  type Item = Gc<dyn Value>;
  type IntoIter = IntoIter<Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a List {
  type Item = &'a Gc<dyn Value>;
  type IntoIter = Iter<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'a> IntoIterator for &'a mut List {
  type Item = &'a mut Gc<dyn Value>;
  type IntoIter = IterMut<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

impl Trace for List {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for v in self.0.iter_mut() {
      v.trace(marked);
    }
  }
}

impl Deref for List {
  type Target = LinkedList<Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for List {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl List {
  #[inline]
  pub fn new() -> Self {
    List(LinkedList::new())
  }

  #[inline]
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let list_kind = new_kind::<List>(scope.clone(), "List");
    scope.set("List", list_kind.clone().into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(scope.clone(), "list.is_empty", vec!["list"], list_is_empty);
    add_external_function(scope.clone(), "list.len", vec!["list"], list_len);
    add_external_function(scope.clone(), "list.nth", vec!["list", "index"], list_nth);
    add_external_function(scope.clone(), "list.get", vec!["list", "index"], list_nth);
    add_external_function(
      scope.clone(),
      "list.push_front",
      vec!["list", "...args"],
      list_push_front,
    );
    add_external_function(
      scope,
      "list.push_back",
      vec!["list", "...args"],
      list_push_back,
    );
  }
}

#[inline]
pub fn list_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let list = args
    .front()
    .expect("List is nil")
    .downcast_ref::<Object<List>>()
    .expect("Failed to downcast to List");

  new_bool(scope, list.is_empty()).into_value()
}

#[inline]
pub fn list_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let list = args
    .front()
    .expect("List is nil")
    .downcast_ref::<Object<List>>()
    .expect("Failed to downcast to List");

  new_isize(scope, list.len() as isize).into_value()
}
#[inline]
pub fn list_nth(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let list = args
    .pop_front()
    .expect("List is nil")
    .downcast::<Object<List>>()
    .expect("Failed to downcast to List");
  let nth = args
    .pop_front()
    .expect("nth is nil")
    .downcast::<Object<isize>>()
    .expect("Failed to downcast to USize");

  list
    .iter()
    .nth(*nth.value() as usize)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn list_push_front(_scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut list = args
    .pop_front()
    .expect("List is nil")
    .downcast::<Object<List>>()
    .expect("Failed to downcast argument to List");

  for value in args.iter() {
    list.push_front(value.clone());
  }

  list.into_value()
}

#[inline]
pub fn list_push_back(_scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut list = args
    .pop_front()
    .expect("List is nil")
    .downcast::<Object<List>>()
    .expect("Failed to downcast argument to List");

  for value in args.iter() {
    list.push_back(value.clone());
  }

  list.into_value()
}

#[inline]
pub fn list_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("List")
      .expect("failed to get List Kind")
  }
}

#[inline]
pub fn new_list(scope: Gc<Object<Scope>>) -> Gc<Object<List>> {
  new_list_from(scope, List::new())
}

#[inline]
pub fn new_list_from(scope: Gc<Object<Scope>>, list: List) -> Gc<Object<List>> {
  new_object(scope.clone(), Object::new(list_kind(scope), list))
}
