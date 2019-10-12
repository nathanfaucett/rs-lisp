use alloc::collections::linked_list::LinkedList;
use alloc::vec::Vec;

use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::Gc;

use gc::Trace;

use super::{add_external_function, new_usize, Kind, List, Object, Scope, Value};

#[derive(Clone, PartialEq, Eq)]
pub struct GcAllocator {
  scope: Gc<Object<Scope>>,
  size: usize,
  max_size: usize,
  values: Vec<Gc<dyn Value>>,
}

impl fmt::Debug for GcAllocator {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("GcAllocator")
      .field("size", &self.size)
      .field("max_size", &self.max_size)
      .finish()
  }
}

impl Hash for GcAllocator {
  #[inline]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl Drop for GcAllocator {
  #[inline]
  fn drop(&mut self) {
    for v in self.values.drain(..) {
      unsafe {
        v.unsafe_drop();
      }
    }
  }
}

impl Trace for GcAllocator {}

impl GcAllocator {
  #[inline]
  pub fn new(scope: Gc<Object<Scope>>) -> Self {
    GcAllocator {
      scope: scope,
      size: 0,
      max_size: 1024 * 1024,
      values: Vec::new(),
    }
  }

  #[inline]
  pub(crate) unsafe fn unsafe_new() -> Self {
    GcAllocator {
      scope: Gc::null(),
      size: 0,
      max_size: 1024 * 1024,
      values: Vec::new(),
    }
  }

  #[inline]
  pub(crate) unsafe fn unsafe_set_scope(&mut self, scope: Gc<Object<Scope>>) -> &mut Self {
    self.scope = scope;
    self
  }

  #[inline]
  pub unsafe fn maintain_value(&mut self, value: Gc<dyn Value>) -> &mut Self {
    self.size += value.kind().size();
    self.values.push(value);

    if self.size > self.max_size {
      self.collect();
    }

    self
  }

  #[inline]
  pub unsafe fn maintain<T>(&mut self, object: Gc<Object<T>>) -> &mut Self
  where
    T: PartialEq + Hash + Debug + Trace + 'static,
  {
    self.maintain_value(object.into_value())
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
  pub fn collect(&mut self) -> usize {
    self.scope.trace(true);

    let mut size = 0;
    let mut removed = LinkedList::new();

    self.values.retain(|v| {
      let marked = v.is_marked();
      if !marked {
        size += v.kind().size();
        removed.push_front(v.clone());
      }
      marked
    });

    for v in removed.into_iter() {
      unsafe {
        v.unsafe_drop();
      }
    }

    for value in self.values.iter_mut() {
      value.trace(false);
    }

    self.size -= size;

    size
  }

  #[inline]
  pub(crate) fn init_scope(scope: Gc<Object<Scope>>) {
    add_external_function(
      scope,
      "gc_allocator.collect",
      vec!["gc_allocator"],
      gc_allocator_collect,
    );
  }
}

#[inline]
pub fn gc_allocator_collect(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let gc_allocator = args
    .front_mut()
    .expect("GcAllocator is nil")
    .downcast_mut::<Object<GcAllocator>>()
    .expect("Failed to downcast to GcAllocator");

  new_usize(scope, gc_allocator.collect()).into_value()
}

#[inline]
pub fn gc_allocator_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("GcAllocator")
      .expect("failed to get GcAllocator Kind")
  }
}
