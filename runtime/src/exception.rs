use core::fmt::{self, Write};

use gc::Gc;

use super::Value;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Exception {
  value: Gc<Value>,
  scope: Gc<Object<List>>,
}

impl fmt::Debug for Exception {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('`')?;
        write!(f, "{:?}", &self.0)
    }
}

impl Into<Gc<Value>> for Exception {
    #[inline]
    fn into(self) -> Gc<Value> {
        self.0
    }
}

impl Exception {
    #[inline]
    pub fn new(value: Gc<Value>) -> Self {
        Exception(value)
    }

    #[inline]
    pub fn inner(&self) -> &Gc<Value> {
        &self.0
    }
    #[inline]
    pub fn inner_mut(&mut self) -> &mut Gc<Value> {
        &mut self.0
    }
}
