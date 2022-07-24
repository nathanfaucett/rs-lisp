use alloc::string::{String, ToString};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Write};
use core::hash::{Hash, Hasher};
use core::{ops::Deref, ptr};

use gc::{Gc, Trace};
use hashbrown::HashMap;
use parking_lot::RwLock;

use super::{new_object, Kind, Object, Value};

pub struct Scope {
  map: RwLock<HashMap<String, Gc<dyn Value>>>,
  parent: Option<Gc<Object<Scope>>>,
}

impl Default for Scope {
  #[inline]
  fn default() -> Self {
    Scope::new(HashMap::default(), None)
  }
}

impl Clone for Scope {
  #[inline]
  fn clone(&self) -> Self {
    Scope::new(self.map.read().clone(), self.parent.clone())
  }
}

impl PartialEq for Scope {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.map.read().deref().eq(other.map.read().deref())
  }
}

impl Eq for Scope {}

impl PartialOrd for Scope {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Trace for Scope {
  #[inline]
  fn trace(&mut self, marked: bool) {
    for (_k, v) in self.map.write().iter_mut() {
      v.trace(marked);
    }
    self.parent.trace(marked);
  }
}

impl Hash for Scope {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl fmt::Debug for Scope {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('{')?;
    let map = self.map.read();
    let mut index = map.len();

    for (key, value) in map.iter() {
      write!(f, "{:?} {:?}", key, value)?;

      index -= 1;
      if index != 0 {
        write!(f, ", ")?;
      }
    }

    f.write_char('}')
  }
}

impl Scope {
  #[inline]
  pub fn new(map: HashMap<String, Gc<dyn Value>>, parent: Option<Gc<Object<Scope>>>) -> Self {
    Scope {
      map: RwLock::new(map),
      parent,
    }
  }

  #[inline]
  pub fn set(&self, key: &str, value: Gc<dyn Value>) -> &Self {
    self.map.write().insert(key.to_string(), value);
    self
  }

  #[inline]
  pub fn remove(&self, key: &str) -> &Self {
    self.map.write().remove(key);
    self
  }

  #[inline]
  pub fn has(&self, key: &str) -> bool {
    self.map.read().contains_key(key)
  }

  #[inline]
  pub fn get(&self, key: &str) -> Option<Gc<dyn Value>> {
    self.map.read().get(key).map(Clone::clone)
  }
}

#[inline]
pub fn scope_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Scope").expect("failed to get Scope Kind")
}
#[inline]
pub fn new_scope(parent: &Gc<Object<Scope>>) -> Gc<Object<Scope>> {
  new_object(
    parent,
    Object::new(
      scope_kind(parent).clone(),
      Scope::new(HashMap::default(), Some(parent.clone())),
    ),
  )
}

#[inline]
pub fn scope_parent(scope: &Gc<Object<Scope>>) -> Option<&Gc<Object<Scope>>> {
  scope.parent.as_ref()
}

#[inline]
pub fn get_scope_root(scope: &Gc<Object<Scope>>) -> &Gc<Object<Scope>> {
  if let Some(parent) = scope_parent(scope) {
    get_scope_root(parent)
  } else {
    scope
  }
}

#[inline]
pub fn scope_get_by_value<'a>(scope: &'a Gc<Object<Scope>>, ident: &str) -> Option<Gc<dyn Value>> {
  if let Some(value) = scope.get(ident) {
    return Some(value);
  } else if let Some(parent) = scope_parent(scope) {
    return scope_get_by_value(parent, ident);
  } else {
    None
  }
}

#[inline]
pub fn scope_get<'a>(scope: &'a Gc<Object<Scope>>, ident: &str) -> Option<Gc<dyn Value>> {
  scope_get_by_value(scope, ident)
}

#[inline]
pub fn scope_set<'a>(scope: &'a Gc<Object<Scope>>, ident: &str, value: Gc<dyn Value>) {
  scope.set(ident, value);
}

#[inline]
pub fn scope_get_with_kind<'a, T>(
  scope: &'a Gc<Object<Scope>>,
  ident: &str,
) -> Option<Gc<Object<T>>>
where
  T: 'static + Hash + Debug + PartialEq + PartialOrd + Trace,
{
  scope_get(scope, ident).and_then(|value| value.downcast_ref::<Object<T>>().map(Clone::clone))
}
