use std::any::{Any, TypeId};
use std::fmt;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{Kind, Object};

pub trait Value: Any {
    fn kind(&self) -> &Object<Kind>;
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn eq(&self, other: &Value) -> bool;
    fn hash(&self, hasher: &mut Hasher);
}

impl Value {
    #[inline]
    pub fn is<T: Value>(&self) -> bool {
        TypeId::of::<T>() == Any::get_type_id(self)
    }

    #[inline]
    pub unsafe fn downcast_ref_unchecked<T: Value>(&self) -> &T {
        &*(self as *const Value as *const T)
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
        &mut *(self as *mut Value as *mut T)
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
    pub unsafe fn downcast_unchecked<T: Value>(mut self: Box<Self>) -> Box<T> {
        Box::from_raw((&mut *self) as *const Value as *mut T)
    }
    #[inline]
    pub fn downcast<T: Value>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.is::<T>() {
            unsafe { Ok(self.downcast_unchecked()) }
        } else {
            Err(self)
        }
    }
}

impl Value {
    #[inline]
    pub unsafe fn into_object_unchecked<T>(self: Gc<Value>) -> Gc<Object<T>>
    where
        T: 'static + PartialEq + Hash + fmt::Debug,
    {
        Gc::from_raw(self.as_ptr() as *const Value as *mut Object<T>)
    }
}

impl PartialEq for Value {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for Value {}

impl Hash for Value {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state);
    }
}

impl fmt::Debug for Value {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::ptr;

    use gc::Gc;

    #[test]
    fn test_is() {
        let kind = unsafe { Kind::new_type_kind() };
        let obj = unsafe { Gc::new(Object::new(kind, 0_usize)) };
        let value = obj.as_value();
        assert!(value.is::<Object<usize>>());
    }

    #[test]
    fn test_downcast() {
        let kind = unsafe { Kind::new_type_kind() };
        let obj = unsafe { Gc::new(Object::new(kind, 0_usize)) };
        let value = obj.as_value();
        assert_eq!(
            unsafe { value.downcast_ref_unchecked::<Object<usize>>() },
            obj.as_ref()
        );
    }

    #[test]
    fn test_as_value() {
        let kind = unsafe { Kind::new_type_kind() };
        let obj = unsafe { Gc::new(Object::new(kind.clone(), 0_usize)) };
        let value = obj.as_value();
        assert!(ptr::eq(kind.as_ptr(), value.kind()));
    }

    #[test]
    fn test_eq() {
        let kind = unsafe { Kind::new_type_kind() };
        let obj_a = unsafe { Gc::new(Object::new(kind.clone(), 0_usize)) };
        let obj_b = unsafe { Gc::new(Object::new(kind.clone(), 1_usize)) };
        let value_a = obj_a.as_value();
        let value_b = obj_b.as_value();
        assert_ne!(value_a, value_b);
        assert_eq!(value_a, value_a);
        assert_eq!(value_b, value_b);
    }
}
