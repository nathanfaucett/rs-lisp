use core::fmt::{self, Write};

use gc::{Gc, Trace};

use super::{new_object, Kind, Object, Scope, Value};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Escape(Gc<dyn Value>);

impl Trace for Escape {
  #[inline]
  fn trace(&mut self, marked: bool) {
    self.0.trace(marked);
  }
}

impl fmt::Debug for Escape {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('`')?;
    write!(f, "{:?}", &self.0)
  }
}

impl Into<Gc<dyn Value>> for Escape {
  #[inline]
  fn into(self) -> Gc<dyn Value> {
    self.0
  }
}

impl Escape {
  #[inline]
  pub fn new(value: Gc<dyn Value>) -> Self {
    Escape(value)
  }

  #[inline]
  pub fn inner(&self) -> &Gc<dyn Value> {
    &self.0
  }
  #[inline]
  pub fn inner_mut(&mut self) -> &mut Gc<dyn Value> {
    &mut self.0
  }
}

#[inline]
pub fn escape_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Escape")
      .expect("failed to get Escape Kind")
  }
}
#[inline]
pub fn new_escape(scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<Object<Escape>> {
  new_object(
    scope.clone(),
    Object::new(escape_kind(scope), Escape::new(value)),
  )
}
