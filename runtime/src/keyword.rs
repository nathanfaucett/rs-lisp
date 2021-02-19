use alloc::string::{String, ToString};
use core::fmt::{self, Write};
use core::ops::{Deref, DerefMut};

use gc::{Gc, Trace};

use super::{new_kind, new_object, scope_get_with_kind, scope_set, Kind, Map, Object, Scope};

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

impl Deref for Keyword {
    type Target = String;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Keyword {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Keyword {
    #[inline]
    pub fn new(value: String) -> Self {
        Keyword(value)
    }

    #[inline]
    pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
        let keyword_kind = new_kind::<Keyword>(scope, "Keyword");
        scope_set(scope, "Keyword", keyword_kind.into_value());
    }
}

#[inline]
pub fn keyword_kind(scope: &Gc<Object<Scope>>) -> &Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Keyword").expect("failed to get Keyword Kind")
}
#[inline]
pub fn new_keyword<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<Keyword>>
where
    T: ToString,
{
    new_keyword_with_meta(scope, value, None)
}
#[inline]
pub fn new_keyword_with_meta<T>(
    scope: &Gc<Object<Scope>>,
    value: T,
    meta: Option<Gc<Object<Map>>>,
) -> Gc<Object<Keyword>>
where
    T: ToString,
{
    new_object(
        scope,
        Object::new_with_meta(
            keyword_kind(scope).clone(),
            Keyword::new(value.to_string()),
            meta,
        ),
    )
}
