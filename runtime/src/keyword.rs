use alloc::string::String;
use core::fmt::{self, Write};

use gc::Trace;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keyword(String);

impl Trace for Keyword {
  #[inline]
  fn mark(&mut self) {}
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
}
