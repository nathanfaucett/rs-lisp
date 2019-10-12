use core::any::{Any, TypeId};
use core::fmt;
use core::hash::{Hash, Hasher};

use gc::{Gc, Trace};

use super::{Kind, Object};

pub trait Value: Any {
  fn kind(&self) -> &Gc<Object<Kind>>;
  fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
  fn equal(&self, other: &dyn Value) -> bool;
  fn hash(&self, hasher: &mut dyn Hasher);
  fn trace(&mut self, marked: bool);
  fn is_marked(&self) -> bool;
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
  pub unsafe fn downcast_unchecked<T: Value>(mut self: Gc<Self>) -> Gc<T> {
    Gc::from_raw((&mut *self) as *const dyn Value as *mut T)
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

impl dyn Value {
  #[inline(always)]
  pub unsafe fn into_object_unchecked<T>(self: Gc<Self>) -> Gc<Object<T>>
  where
    T: 'static + PartialEq + Hash + fmt::Debug + Trace,
  {
    self.downcast_unchecked::<Object<T>>()
  }
  #[inline(always)]
  pub fn into_object<T>(self: Gc<Self>) -> Result<Gc<Object<T>>, Gc<Self>>
  where
    T: 'static + PartialEq + Hash + fmt::Debug + Trace,
  {
    self.downcast::<Object<T>>()
  }
}

impl PartialEq for dyn Value {
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.equal(other)
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
