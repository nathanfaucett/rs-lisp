use std::mem;

use gc::Gc;

use super::Object;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kind {
    name: String,
    size: usize,
    align: usize,
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
    pub unsafe fn new_type_kind() -> Gc<Object<Kind>> {
        let mut kind = Gc::new(Object::new(
            Gc::null(),
            Kind::new(
                "Type".into(),
                mem::size_of::<Kind>(),
                mem::align_of::<Kind>(),
            ),
        ));
        kind.kind = kind.clone();
        kind
    }

    #[inline(always)]
    pub fn new_kind<T>(kind: Gc<Object<Self>>, name: &str) -> Object<Self> {
        Object::new(
            kind,
            Kind::new(name.into(), mem::size_of::<T>(), mem::align_of::<T>()),
        )
    }
}
