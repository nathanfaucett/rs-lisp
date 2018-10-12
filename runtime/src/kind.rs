use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;

use gc::Gc;

use super::{Object, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kind {
    name: String,
    size: usize,
    align: usize,
    data: HashMap<Gc<Value>, Gc<Value>>,
}

impl Hash for Kind {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.size.hash(state);
        self.align.hash(state);
    }
}

impl Kind {
    #[inline(always)]
    pub fn new(name: String, size: usize, align: usize) -> Self {
        Kind {
            name: name,
            size: size,
            align: align,
            data: HashMap::new(),
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

    #[inline]
    pub fn get(&self, key: &Gc<Value>) -> Option<&Gc<Value>> {
        self.data.get(key)
    }

    #[inline]
    pub fn get_mut(&mut self, key: &Gc<Value>) -> Option<&mut Gc<Value>> {
        self.data.get_mut(key)
    }

    #[inline]
    pub fn set(&mut self, key: Gc<Value>, value: Gc<Value>) -> &mut Self {
        self.data.insert(key, value);
        self
    }
}
