use alloc::string::{String, ToString};
use core::hash::{Hash, Hasher};
use core::{fmt, mem, ptr};

use gc::{Gc, Trace};

use super::{
    add_external_function, new_object, new_string, new_usize, nil_value, scope_get_with_kind,
    LispMap, Object, Scope, Value, Vector,
};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Kind {
    name: String,
    size: usize,
    align: usize,
}

impl fmt::Debug for Kind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = LispMap::default();

        map.0.insert("name", self.name.clone());
        map.0.insert("size", self.size.to_string());
        map.0.insert("align", self.align.to_string());

        f.debug_tuple("").field(&"Kind").field(&map).finish()
    }
}

impl Trace for Kind {
    #[inline]
    fn trace(&mut self, _marked: bool) {}
}

impl Hash for Kind {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl Kind {
    #[inline(always)]
    pub fn new(name: String, size: usize, align: usize) -> Self {
        Kind {
            name: name,
            size: size,
            align: align,
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn new_kind_kind() -> Gc<Object<Kind>> {
        let mut kind = Gc::new(Object::new(
            Gc::null(),
            Kind::new(
                "Kind".into(),
                mem::size_of::<Kind>(),
                mem::align_of::<Kind>(),
            ),
        ));
        kind.kind = kind.clone();
        kind
    }

    #[inline(always)]
    pub fn new_kind_object<T>(kind: Gc<Object<Self>>, name: &str) -> Object<Self> {
        Object::new(
            kind,
            Kind::new(name.into(), mem::size_of::<T>(), mem::align_of::<T>()),
        )
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
    #[inline]
    pub fn align(&self) -> usize {
        self.align
    }

    #[inline]
    pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
        add_external_function(scope, "kind.of", vec!["value"], kind_of);
        add_external_function(scope, "kind.name", vec!["value"], kind_name);
        add_external_function(scope, "kind.size", vec!["value"], kind_size);
        add_external_function(scope, "kind.align", vec!["value"], kind_align);
    }
}

#[inline]
pub fn kind_of(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    args.front()
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value())
        .kind()
        .clone()
        .into_value()
}

#[inline]
pub fn kind_name(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    new_string(
        scope,
        args.front()
            .map(Clone::clone)
            .unwrap_or_else(|| nil_value(scope).clone().into_value())
            .kind()
            .name(),
    )
    .into_value()
}

#[inline]
pub fn kind_size(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    new_usize(
        scope,
        args.front()
            .map(Clone::clone)
            .unwrap_or_else(|| nil_value(scope).clone().into_value())
            .kind()
            .size(),
    )
    .into_value()
}

#[inline]
pub fn kind_align(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    new_usize(
        scope,
        args.front()
            .map(Clone::clone)
            .unwrap_or_else(|| nil_value(scope).clone().into_value())
            .kind()
            .align(),
    )
    .into_value()
}

#[inline]
pub fn kind_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Kind").expect("failed to get Kind Kind")
}
#[inline]
pub fn new_kind<T>(scope: &Gc<Object<Scope>>, name: &str) -> Gc<Object<Kind>> {
    new_object(
        scope,
        Kind::new_kind_object::<T>(kind_kind(scope).clone(), name),
    )
}
