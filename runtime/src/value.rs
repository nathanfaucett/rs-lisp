use core::any::{Any, TypeId};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};

use gc::Gc;

use super::{
    add_external_function, false_value, new_bool, nil_value, Kind, Map, Object, Scope, Vector,
};

pub trait Value: Any {
    fn kind(&self) -> Gc<Object<Kind>>;
    fn meta(&self) -> Option<Gc<Object<Map>>>;
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn compare(&self, other: &dyn Value) -> Option<Ordering>;
    fn hash(&self, hasher: &mut dyn Hasher);
    fn is_marked(&self) -> bool;
    fn trace(&mut self, marked: bool);
    fn mark(&mut self, marked: bool);
}

impl dyn Value {
    #[inline]
    pub fn is<T: Value>(&self) -> bool {
        TypeId::of::<T>() == Any::type_id(self)
    }

    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: Value>(&self) -> &T {
        &*(self as *const dyn Value as *const T)
    }
    #[inline]
    pub fn downcast_ref<T: Value>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(self.downcast_ref_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    pub unsafe fn downcast_mut_unchecked<T: Value>(&mut self) -> &mut T {
        &mut *(self as *mut dyn Value as *mut T)
    }
    #[inline]
    pub fn downcast_mut<T: Value>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(self.downcast_mut_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
        add_external_function(scope, "=", vec!["a", "b"], value_eq);
        add_external_function(scope, "!=", vec!["a", "b"], value_ne);
        add_external_function(scope, ">", vec!["a", "b"], value_gt);
        add_external_function(scope, ">=", vec!["a", "b"], value_ge);
        add_external_function(scope, "<", vec!["a", "b"], value_lt);
        add_external_function(scope, "<=", vec!["a", "b"], value_le);
        add_external_function(scope, "!", vec!["a", "b"], value_not);
    }
}

#[inline]
pub fn value_eq(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let a = args
        .get(0)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());
    let b = args
        .get(1)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    new_bool(scope, a == b).into_value()
}

#[inline]
pub fn value_ne(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let a = args
        .get(0)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());
    let b = args
        .get(1)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    new_bool(scope, a != b).into_value()
}

#[inline]
pub fn value_gt(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let a = args
        .get(0)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());
    let b = args
        .get(1)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    new_bool(scope, a > b).into_value()
}

#[inline]
pub fn value_ge(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let a = args
        .get(0)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());
    let b = args
        .get(1)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    new_bool(scope, a >= b).into_value()
}

#[inline]
pub fn value_lt(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let a = args
        .get(0)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());
    let b = args
        .get(1)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    new_bool(scope, a < b).into_value()
}

#[inline]
pub fn value_le(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let a = args
        .get(0)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());
    let b = args
        .get(1)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    new_bool(scope, a <= b).into_value()
}

#[inline]
pub fn value_not(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let value = args
        .get(0)
        .and_then(|value| value.downcast_ref::<Object<bool>>().map(Clone::clone))
        .unwrap_or_else(|| false_value(scope).clone());

    new_bool(scope, !value.value()).into_value()
}

impl PartialOrd for dyn Value {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind() == other.kind() {
            self.compare(other)
        } else {
            None
        }
    }
}

impl PartialEq for dyn Value {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .map(|ordering| ordering == Ordering::Equal)
            .unwrap_or(false)
    }
}

impl Eq for dyn Value {}

impl Hash for dyn Value {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state);
    }
}

impl fmt::Debug for dyn Value {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug(f)
    }
}
