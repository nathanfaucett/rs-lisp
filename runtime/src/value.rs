use std::any::{Any, TypeId};
use std::fmt;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{Kind, Object};

pub trait Value: Any {
    fn kind(&self) -> &Gc<Object<Kind>>;
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn equal(&self, other: &Value) -> bool;
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
    pub unsafe fn downcast_unchecked<T: Value>(mut self: Gc<Self>) -> Gc<T> {
        Gc::from_raw((&mut *self) as *const Value as *mut T)
    }
    #[inline]
    pub fn downcast<T: Value>(self: Gc<Self>) -> Result<Gc<T>, Gc<Self>> {
        if self.is::<T>() {
            unsafe { Ok(self.downcast_unchecked()) }
        } else {
            Err(self)
        }
    }
}

impl Value {
    #[inline(always)]
    pub unsafe fn into_object_unchecked<T>(self: Gc<Self>) -> Gc<Object<T>>
    where
        T: 'static + PartialEq + Hash + fmt::Debug,
    {
        self.downcast_unchecked::<Object<T>>()
    }
    #[inline(always)]
    pub fn into_object<T>(self: Gc<Self>) -> Result<Gc<Object<T>>, Gc<Self>>
    where
        T: 'static + PartialEq + Hash + fmt::Debug,
    {
        self.downcast::<Object<T>>()
    }
}

impl PartialEq for Value {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
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

// https://github.com/Rufflewind/any_key/blob/master/src/lib.rs#L40
pub struct HasherMut<H: ?Sized + Hasher>(pub H);

impl<H> Hasher for HasherMut<H>
where
    H: ?Sized + Hasher,
{
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}
