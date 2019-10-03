use alloc::string::String;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::ptr;

use super::{add_external_function, new_bool, new_scope, nil_value, Kind, List, Object, Value};
use gc::{Gc, Trace};
use hashbrown::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scope {
  parent: Option<Gc<Object<Scope>>>,
  map: HashMap<String, Gc<dyn Value>>,
}

impl Hash for Scope {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state);
  }
}

impl Trace for Scope {
  #[inline]
  fn mark(&mut self) {
    if let Some(parent) = &mut self.parent {
      parent.mark();
    }

    for (_, v) in self.map.iter_mut() {
      v.mark();
    }
  }
}

impl Scope {
  #[inline(always)]
  pub fn new(parent: Option<Gc<Object<Scope>>>) -> Self {
    Scope {
      parent: parent,
      map: HashMap::default(),
    }
  }

  #[inline]
  pub fn get(&self, ident: &str) -> Option<&Gc<dyn Value>> {
    if let Some(value) = self.map.get(ident) {
      return Some(value);
    }
    if let Some(ref parent) = self.parent {
      return parent.get(ident);
    }
    None
  }
  #[inline]
  pub fn get_mut(&mut self, ident: &str) -> Option<&mut Gc<dyn Value>> {
    if let Some(value) = self.map.get_mut(ident) {
      return Some(value);
    }
    if let Some(ref mut parent) = self.parent {
      return parent.get_mut(ident);
    }
    None
  }

  #[inline]
  pub unsafe fn get_with_type<T>(&self, ident: &str) -> Option<Gc<Object<T>>>
  where
    T: 'static + Hash + Debug + PartialEq + Trace,
  {
    self
      .get(ident)
      .map(|value| value.clone().into_object_unchecked())
  }

  #[inline]
  pub fn scope_with(&self, ident: &str) -> Option<&Scope> {
    if self.map.contains_key(ident) {
      return Some(self);
    }
    if let Some(ref parent) = self.parent {
      return parent.scope_with(ident);
    }
    None
  }
  #[inline]
  pub fn scope_with_mut(&mut self, ident: &str) -> Option<&mut Scope> {
    if self.map.contains_key(ident) {
      return Some(self);
    }
    if let Some(ref mut parent) = self.parent {
      return parent.scope_with_mut(ident);
    }
    None
  }

  #[inline]
  pub fn has(&self, ident: &str) -> bool {
    if self.map.contains_key(ident) {
      return true;
    }
    if let Some(ref parent) = self.parent {
      return parent.has(ident);
    }
    false
  }

  #[inline]
  pub fn set(&mut self, ident: &str, value: Gc<dyn Value>) -> bool {
    if let Some(scope) = self.scope_with_mut(ident) {
      scope.map.insert(ident.into(), value);
      return false;
    }
    self.map.insert(ident.into(), value);
    true
  }

  #[inline]
  pub fn add(&mut self, ident: &str, value: Gc<dyn Value>) -> bool {
    self.map.insert(ident.into(), value).is_some()
  }

  #[inline]
  pub(crate) fn init_scope(mut scope: Gc<Object<Scope>>, scope_kind: Gc<Object<Kind>>) {
    let mut scope_scope = new_scope(scope.clone());

    scope.set("scope", scope_scope.clone().into_value());

    scope_scope.set("Scope", scope_kind.clone().into_value());
    add_external_function(scope_scope.clone(), "get", vec!["scope", "key"], scope_get);
    add_external_function(scope_scope.clone(), "has", vec!["scope", "key"], scope_has);
    add_external_function(scope_scope, "set", vec!["scope", "key", "value"], scope_set);

    add_external_function(scope, "scope_get", vec!["scope", "key"], scope_get);
  }
}

#[inline]
pub fn scope_has(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let local_scope = mut_args
    .pop_front()
    .expect("Scope is nil")
    .downcast::<Object<Scope>>()
    .expect("Failed to downcast to Scope");
  let key = mut_args
    .pop_front()
    .expect("key is nil")
    .downcast::<Object<String>>()
    .expect("Failed to downcast to String");

  new_bool(scope, local_scope.has(key.value())).into_value()
}

#[inline]
pub fn scope_get(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let local_scope = mut_args
    .pop_front()
    .expect("Scope is nil")
    .downcast::<Object<Scope>>()
    .expect("Failed to downcast to Scope");
  let key = mut_args
    .pop_front()
    .expect("key is nil")
    .downcast::<Object<String>>()
    .expect("Failed to downcast to String");

  local_scope
    .get(key.value())
    .map(Clone::clone)
    .unwrap_or(nil_value(scope).into_value())
}

#[inline]
pub fn scope_set(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut mut_args = args.clone();
  let mut local_scope = mut_args
    .pop_front()
    .expect("Scope is nil")
    .downcast::<Object<Scope>>()
    .expect("Failed to downcast to Scope");
  let key = mut_args
    .pop_front()
    .expect("key is nil")
    .downcast::<Object<String>>()
    .expect("Failed to downcast to String");
  let value = mut_args
    .pop_front()
    .unwrap_or(nil_value(scope).into_value());

  local_scope.set(key.value(), value);
  local_scope.into_value()
}
