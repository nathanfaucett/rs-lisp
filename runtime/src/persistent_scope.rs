use alloc::string::{String, ToString};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Write};
use core::hash::{Hash, Hasher};
use core::ops::Deref;
use core::ptr;

use gc::{Gc, Trace};
use hashbrown::hash_map::{IntoIter, Iter};
use hashbrown::HashMap;

use super::{
  add_external_function, new_bool, new_object, new_usize, nil_value, Kind, Object,
  PersistentVector, Symbol, Value,
};

#[derive(Clone, PartialEq, Eq)]
pub struct PersistentScope {
  pub map: HashMap<String, Gc<dyn Value>>,
  parent: Option<Gc<Object<PersistentScope>>>,
}

impl Default for PersistentScope {
  #[inline]
  fn default() -> Self {
    PersistentScope {
      map: HashMap::default(),
      parent: None,
    }
  }
}

impl PartialOrd for PersistentScope {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Trace for PersistentScope {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for (_k, v) in self.map.iter_mut() {
      v.trace(marked);
    }
  }
}

impl Hash for PersistentScope {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for PersistentScope {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('{')?;
    let mut index = self.len();

    for (key, value) in self.map.iter() {
      write!(f, "{:?} {:?}", key, value)?;

      index -= 1;
      if index != 0 {
        write!(f, ", ")?;
      }
    }

    f.write_char('}')
  }
}

impl IntoIterator for PersistentScope {
  type Item = (String, Gc<dyn Value>);
  type IntoIter = IntoIter<String, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.map.into_iter()
  }
}

impl<'a> IntoIterator for &'a PersistentScope {
  type Item = (&'a String, &'a Gc<dyn Value>);
  type IntoIter = Iter<'a, String, Gc<dyn Value>>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.map.iter()
  }
}

impl Deref for PersistentScope {
  type Target = HashMap<String, Gc<dyn Value>>;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.map
  }
}

impl PersistentScope {
  #[inline]
  pub fn new(
    map: HashMap<String, Gc<dyn Value>>,
    parent: Option<Gc<Object<PersistentScope>>>,
  ) -> Self {
    PersistentScope { map, parent }
  }

  #[inline]
  pub fn set(&self, key: &str, value: Gc<dyn Value>) -> Self {
    let mut new_map = self.map.clone();
    new_map.insert(key.to_string(), value);
    Self::new(new_map, self.parent.clone())
  }

  #[inline]
  pub fn remove(&self, key: &str) -> Self {
    let mut new_map = self.map.clone();
    new_map.remove(key);
    Self::new(new_map, self.parent.clone())
  }

  #[inline]
  pub fn has(&self, key: &str) -> bool {
    self.map.contains_key(key)
  }

  #[inline]
  pub fn get_mut(&mut self, key: &str) -> Option<&mut Gc<dyn Value>> {
    self.map.get_mut(key)
  }

  #[inline]
  pub(crate) fn init_scope(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let mut new_scope = add_external_function(
      scope,
      "persistent_scope.is_empty",
      vec!["persistent_scope"],
      persistent_scope_is_empty,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_scope.len",
      vec!["persistent_scope"],
      persistent_scope_len,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_scope.get",
      vec!["persistent_scope", "key"],
      persistent_scope_get,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_scope.remove",
      vec!["persistent_scope", "key"],
      persistent_scope_remove,
    );
    new_scope = add_external_function(
      &new_scope,
      "persistent_scope.has",
      vec!["persistent_scope", "key"],
      persistent_scope_has,
    );
    add_external_function(
      &new_scope,
      "persistent_scope.set",
      vec!["persistent_scope", "key", "value"],
      persistent_scope_set,
    )
  }
}

#[inline]
pub fn persistent_scope_is_empty(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_scope = args
    .front()
    .expect("PersistentScope is nil")
    .downcast_ref::<Object<PersistentScope>>()
    .expect("Failed to downcast to PersistentScope");

  new_bool(scope, persistent_scope.is_empty()).into_value()
}

#[inline]
pub fn persistent_scope_len(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_scope = args
    .front()
    .expect("PersistentScope is nil")
    .downcast_ref::<Object<PersistentScope>>()
    .expect("Failed to downcast to PersistentScope");

  new_usize(scope, persistent_scope.len()).into_value()
}

#[inline]
pub fn persistent_scope_has(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_scope_value = args.front().expect("PersistentScope is nil").clone();
  let persistent_scope = persistent_scope_value
    .downcast_ref::<Object<PersistentScope>>()
    .expect("Failed to downcast to PersistentScope");
  let key = args
    .get(1)
    .expect("key is nil")
    .downcast_ref::<Object<Symbol>>()
    .expect("failed to downcast key to Symbol");

  new_bool(scope, persistent_scope.has(key)).into_value()
}

#[inline]
pub fn persistent_scope_get(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_scope_value = args.front().expect("PersistentScope is nil").clone();
  let persistent_scope = persistent_scope_value
    .downcast_ref::<Object<PersistentScope>>()
    .expect("Failed to downcast to PersistentScope");
  let key = args
    .get(1)
    .expect("key is nil")
    .downcast_ref::<Object<Symbol>>()
    .expect("failed to downcast key to Symbol");

  persistent_scope
    .get(key.value().deref())
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn persistent_scope_remove(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_scope_value = args.front().expect("PersistentScope is nil");
  let persistent_scope = persistent_scope_value
    .downcast_ref::<Object<PersistentScope>>()
    .expect("Failed to downcast to PersistentScope");
  let key = args
    .get(1)
    .expect("key is nil")
    .downcast_ref::<Object<Symbol>>()
    .expect("failed to downcast key to Symbol");

  new_persistent_scope_from(scope, persistent_scope.remove(key)).into_value()
}

#[inline]
pub fn persistent_scope_set(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let persistent_scope_value = args.front().expect("PersistentScope is nil").clone();
  let persistent_scope = persistent_scope_value
    .downcast_ref::<Object<PersistentScope>>()
    .expect("Failed to downcast to PersistentScope");
  let key_value = args.get(1).expect("key is nil");
  let key = key_value
    .downcast_ref::<Object<Symbol>>()
    .expect("failed to downcast key to Symbol");
  let value = args
    .get(2)
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  new_persistent_scope_from(scope, persistent_scope.set(key.value().deref(), value)).into_value()
}

#[inline]
pub fn persistent_scope_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "PersistentScope").expect("failed to get PersistentScope Kind")
}
#[inline]
pub fn new_persistent_scope(parent: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
  new_persistent_scope_from(
    parent,
    PersistentScope::new(HashMap::default(), Some(parent.clone())),
  )
}

#[inline]
pub fn new_persistent_scope_from(
  scope: &Gc<Object<PersistentScope>>,
  persistent_scope: PersistentScope,
) -> Gc<Object<PersistentScope>> {
  new_object(
    scope,
    Object::new(persistent_scope_kind(scope).clone(), persistent_scope),
  )
}

#[inline]
pub fn scope_parent(scope: &Gc<Object<PersistentScope>>) -> Option<&Gc<Object<PersistentScope>>> {
  scope.parent.as_ref()
}

#[inline]
pub fn get_scope_root(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<PersistentScope>> {
  if let Some(parent) = scope_parent(scope) {
    get_scope_root(parent)
  } else {
    scope
  }
}

#[inline]
pub fn scope_get_by_value<'a>(
  scope: &'a Gc<Object<PersistentScope>>,
  ident: &str,
) -> Option<&'a Gc<dyn Value>> {
  if let Some(value) = scope.get(ident) {
    return Some(value);
  } else if let Some(parent) = scope_parent(scope) {
    return scope_get_by_value(parent, ident);
  } else {
    None
  }
}
#[inline]
pub fn scope_get_mut_by_value<'a>(
  scope: &'a Gc<Object<PersistentScope>>,
  ident: &str,
) -> Option<&'a mut Gc<dyn Value>> {
  if let Some(value) = unsafe { scope.unsafe_as_mut() }.get_mut(ident) {
    return Some(value);
  } else if let Some(ref parent) = scope_parent(scope) {
    return scope_get_mut_by_value(parent, ident);
  } else {
    None
  }
}

#[inline]
pub fn scope_get<'a>(
  scope: &'a Gc<Object<PersistentScope>>,
  ident: &str,
) -> Option<&'a Gc<dyn Value>> {
  scope_get_by_value(scope, ident)
}

#[inline]
pub fn scope_get_mut<'a>(
  scope: &'a Gc<Object<PersistentScope>>,
  ident: &str,
) -> Option<&'a mut Gc<dyn Value>> {
  scope_get_mut_by_value(scope, ident)
}

#[inline]
pub fn scope_set(
  scope: &Gc<Object<PersistentScope>>,
  ident: &str,
  value: Gc<dyn Value>,
) -> Gc<Object<PersistentScope>> {
  let new_scope = scope.set(ident, value);
  new_persistent_scope_from(scope, new_scope)
}

#[inline]
pub fn scope_get_with_kind<'a, T>(
  scope: &'a Gc<Object<PersistentScope>>,
  ident: &str,
) -> Option<&'a Gc<Object<T>>>
where
  T: 'static + Hash + Debug + PartialEq + PartialOrd + Trace,
{
  scope_get(scope, ident).and_then(|value| value.downcast_ref::<Object<T>>())
}

#[inline]
pub fn scope_get_mut_with_kind<'a, T>(
  scope: &'a Gc<Object<PersistentScope>>,
  ident: &str,
) -> Option<&'a mut Gc<Object<T>>>
where
  T: 'static + Hash + Debug + PartialEq + PartialOrd + Trace,
{
  scope_get_mut(scope, ident).and_then(|value| value.downcast_mut::<Object<T>>())
}

#[inline]
pub fn new_scope(parent: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
  new_persistent_scope_from(
    parent,
    PersistentScope::new(HashMap::default(), Some(parent.clone())),
  )
}
