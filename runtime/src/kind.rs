use alloc::string::String;
use core::hash::{Hash, Hasher};
use core::{mem, ptr};

use gc::{Gc, Trace};

use super::{new_object, Object, Scope};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Kind {
  name: String,
  size: usize,
  align: usize,
}

impl Trace for Kind {
  #[inline]
  fn trace(&mut self, _marked: bool) {}
}

impl Hash for Kind {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl Kind {
  #[inline(always)]
  pub fn new(name: String, size: usize, align: usize) -> Self {
    Kind {
      name: name,
      size: size,
      align: align,
    }
  }

  #[inline(always)]
  pub(crate) unsafe fn new_kind_kind() -> Gc<Object<Kind>> {
    let mut kind = Gc::new(Object::new(
      Gc::null(),
      Kind::new(
        "Kind".into(),
        mem::size_of::<Kind>(),
        mem::align_of::<Kind>(),
      ),
    ));
    kind.kind = kind.clone();
    kind
  }

  #[inline(always)]
  pub fn new_kind_object<T>(kind: Gc<Object<Self>>, name: &str) -> Object<Self> {
    Object::new(
      kind,
      Kind::new(name.into(), mem::size_of::<T>(), mem::align_of::<T>()),
    )
  }

  #[inline]
  pub fn name(&self) -> &String {
    &self.name
  }
  #[inline]
  pub fn size(&self) -> usize {
    self.size
  }
  #[inline]
  pub fn align(&self) -> usize {
    self.align
  }
}

#[inline]
pub fn kind_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Kind")
      .expect("failed to get Kind Kind")
  }
}
#[inline]
pub fn new_kind<T>(scope: Gc<Object<Scope>>, name: &str) -> Gc<Object<Kind>> {
  new_object(
    scope.clone(),
    Kind::new_kind_object::<T>(kind_kind(scope), name),
  )
}
