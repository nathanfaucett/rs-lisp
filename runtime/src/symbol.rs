use alloc::string::{String, ToString};
use core::fmt;
use core::ops::{Deref, DerefMut};

use gc::{Gc, Trace};

use super::{new_kind, new_object, scope_get_with_kind, scope_set, Kind, Map, Object, Scope};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(String);

impl Trace for Symbol {
  #[inline]
  fn trace(&mut self, _marked: bool) {}
}

impl fmt::Debug for Symbol {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl fmt::Display for Symbol {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl Into<String> for Symbol {
  #[inline]
  fn into(self) -> String {
    self.0
  }
}

impl Deref for Symbol {
  type Target = String;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Symbol {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Symbol {
  #[inline]
  pub fn new(value: String) -> Self {
    Symbol(value)
  }

  #[inline]
  pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
    let symbol_kind = new_kind::<Symbol>(scope, "Symbol");
    scope_set(scope, "Symbol", symbol_kind.into_value());
  }
}

#[inline]
pub fn symbol_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Symbol").expect("failed to get Symbol Kind")
}
#[inline]
pub fn new_symbol<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<Symbol>>
where
  T: ToString,
{
  new_symbol_with_meta(scope, value, None)
}
#[inline]
pub fn new_symbol_with_meta<T>(
  scope: &Gc<Object<Scope>>,
  value: T,
  meta: Option<Gc<Object<Map>>>,
) -> Gc<Object<Symbol>>
where
  T: ToString,
{
  new_object(
    scope,
    Object::new_with_meta(
      symbol_kind(scope).clone(),
      Symbol::new(value.to_string()),
      meta,
    ),
  )
}
