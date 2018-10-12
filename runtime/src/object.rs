use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use gc::Gc;

use super::{Kind, Value};

#[derive(Clone)]
pub struct Object<T>
where
    T: PartialEq + Hash + Debug,
{
    pub(crate) kind: Gc<Object<Kind>>,
    pub(crate) value: T,
}

impl<T> Object<T>
where
    T: PartialEq + Hash + Debug,
{
    #[inline(always)]
    pub fn new(kind: Gc<Object<Kind>>, value: T) -> Self {
        Object {
            kind: kind,
            value: value,
        }
    }

    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.value
    }
    #[inline(always)]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Deref for Object<T>
where
    T: PartialEq + Hash + Debug,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Object<T>
where
    T: PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Object<T>
where
    T: 'static + PartialEq + Hash + Debug,
{
    #[inline(always)]
    pub fn into_value(self: Gc<Self>) -> Gc<Value> {
        unsafe { Gc::from_raw(self.as_ptr() as *mut Value) }
    }
}

impl<T> Value for Object<T>
where
    T: 'static + PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn kind(&self) -> &Gc<Object<Kind>> {
        &self.kind
    }

    #[inline(always)]
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.value, f)
    }

    #[inline(always)]
    fn equal(&self, other: &Value) -> bool {
        match other.downcast_ref::<Object<T>>() {
            Some(other) => (self.kind() == other.kind() && self.value() == other.value()),
            None => false,
        }
    }

    #[inline(always)]
    fn hash(&self, hasher: &mut Hasher) {
        Hash::hash(self, &mut HasherMut(hasher));
    }
}

impl<T> PartialOrd for Object<T>
where
    T: PartialOrd + PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(other.value())
    }
}

impl<T> Ord for Object<T>
where
    T: Ord + PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(other.value())
    }
}

impl<T> PartialEq for Object<T>
where
    T: PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(other.value())
    }
}

impl<T> Eq for Object<T> where T: Eq + PartialEq + Hash + Debug {}

impl<T> Hash for Object<T>
where
    T: PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(self.value(), state);
    }
}

impl<T> Debug for Object<T>
where
    T: PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<T> Display for Object<T>
where
    T: Display + PartialEq + Hash + Debug,
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
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
