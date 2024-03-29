use alloc::boxed::Box;

use core::any::{Any, TypeId};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::{mem, ptr};

use super::Trace;

pub struct Gc<T>
where
  T: ?Sized,
{
  ptr: *mut T,
}

unsafe impl<T> Send for Gc<T> {}
unsafe impl<T> Sync for Gc<T> {}

impl<T> Trace for Gc<T>
where
  T: Trace,
{
  fn trace(&mut self, marked: bool) {
    self.as_mut().trace(marked)
  }
}

impl<T> Gc<T> {
  #[inline(always)]
  pub const unsafe fn null() -> Self {
    Gc {
      ptr: ptr::null_mut::<T>(),
    }
  }
  #[inline(always)]
  pub unsafe fn new(value: T) -> Self {
    Gc {
      ptr: Box::into_raw(Box::new(value)),
    }
  }

  #[inline(always)]
  pub fn is_null(&self) -> bool {
    self.ptr.is_null()
  }

  #[inline(always)]
  pub unsafe fn set_from_value(&mut self, v: T) -> &mut Self {
    assert!(self.is_null());
    self.ptr = Box::into_raw(Box::new(v));
    self
  }

  #[inline(always)]
  pub unsafe fn set_from_ptr(&mut self, ptr: *mut T) -> &mut Self {
    assert!(self.is_null());
    self.ptr = ptr;
    self
  }

  #[inline(always)]
  pub unsafe fn set_from_gc(&mut self, g: Gc<T>) -> &mut Self {
    assert!(self.is_null());
    self.ptr = g.as_ptr();
    self
  }
}

impl<T> Gc<T>
where
  T: ?Sized,
{
  #[inline(always)]
  pub unsafe fn from_raw(ptr: *mut T) -> Self {
    Gc { ptr }
  }
  #[inline(always)]
  pub unsafe fn from_box(b: Box<T>) -> Self {
    Gc {
      ptr: Box::into_raw(b),
    }
  }
  #[inline(always)]
  pub unsafe fn from_gc(g: Gc<T>) -> Self {
    Gc { ptr: g.as_ptr() }
  }

  #[inline(always)]
  pub unsafe fn into_raw(self) -> *mut T {
    self.ptr
  }
  #[inline(always)]
  pub unsafe fn into_box(self) -> Box<T> {
    Box::from_raw(self.ptr)
  }

  #[inline(always)]
  pub fn as_ptr(&self) -> *mut T {
    self.ptr
  }
  #[inline(always)]
  pub fn as_ref(&self) -> &T {
    unsafe { &*self.as_ptr() }
  }
  #[inline(always)]
  pub fn as_mut(&mut self) -> &mut T {
    unsafe { &mut *self.as_ptr() }
  }

  #[inline(always)]
  pub unsafe fn unsafe_as_mut(&self) -> &mut T {
    &mut *self.as_ptr()
  }
  
  #[inline(always)]
  pub unsafe fn unsafe_drop(self) {
    ptr::drop_in_place(self.as_ptr());
  }
}

impl<T> Gc<T>
where
  T: Any + ?Sized,
{
  #[inline(always)]
  pub fn is<V: Any + ?Sized>(&self) -> bool {
    TypeId::of::<V>() == Any::type_id(self.as_ref())
  }

  #[inline(always)]
  pub unsafe fn downcast_ref_unchecked<V: Any>(&self) -> &Gc<V> {
    &*(self as *const dyn Any as *const Gc<V>)
  }
  #[inline]
  pub fn downcast_ref<V: Any>(&self) -> Option<&Gc<V>> {
    if self.is::<V>() {
      unsafe { Some(self.downcast_ref_unchecked()) }
    } else {
      None
    }
  }

  #[inline(always)]
  pub unsafe fn downcast_mut_unchecked<V: Any>(&mut self) -> &mut Gc<V> {
    &mut *(self as *mut dyn Any as *mut Gc<V>)
  }
  #[inline]
  pub fn downcast_mut<V: Any>(&mut self) -> Option<&mut Gc<V>> {
    if self.is::<V>() {
      unsafe { Some(self.downcast_mut_unchecked()) }
    } else {
      None
    }
  }
}

impl<T> Gc<T>
where
  T: Any,
{
  #[inline(always)]
  pub unsafe fn downcast_unchecked<V: Any>(self: Gc<T>) -> Gc<V> {
    mem::transmute(self)
  }
  #[inline]
  pub fn downcast<V: Any>(self: Gc<T>) -> Result<Gc<V>, Gc<T>> {
    if self.is::<T>() {
      unsafe { Ok(self.downcast_unchecked()) }
    } else {
      Err(self)
    }
  }
}

impl<T> Deref for Gc<T>
where
  T: ?Sized,
{
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T> DerefMut for Gc<T>
where
  T: ?Sized,
{
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}

impl<T> PartialOrd for Gc<T>
where
  T: ?Sized + PartialOrd,
{
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.as_ref().partial_cmp(other.as_ref())
  }
}

impl<T> Ord for Gc<T>
where
  T: Ord + ?Sized,
{
  #[inline(always)]
  fn cmp(&self, other: &Self) -> Ordering {
    self.as_ref().cmp(other.as_ref())
  }
}

impl<T> PartialEq for Gc<T>
where
  T: PartialEq + ?Sized,
{
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.as_ref().eq(other.as_ref())
  }
}

impl<T> Eq for Gc<T> where T: Eq + ?Sized {}

impl<T> Hash for Gc<T>
where
  T: Hash + ?Sized,
{
  #[inline(always)]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_ref().hash(state)
  }
}

impl<T> Debug for Gc<T>
where
  T: Debug + ?Sized,
{
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<T> Display for Gc<T>
where
  T: Display + ?Sized,
{
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<T> Clone for Gc<T>
where
  T: ?Sized,
{
  #[inline(always)]
  fn clone(&self) -> Self {
    Gc { ptr: self.ptr }
  }
}
