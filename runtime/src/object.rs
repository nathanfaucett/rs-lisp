use core::cmp::Ordering;
use core::fmt::{self, Debug, Display};
use core::hash::{Hash, Hasher};
use core::mem;
use core::ops::{Deref, DerefMut};

use gc::{Gc, Trace};

use super::{scope_get_mut_with_kind, GcAllocator, Kind, Map, PersistentScope, Value};

#[derive(Clone)]
pub struct Object<T> {
  pub(crate) marked: bool,
  pub(crate) kind: Gc<Object<Kind>>,
  pub(crate) meta: Option<Gc<Object<Map>>>,
  pub(crate) value: T,
}

impl<T> Object<T>
where
  T: 'static + PartialEq + PartialOrd + Hash + Debug + Trace,
{
  #[inline(always)]
  pub fn new(kind: Gc<Object<Kind>>, value: T) -> Self {
    Object {
      marked: false,
      kind: kind,
      meta: None,
      value: value,
    }
  }
  #[inline(always)]
  pub fn new_with_meta(kind: Gc<Object<Kind>>, value: T, meta: Gc<Object<Map>>) -> Self {
    Object {
      marked: false,
      kind: kind,
      meta: Some(meta),
      value: value,
    }
  }
}

impl<T> Object<T> {
  #[inline(always)]
  pub fn value(&self) -> &T {
    &self.value
  }
  #[inline(always)]
  pub fn value_mut(&mut self) -> &mut T {
    &mut self.value
  }
  #[inline(always)]
  pub fn meta(&self) -> Option<&Gc<Object<Map>>> {
    self.meta.as_ref()
  }
  #[inline(always)]
  pub fn meta_mut(&mut self) -> Option<&mut Gc<Object<Map>>> {
    self.meta.as_mut()
  }
  #[inline(always)]
  pub fn set_meta(&mut self, meta: Gc<Object<Map>>) -> &mut Self {
    self.meta.replace(meta);
    self
  }
}

impl<T> Deref for Object<T> {
  type Target = T;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<T> DerefMut for Object<T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.value
  }
}

impl<T> Object<T> {
  #[inline(always)]
  pub fn as_value<'a>(self: &'a Gc<Self>) -> &'a Gc<dyn Value> {
    unsafe { mem::transmute(self) }
  }
  #[inline(always)]
  pub fn as_value_mut<'a>(self: &'a mut Gc<Self>) -> &'a mut Gc<dyn Value> {
    unsafe { mem::transmute(self) }
  }
}

impl<T> Object<T>
where
  T: 'static + PartialEq + PartialOrd + Hash + Debug + Trace,
{
  #[inline(always)]
  pub fn into_value(self: Gc<Self>) -> Gc<dyn Value> {
    unsafe { Gc::from_raw(self.as_ptr() as *mut dyn Value) }
  }
}

impl<T> Value for Object<T>
where
  T: 'static + PartialEq + PartialOrd + Hash + Debug + Trace,
{
  #[inline(always)]
  fn kind(&self) -> &Gc<Object<Kind>> {
    &self.kind
  }

  #[inline(always)]
  fn meta(&self) -> Option<&Gc<Object<Map>>> {
    self.meta.as_ref()
  }

  #[inline(always)]
  fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Debug::fmt(&self.value, f)
  }

  #[inline(always)]
  fn compare(&self, other: &dyn Value) -> Option<Ordering> {
    match other.downcast_ref::<Object<T>>() {
      Some(other) => self.value().partial_cmp(other.value()),
      None => None,
    }
  }

  #[inline(always)]
  fn hash(&self, hasher: &mut dyn Hasher) {
    Hash::hash(self, &mut HasherMut(hasher));
  }

  #[inline(always)]
  fn trace(&mut self, marked: bool) {
    Trace::trace(self, marked);
  }

  #[inline(always)]
  fn is_marked(&self) -> bool {
    Trace::is_marked(self)
  }
}

impl<T> Ord for Object<T>
where
  T: 'static + Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.value().cmp(other.value())
  }
}

impl<T> Trace for Object<T>
where
  T: 'static + Trace,
{
  #[inline(always)]
  fn is_marked(&self) -> bool {
    self.marked
  }
  #[inline(always)]
  fn trace(&mut self, marked: bool) {
    if self.is_marked() != marked {
      self.marked = marked;
      self.kind.trace(marked);
      self.value.trace(marked);
    }
  }
}

impl<T> Hash for Object<T>
where
  T: Hash,
{
  #[inline(always)]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value.hash(state);
  }
}

impl<T> PartialOrd for Object<T>
where
  T: PartialOrd,
{
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value().partial_cmp(other.value())
  }
}

impl<T> PartialEq for Object<T>
where
  T: PartialEq,
{
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.value().eq(other.value())
  }
}

impl<T> Eq for Object<T> where T: Eq {}

impl<T> Debug for Object<T>
where
  T: Debug,
{
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Debug::fmt(&self.value, f)
  }
}

impl<T> Display for Object<T>
where
  T: Display,
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

#[inline]
pub fn new_object<T>(scope: &Gc<Object<PersistentScope>>, object: Object<T>) -> Gc<Object<T>>
where
  T: PartialEq + PartialOrd + Hash + Debug + Trace + 'static,
{
  scope_get_mut_with_kind::<GcAllocator>(scope, "default_gc_allocator")
    .expect("failed to get default_gc_allocator")
    .alloc(object)
}
