use std::fmt::{self, Write};

use gc::Gc;

use super::Value;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Escape(Gc<Value>);

impl fmt::Debug for Escape {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('`')?;
        write!(f, "{:?}", &self.0)
    }
}

impl Into<Gc<Value>> for Escape {
    #[inline]
    fn into(self) -> Gc<Value> {
        self.0
    }
}

impl Escape {
    #[inline]
    pub fn new(value: Gc<Value>) -> Self {
        Escape(value)
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
