use std::collections::LinkedList;
use std::fmt;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{Object, Scope, Value};

pub enum FunctionKind {
    Internal(Gc<Object<LinkedList<Gc<Value>>>>),
    Rust(Box<Fn(Gc<Object<Scope>>, Gc<Object<LinkedList<Gc<Value>>>>) -> Gc<Value>>),
}

impl FunctionKind {
    #[inline]
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<LinkedList<Gc<Value>>>>) -> Gc<Value>,
    {
        FunctionKind::Rust(Box::new(f))
    }
}

impl fmt::Debug for FunctionKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FunctionKind").finish()
    }
}

impl Hash for FunctionKind {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FunctionKind::Internal(body) => {
                body.hash(state);
            }
            FunctionKind::Rust(f) => {
                (f.as_ref()
                    as *const Fn(Gc<Object<Scope>>, Gc<Object<LinkedList<Gc<Value>>>>)
                        -> Gc<Value>)
                    .hash(state);
            }
        }
    }
}

impl PartialEq for FunctionKind {
    #[inline]
    fn eq(&self, other: &FunctionKind) -> bool {
        match self {
            FunctionKind::Internal(body) => match other {
                FunctionKind::Internal(other_body) => body == other_body,
                _ => false,
            },
            FunctionKind::Rust(f) => match other {
                FunctionKind::Rust(other_f) => ::std::ptr::eq(f.as_ref(), other_f.as_ref()),
                _ => false,
            },
        }
    }
}

impl Eq for FunctionKind {}
