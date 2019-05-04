use alloc::vec::Vec;

use core::fmt::Debug;
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::Gc;

use gc::Trace;

use super::{Kind, Object, Value, Scope};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcAllocator {
  scope: Gc<Object<Scope>>,
  size: usize,
  max_size: usize,
  values: Vec<Gc<Value>>,
}

impl Hash for GcAllocator {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl Trace for GcAllocator {
  fn mark(&mut self) {
    for v in self.values.iter_mut() {
      v.mark();
    }
  }
}

impl GcAllocator {
  #[inline]
  pub fn new(scope: Gc<Object<Scope>>) -> Self {
    GcAllocator {
      scope: scope,
      size: 0,
      max_size: 1024 * 8,
      values: Vec::new(),
    }
  }

  #[inline]
  pub unsafe fn maintain<T>(&mut self, value: Gc<Object<T>>) -> &mut Self
  where
    T: PartialEq + Hash + Debug + Trace + 'static,
  {
    self.size += value.kind().size();
    self.values.push(value.into_value());

    if self.size > self.max_size {
      self.collect(&mut (self.scope.clone()));
    }

    self
  }

  #[inline(always)]
  pub fn alloc<T>(&mut self, object: Object<T>) -> Gc<Object<T>>
  where
    T: PartialEq + Hash + Debug + Trace + 'static,
  {
    unsafe {
      let object = Gc::new(object);
      self.maintain(object.clone());
      object
    }
  }

  #[inline(always)]
  pub fn alloc_object<T>(&mut self, kind: Gc<Object<Kind>>, value: T) -> Gc<Object<T>>
  where
    T: PartialEq + Hash + Debug + Trace + 'static,
  {
    self.alloc(Object::new(kind, value))
  }

  #[inline(always)]
  pub fn collect(&mut self, scope: &mut Gc<Object<Scope>>) -> &mut Self {
    scope.mark();

    let mut size = 0;

    self.values.retain(|v| {
      let marked = v.is_marked();
      if !marked {
        size += v.kind().size();
      }
      marked
    });
    self.size -= size;

    self
  }
}
