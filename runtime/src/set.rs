use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::ptr;

use gc::{Gc, Trace};
use hashbrown::hash_set::{IntoIter, Iter};
use hashbrown::HashSet;

use super::{
  add_external_function, new_bool, new_kind, new_object, new_usize, nil_value, Kind, List, Object,
  Scope, Value,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Set(HashSet<Gc<dyn Value>>);

impl PartialOrd for Set {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Trace for Set {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for v in self.0.iter() {
      unsafe {
        v.unsafe_as_mut().trace(marked);
      }
    }
  }
}

impl Hash for Set {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for Set {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('{')?;
    let mut index = self.len();

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

impl IntoIterator for Set {
  type Item = Gc<dyn Value>;
  type IntoIter = IntoIter<Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Set {
  type Item = &'a Gc<dyn Value>;
  type IntoIter = Iter<'a, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Deref for Set {
  type Target = HashSet<Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Set {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Set {
  #[inline]
  pub fn new() -> Self {
    Set(HashSet::default())
  }

  #[inline]
  pub fn add(&mut self, value: Gc<dyn Value>) -> &mut Self {
    self.0.insert(value);
    self
  }

  #[inline]
  pub fn has(&self, key: &Gc<dyn Value>) -> bool {
    self.0.contains(key)
  }

  #[inline]
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let set_kind = new_kind::<Set>(scope.clone(), "Set");
    scope.set("Set", set_kind.into_value());
  }

  #[inline]
  pub fn remove(&mut self, value: &Gc<dyn Value>) -> Option<Gc<dyn Value>> {
    if self.0.remove(value) {
      Some(value.clone())
    } else {
      None
    }
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(scope.clone(), "set.is_empty", vec!["set"], set_is_empty);
    add_external_function(scope.clone(), "set.len", vec!["set"], set_len);
    add_external_function(scope.clone(), "set.get", vec!["set", "key"], set_get);
    add_external_function(scope.clone(), "set.remove", vec!["set", "key"], set_remove);
    add_external_function(scope.clone(), "set.has", vec!["set", "key"], set_has);
    add_external_function(scope, "set.add", vec!["set", "value"], set_add);
  }
}

#[inline]
pub fn set_is_empty(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let set = args
    .front()
    .expect("Set is nil")
    .downcast_ref::<Object<Set>>()
    .expect("Failed to downcast to Set");

  new_bool(scope, set.is_empty()).into_value()
}

#[inline]
pub fn set_len(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let set = args
    .front()
    .expect("Set is nil")
    .downcast_ref::<Object<Set>>()
    .expect("Failed to downcast to Set");

  new_usize(scope, set.len()).into_value()
}

#[inline]
pub fn set_has(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let set = mut_args
    .pop_front()
    .expect("Set is nil")
    .downcast::<Object<Set>>()
    .expect("Failed to downcast to Set");
  let value = mut_args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  new_bool(scope, set.has(&value)).into_value()
}

#[inline]
pub fn set_get(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let set = mut_args
    .pop_front()
    .expect("Set is nil")
    .downcast::<Object<Set>>()
    .expect("Failed to downcast to Set");
  let value = mut_args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  set
    .get(&value)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn set_remove(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let mut set = mut_args
    .pop_front()
    .expect("Set is nil")
    .downcast::<Object<Set>>()
    .expect("Failed to downcast to Set");
  let value = mut_args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  set
    .remove(&value)
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn set_add(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let mut set = mut_args
    .pop_front()
    .expect("Set is nil")
    .downcast::<Object<Set>>()
    .expect("Failed to downcast to Set");
  let value = mut_args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope).into_value());

  set.add(value);
  set.into_value()
}

#[inline]
pub fn set_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Set")
      .expect("failed to get Set Kind")
  }
}
#[inline]
pub fn new_set(scope: Gc<Object<Scope>>) -> Gc<Object<Set>> {
  new_object(scope.clone(), Object::new(set_kind(scope), Set::new()))
}

#[inline]
pub fn new_set_from(scope: Gc<Object<Scope>>, set: Set) -> Gc<Object<Set>> {
  new_object(scope.clone(), Object::new(set_kind(scope), set))
}
