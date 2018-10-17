use std::fmt;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{List, Object, Scope, Value};

pub enum FunctionKind {
    Internal(Gc<Value>),
    External(Box<Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>>),
}

impl Eq for FunctionKind {}

impl PartialEq for FunctionKind {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self {
            &FunctionKind::Internal(ref body) => match other {
                &FunctionKind::Internal(ref other_body) => body == other_body,
                _ => false,
            },
            &FunctionKind::External(ref func) => match other {
                &FunctionKind::External(ref other_func) => ::std::ptr::eq(func, other_func),
                _ => false,
            },
        }
    }
}

impl fmt::Debug for FunctionKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FunctionKind::Internal(ref body) => write!(f, "{:?}", body),
            &FunctionKind::External(_) => f.write_str(":external"),
        }
    }
}

impl Hash for FunctionKind {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &FunctionKind::Internal(ref body) => body.hash(state),
            &FunctionKind::External(ref func) => {
                (&**func as *const Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>).hash(state)
            }
        }
    }
}

impl FunctionKind {
    #[inline]
    pub fn new_internal(body: Gc<Value>) -> Self {
        FunctionKind::Internal(body)
    }
    #[inline]
    pub fn new_external<F>(body: F) -> Self
    where
        F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
    {
        FunctionKind::External(Box::new(body))
    }
}
