use alloc::collections::linked_list::{IntoIter, Iter, IterMut};
use alloc::collections::LinkedList;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::{Gc, Trace};

use super::{
  add_external_function, new_bool, new_isize, new_kind, new_object, Kind, List, Object, Scope,
  Value,
};

#[derive(Clone, PartialEq, PartialOrd, Eq)]
pub struct LinkedMap(LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>);

impl Trace for LinkedMap {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for (k, v) in self.0.iter_mut() {
      k.trace(marked);
      v.trace(marked);
    }
  }
}

impl Hash for LinkedMap {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for LinkedMap {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.is_empty() {
      f.write_str("{}")
    } else {
      f.write_char('{')?;
      let mut index = self.len();

      for (k, v) in self.0.iter() {
        write!(f, "{:?} {:?}", k, v)?;

        index -= 1;
        if index != 0 {
          write!(f, ", ")?;
        }
      }

      f.write_char('}')
    }
  }
}

impl IntoIterator for LinkedMap {
  type Item = (Gc<dyn Value>, Gc<dyn Value>);
  type IntoIter = IntoIter<Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a LinkedMap {
  type Item = &'a (Gc<dyn Value>, Gc<dyn Value>);
  type IntoIter = Iter<'a, (Gc<dyn Value>, Gc<dyn Value>)>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'a> IntoIterator for &'a mut LinkedMap {
  type Item = &'a mut (Gc<dyn Value>, Gc<dyn Value>);
  type IntoIter = IterMut<'a, (Gc<dyn Value>, Gc<dyn Value>)>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

impl From<LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>> for LinkedMap {
  #[inline]
  fn from(linked_list: LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>) -> Self {
    LinkedMap(linked_list)
  }
}

impl LinkedMap {
  #[inline]
  pub fn new() -> Self {
    LinkedMap(LinkedList::new())
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
  pub fn has(&mut self, key: &Gc<dyn Value>) -> bool {
    self.iter().find(|&(ref k, _)| k == key).is_some()
  }

  #[inline]
  pub fn get(&mut self, key: &Gc<dyn Value>) -> Option<&Gc<dyn Value>> {
    self
      .iter()
      .find(|&(ref k, _)| k == key)
      .map(|&(_, ref v)| v)
  }

  #[inline]
  pub fn push_front(&mut self, key_value: (Gc<dyn Value>, Gc<dyn Value>)) -> &mut Self {
    self.0.push_front(key_value);
    self
  }
  #[inline]
  pub fn push_back(&mut self, key_value: (Gc<dyn Value>, Gc<dyn Value>)) -> &mut Self {
    self.0.push_back(key_value);
    self
  }

  #[inline]
  pub fn pop_front(&mut self) -> Option<(Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.pop_front()
  }
  #[inline]
  pub fn pop_back(&mut self) -> Option<(Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.pop_back()
  }

  #[inline]
  pub fn front(&self) -> Option<&(Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.front()
  }
  #[inline]
  pub fn back(&self) -> Option<&(Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.back()
  }

  #[inline]
  pub fn front_mut(&mut self) -> Option<&mut (Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.front_mut()
  }
  #[inline]
  pub fn back_mut(&mut self) -> Option<&mut (Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.back_mut()
  }

  #[inline]
  pub fn append(&mut self, linked_map: &mut LinkedMap) -> &mut Self {
    self.0.append(&mut linked_map.0);
    self
  }
  #[inline]
  pub fn to_vec(&self) -> ::alloc::vec::Vec<(Gc<dyn Value>, Gc<dyn Value>)> {
    self
      .0
      .iter()
      .map(Clone::clone)
      .collect::<::alloc::vec::Vec<(Gc<dyn Value>, Gc<dyn Value>)>>()
  }

  #[inline]
  pub fn iter(&self) -> Iter<(Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.iter()
  }
  #[inline]
  pub fn iter_mut(&mut self) -> IterMut<(Gc<dyn Value>, Gc<dyn Value>)> {
    self.0.iter_mut()
  }

  #[inline]
  pub(crate) unsafe fn init_kind(mut scope: Gc<Object<Scope>>) {
    let linked_map_kind = new_kind::<LinkedMap>(scope.clone(), "LinkedMap");
    scope.set("LinkedMap", linked_map_kind.into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope.clone(),
      "linked_map.is_empty",
      vec!["linked_map"],
      linked_map_is_empty,
    );
    add_external_function(scope, "linked_map.len", vec!["linked_map"], linked_map_len);
  }
}

#[inline]
pub fn linked_map_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let linked_map = args
    .front()
    .expect("LinkedMap is nil")
    .downcast_ref::<Object<LinkedMap>>()
    .expect("Failed to downcast to LinkedMap");

  new_bool(scope, linked_map.is_empty()).into_value()
}

#[inline]
pub fn linked_map_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let linked_map = args
    .front()
    .expect("LinkedMap is nil")
    .downcast_ref::<Object<LinkedMap>>()
    .expect("Failed to downcast to LinkedMap");

  new_isize(scope, linked_map.len() as isize).into_value()
}

#[inline]
pub fn linked_map_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("LinkedMap")
      .expect("failed to get LinkedMap Kind")
  }
}

#[inline]
pub fn new_linked_map(
  scope: Gc<Object<Scope>>,
  linked_list: LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>,
) -> Gc<Object<LinkedMap>> {
  new_object(
    scope.clone(),
    Object::new(linked_map_kind(scope), LinkedMap::from(linked_list)),
  )
}
