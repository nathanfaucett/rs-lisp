use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ptr;

use gc::{Gc, Trace};
use libloading::{Library, Result, Symbol};
use runtime::{
  add_external_function, new_kind, new_object, Keyword, Kind, List, Object, Scope, Value,
};

pub type DyLibFunction = unsafe extern "C" fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>;

pub struct DyLib {
  library: Library,
}

impl Trace for DyLib {}

impl PartialEq for DyLib {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    ptr::eq(self, other)
  }
}

impl Eq for DyLib {}

impl PartialOrd for DyLib {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Hash for DyLib {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state);
  }
}

impl fmt::Debug for DyLib {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", &self.library)
  }
}

impl DyLib {
  #[inline]
  pub fn new<T>(path: T) -> Self
  where
    T: ToString,
  {
    DyLib {
      library: Library::new(path.to_string())
        .expect(&format!("failed to find dylib {}", path.to_string())),
    }
  }

  #[inline]
  pub unsafe fn get<'a, T>(&'a self, symbol: &str) -> Result<Symbol<'a, T>> {
    self.library.get::<T>(symbol.as_bytes())
  }

  #[inline]
  pub unsafe fn call(
    &self,
    name: &str,
    scope: Gc<Object<Scope>>,
    args: Gc<Object<List>>,
  ) -> Result<Gc<dyn Value>> {
    let func = self.get::<DyLibFunction>(name)?;
    Ok(func(scope, args))
  }

  #[inline]
  pub(crate) fn init_kind(mut scope: Gc<Object<Scope>>) {
    let dylib_kind = new_kind::<DyLib>(scope.clone(), "DyLib");
    scope.set("DyLib", dylib_kind.into_value());
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope.clone(),
      "dylib.call",
      vec!["dylib", "name", "...args"],
      dylib_call,
    );
  }
}

#[inline]
pub fn dylib_call(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let dylib = args
    .pop_front()
    .expect("DyLib is nil")
    .downcast::<Object<DyLib>>()
    .expect("Failed to downcast dylib to DyLib");
  let name = args
    .pop_front()
    .expect("name is nil")
    .downcast::<Object<Keyword>>()
    .expect("Failed to downcast name to Keyword");

  unsafe {
    dylib
      .call(name.inner(), scope, args)
      .expect(&format!("Failed to call dylib function {}", name.inner()))
  }
}

#[inline]
pub fn dylib_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("DyLib")
      .expect("failed to get DyLib Kind")
  }
}
#[inline]
pub fn new_dylib<T>(scope: Gc<Object<Scope>>, path: T) -> Gc<Object<DyLib>>
where
  T: ToString,
{
  new_object(
    scope.clone(),
    Object::new(dylib_kind(scope), DyLib::new(path)),
  )
}
