use core::fmt::{self, Write};
use core::ops::{Deref, DerefMut};

use gc::{Gc, Trace};

use super::{new_kind, new_object, scope_get_with_kind, scope_set, Kind, Object, Scope, Value};

#[derive(Clone, PartialEq, PartialOrd, Eq, Hash)]
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

impl Deref for Escape {
    type Target = Gc<dyn Value>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Escape {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Escape {
    #[inline]
    pub fn new(value: Gc<dyn Value>) -> Self {
        Escape(value)
    }

    #[inline]
    pub fn escape_value(&self) -> &Gc<dyn Value> {
        &self.0
    }

    #[inline]
    pub(crate) unsafe fn init_kind(scope: &Gc<Object<Scope>>) {
        let escape_kind = new_kind::<Escape>(scope, "Escape");
        scope_set(scope, "Escape", escape_kind.into_value());
    }
}

#[inline]
pub fn escape_kind(scope: &Gc<Object<Scope>>) -> &Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Escape").expect("failed to get Escape Kind")
}
#[inline]
pub fn new_escape(scope: &Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<Object<Escape>> {
    new_object(
        scope,
        Object::new(escape_kind(scope).clone(), Escape::new(value)),
    )
}
