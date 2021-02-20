use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr;

use gc::{Gc, Trace};
use libloading::{Error, Library, Symbol};
use runtime::{
    add_external_function, new_kind, new_object, scope_get_with_kind, scope_set, Keyword, Kind,
    Object, Scope, Value, Vector,
};

pub type DyLibFunction =
    unsafe extern "C" fn(&Gc<Object<Scope>>, &Gc<Object<Vector>>) -> Gc<dyn Value>;

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
    pub unsafe fn new<T>(path: T) -> Self
    where
        T: ToString,
    {
        DyLib {
            library: Library::new(path.to_string())
                .expect(&format!("failed to find dylib {}", path.to_string())),
        }
    }

    #[inline]
    pub unsafe fn get<'a, T>(&'a self, symbol: &str) -> Result<Symbol<'a, T>, Error> {
        self.library.get::<T>(symbol.as_bytes())
    }

    #[inline]
    pub unsafe fn call(
        &self,
        name: &str,
        scope: &Gc<Object<Scope>>,
        args: &Gc<Object<Vector>>,
    ) -> Result<Gc<dyn Value>, Error> {
        let func = self.get::<DyLibFunction>(name)?;
        Ok(func(scope, args))
    }

    #[inline]
    pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
        let dylib_kind = new_kind::<DyLib>(scope, "DyLib");
        scope_set(scope, "DyLib", dylib_kind.into_value());
    }

    #[inline]
    pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
        add_external_function(
            scope,
            "dylib.call",
            vec!["dylib", "name", "...args"],
            dylib_call,
        );
    }
}

#[inline]
pub fn dylib_call(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let dylib = args
        .get(0)
        .expect("DyLib is nil")
        .downcast_ref::<Object<DyLib>>()
        .expect("Failed to downcast dylib to DyLib")
        .clone();
    let name = args
        .get(1)
        .expect("name is nil")
        .downcast_ref::<Object<Keyword>>()
        .expect("Failed to downcast name to Keyword")
        .clone();

    unsafe {
        dylib
            .call(name.deref(), scope, args)
            .expect(&format!("Failed to call dylib function {}", name.deref()))
    }
}

#[inline]
pub fn dylib_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "DyLib").expect("failed to get DyLib Kind")
}
#[inline]
pub unsafe fn new_dylib<T>(scope: &Gc<Object<Scope>>, path: T) -> Gc<Object<DyLib>>
where
    T: ToString,
{
    new_object(
        scope,
        Object::new(dylib_kind(scope).clone(), DyLib::new(path)),
    )
}
