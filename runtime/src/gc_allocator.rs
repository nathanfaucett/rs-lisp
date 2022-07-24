use alloc::collections::linked_list::LinkedList;
use alloc::vec::Vec;

use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::{cmp, ptr};

use parking_lot::Mutex;

use gc::Gc;

use gc::Trace;

use super::{
  add_external_macro, new_usize, scope_get_with_kind, Kind, Object, Scope, Value, Vector,
};

pub struct GcAllocator {
  size: usize,
  max_size: usize,
  values: Mutex<Vec<Gc<dyn Value>>>,
}

impl Eq for GcAllocator {}

impl PartialEq for GcAllocator {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    ptr::eq(self, other)
  }
}

impl PartialOrd for GcAllocator {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<cmp::Ordering> {
    None
  }
}

impl Clone for GcAllocator {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      size: self.size,
      max_size: self.max_size,
      values: Mutex::new(self.values.lock().clone()),
    }
  }
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
    for v in self.values.lock().drain(..) {
      unsafe {
        v.unsafe_drop();
      }
    }
  }
}

impl Trace for GcAllocator {}

impl GcAllocator {
  #[inline]
  pub fn new() -> Self {
    GcAllocator {
      size: 0,
      max_size: 1024 * 1024,
      values: Mutex::default(),
    }
  }

  #[inline]
  pub unsafe fn maintain_value(
    &mut self,
    scope: &mut Gc<Object<Scope>>,
    value: Gc<dyn Value>,
  ) -> &mut Self {
    self.unsafe_maintain_value(value);

    if self.size > self.max_size {
      self.collect(scope);
    }

    self
  }

  #[inline]
  pub unsafe fn unsafe_maintain_value(&mut self, value: Gc<dyn Value>) -> &mut Self {
    self.size += value.kind().size();
    self.values.lock().push(value);
    self
  }

  #[inline]
  pub unsafe fn maintain<T>(
    &mut self,
    scope: &mut Gc<Object<Scope>>,
    object: Gc<Object<T>>,
  ) -> Gc<Object<T>>
  where
    T: PartialEq + PartialOrd + Hash + Debug + Trace + 'static,
  {
    self.maintain_value(scope, object.clone().into_value());
    object
  }

  #[inline]
  pub unsafe fn unsafe_maintain<T>(&mut self, object: Gc<Object<T>>) -> Gc<Object<T>>
  where
    T: PartialEq + PartialOrd + Hash + Debug + Trace + 'static,
  {
    self.unsafe_maintain_value(object.clone().into_value());
    object
  }

  #[inline(always)]
  pub fn alloc<T>(&mut self, scope: &mut Gc<Object<Scope>>, object: Object<T>) -> Gc<Object<T>>
  where
    T: PartialEq + PartialOrd + Hash + Debug + Trace + 'static,
  {
    unsafe {
      let object = Gc::new(object);
      self.maintain(scope, object.clone());
      object
    }
  }

  #[inline(always)]
  pub fn unsafe_alloc<T>(&mut self, object: Object<T>) -> Gc<Object<T>>
  where
    T: PartialEq + PartialOrd + Hash + Debug + Trace + 'static,
  {
    unsafe {
      let object = Gc::new(object);
      self.unsafe_maintain(object.clone());
      object
    }
  }

  #[inline(always)]
  pub fn alloc_object<T>(
    &mut self,
    scope: &mut Gc<Object<Scope>>,
    kind: Gc<Object<Kind>>,
    value: T,
  ) -> Gc<Object<T>>
  where
    T: PartialEq + PartialOrd + Hash + Debug + Trace + 'static,
  {
    self.alloc(scope, Object::new(kind, value))
  }

  #[inline(always)]
  pub fn collect(&mut self, scope: &mut Gc<Object<Scope>>) -> usize {
    scope.trace(true);

    let mut size = 0;
    let mut removed = LinkedList::new();
    {
      let mut values = self.values.lock();
      values.retain(|v| {
        let marked = v.is_marked();
        if !marked {
          size += v.kind().size();
          removed.push_front(v.clone());
        }
        marked
      });

      for value in values.iter_mut() {
        value.mark(false);
      }
    }

    for v in removed.into_iter() {
      unsafe {
        v.unsafe_drop();
      }
    }

    self.size -= size;

    size
  }

  #[inline]
  pub fn size(&self) -> usize {
    self.size
  }

  #[inline]
  pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
    add_external_macro(
      scope,
      "gc_allocator.collect",
      vec!["gc_allocator"],
      gc_allocator_collect,
    );
  }
}

#[inline]
pub fn gc_allocator_collect(
  scope: &Gc<Object<Scope>>,
  _args: &Gc<Object<Vector>>,
) -> Gc<dyn Value> {
  let mut gc_allocator = scope_get_with_kind::<GcAllocator>(&scope, "default_gc_allocator")
    .expect("Failed to get `default_gc_allocator`")
    .clone();
  let collected_bytes = gc_allocator.collect(&mut scope.clone());
  new_usize(scope, collected_bytes).into_value()
}

#[inline]
pub fn gc_allocator_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "GcAllocator").expect("failed to get GcAllocator Kind")
}
