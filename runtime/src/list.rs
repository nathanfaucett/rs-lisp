use alloc::collections::linked_list::{IntoIter, Iter, IterMut};
use alloc::collections::LinkedList;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_isize, new_kind, new_object, Kind, Object, Scope, Value,
};

#[derive(Clone, PartialEq, PartialOrd, Eq)]
pub struct List(LinkedList<Gc<dyn Value>>);

impl Hash for List {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for List {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_empty() {
      f.write_str("()")
    } else {
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

impl List {
  #[inline]
  pub fn new() -> Self {
    List(LinkedList::new())
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
  pub fn push_front(&mut self, value: Gc<dyn Value>) -> &mut Self {
    self.0.push_front(value);
    self
  }
  #[inline]
  pub fn push_back(&mut self, value: Gc<dyn Value>) -> &mut Self {
    self.0.push_back(value);
    self
  }

  #[inline]
  pub fn pop_front(&mut self) -> Option<Gc<dyn Value>> {
    self.0.pop_front()
  }
  #[inline]
  pub fn pop_back(&mut self) -> Option<Gc<dyn Value>> {
    self.0.pop_back()
  }

  #[inline]
  pub fn front(&self) -> Option<&Gc<dyn Value>> {
    self.0.front()
  }
  #[inline]
  pub fn back(&self) -> Option<&Gc<dyn Value>> {
    self.0.back()
  }

  #[inline]
  pub fn front_mut(&mut self) -> Option<&mut Gc<dyn Value>> {
    self.0.front_mut()
  }
  #[inline]
  pub fn back_mut(&mut self) -> Option<&mut Gc<dyn Value>> {
    self.0.back_mut()
  }

  #[inline]
  pub fn append(&mut self, list: &mut List) -> &mut Self {
    self.0.append(&mut list.0);
    self
  }
  #[inline]
  pub fn to_vec(&self) -> ::alloc::vec::Vec<Gc<dyn Value>> {
    self
      .0
      .iter()
      .map(Clone::clone)
      .collect::<::alloc::vec::Vec<Gc<dyn Value>>>()
  }

  #[inline]
  pub fn iter(&self) -> Iter<Gc<dyn Value>> {
    self.0.iter()
  }
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut<Gc<dyn Value>> {
    self.0.iter_mut()
  }

  #[inline]
  pub(crate) unsafe fn init_kind(mut scope: Gc<Object<Scope>>) {
    let list_kind = new_kind::<List>(scope.clone(), "List");
    scope.set("List", list_kind.clone().into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(scope.clone(), "list.is_empty", vec!["list"], list_is_empty);
    add_external_function(scope, "list.len", vec!["list"], list_len);
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
  new_object(scope.clone(), Object::new(list_kind(scope), list.clone()))
}
