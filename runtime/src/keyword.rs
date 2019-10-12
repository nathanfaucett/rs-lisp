use alloc::string::{String, ToString};
use core::fmt::{self, Write};

use gc::{Gc, Trace};

use super::{new_kind, new_object, Kind, Object, Scope};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyword(String);

impl Trace for Keyword {
  #[inline]
  fn trace(&mut self, _marked: bool) {}
}

impl fmt::Debug for Keyword {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char(':')?;
    f.write_str(&self.0)
  }
}

impl fmt::Display for Keyword {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char(':')?;
    f.write_str(&self.0)
  }
}

impl Into<String> for Keyword {
  #[inline]
  fn into(self) -> String {
    self.0
  }
}

impl Keyword {
  #[inline]
  pub fn new(value: String) -> Self {
    Keyword(value)
  }

  #[inline]
  pub fn inner(&self) -> &String {
    &self.0
  }
  #[inline]
  pub fn inner_mut(&mut self) -> &mut String {
    &mut self.0
  }

  #[inline]
  pub(crate) unsafe fn init_kind(mut scope: Gc<Object<Scope>>) {
    let keyword_kind = new_kind::<Keyword>(scope.clone(), "Keyword");
    scope.set("Keyword", keyword_kind.into_value());
  }
}

#[inline]
pub fn keyword_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Keyword")
      .expect("failed to get Keyword Kind")
  }
}
#[inline]
pub fn new_keyword<T>(scope: Gc<Object<Scope>>, value: T) -> Gc<Object<Keyword>>
where
  T: ToString,
{
  new_object(
    scope.clone(),
    Object::new(keyword_kind(scope), Keyword::new(value.to_string())),
  )
}
