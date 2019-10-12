use alloc::string::{String, ToString};

use super::{
  new_kind, new_object, Escape, Function, GcAllocator, Keyword, Kind, LinkedMap, List, Map, Object,
  Scope, SpecialForm, Stack, Symbol, Vec,
};
use gc::Gc;

#[inline]
pub fn new_context() -> Gc<Object<Scope>> {
  unsafe {
    let mut scope = init_root_scope();

    init_nil(scope.clone());
    init_bool(scope.clone());
    init_char(scope.clone());
    init_string(scope.clone());
    init_numbers(scope.clone());
    Function::init_kind(scope.clone());
    Stack::init_kind(scope.clone());
    SpecialForm::init_kind(scope.clone());
    Symbol::init_kind(scope.clone());
    Keyword::init_kind(scope.clone());
    Escape::init_kind(scope.clone());
    List::init_kind(scope.clone());
    LinkedMap::init_kind(scope.clone());
    Vec::init_kind(scope.clone());
    Map::init_kind(scope.clone());

    GcAllocator::init_scope(scope.clone());
    Stack::init_scope(scope.clone());
    Scope::init_scope(scope.clone());
    List::init_scope(scope.clone());
    LinkedMap::init_scope(scope.clone());
    Vec::init_scope(scope.clone());
    Map::init_scope(scope.clone());

    scope
      .get_mut("default_gc_allocator")
      .expect("failed to find default_gc_allocator")
      .downcast_mut::<Object<GcAllocator>>()
      .expect("failed to downcast default_gc_allocator to GcAllocator")
      .collect();

    scope
  }
}

#[inline]
unsafe fn init_root_scope() -> Gc<Object<Scope>> {
  let mut scope: Gc<Object<Scope>> = Gc::null();

  let mut gc_allocator = GcAllocator::unsafe_new();

  let kind_kind = Kind::new_kind_kind();

  gc_allocator.maintain(kind_kind.clone());

  let scope_kind = gc_allocator.alloc(Kind::new_kind_object::<Scope>(kind_kind.clone(), "Scope"));
  scope.set_from_value(Object::new(scope_kind.clone(), Scope::new(None)));

  gc_allocator.unsafe_set_scope(scope.clone());

  scope.set("Kind", kind_kind.clone().into_value());
  scope.set("Scope", scope_kind.clone().into_value());

  let gc_allocator_kind = gc_allocator.alloc(Kind::new_kind_object::<GcAllocator>(
    kind_kind,
    "GcAllocator",
  ));
  let mut gc_allocator_object = Gc::new(Object::new(gc_allocator_kind.clone(), gc_allocator));
  let gc_allocator_value = gc_allocator_object.clone().into_value();

  gc_allocator_object.maintain_value(gc_allocator_value.clone());

  scope.set("GcAllocator", gc_allocator_kind.into_value());
  scope.set("default_gc_allocator", gc_allocator_value);

  scope
}

#[inline]
unsafe fn init_nil(mut scope: Gc<Object<Scope>>) {
  let nil_kind = new_kind::<()>(scope.clone(), "Nil");
  let nil_value = new_object(scope.clone(), Object::new(nil_kind.clone(), ()));

  scope.set("Nil", nil_kind.into_value());
  scope.set("nil", nil_value.into_value());
}

#[inline]
unsafe fn init_bool(mut scope: Gc<Object<Scope>>) {
  let boolean_kind = new_kind::<bool>(scope.clone(), "Bool");
  let true_value = new_object(scope.clone(), Object::new(boolean_kind.clone(), true));
  let false_value = new_object(scope.clone(), Object::new(boolean_kind.clone(), false));

  scope.set("Bool", boolean_kind.into_value());
  scope.set("true", true_value.into_value());
  scope.set("false", false_value.into_value());
}

#[inline]
unsafe fn init_char(mut scope: Gc<Object<Scope>>) {
  let character_kind = new_kind::<char>(scope.clone(), "Char");
  scope.set("Char", character_kind.into_value());
}

#[inline]
unsafe fn init_string(mut scope: Gc<Object<Scope>>) {
  let string_kind = new_kind::<String>(scope.clone(), "String");
  scope.set("String", string_kind.into_value());
}

#[inline]
unsafe fn init_numbers(mut scope: Gc<Object<Scope>>) {
  // Unsigned
  let u8_kind = new_kind::<u8>(scope.clone(), "U8");
  scope.set("U8", u8_kind.into_value());

  let u16_kind = new_kind::<u16>(scope.clone(), "U16");
  scope.set("U16", u16_kind.into_value());

  let u32_kind = new_kind::<u32>(scope.clone(), "U32");
  scope.set("U32", u32_kind.into_value());

  let u64_kind = new_kind::<u64>(scope.clone(), "U64");
  scope.set("U64", u64_kind.into_value());

  let usize_kind = new_kind::<usize>(scope.clone(), "USize");
  scope.set("USize", usize_kind.into_value());

  // Signed
  let i8_kind = new_kind::<i8>(scope.clone(), "I8");
  scope.set("I8", i8_kind.into_value());

  let i16_kind = new_kind::<i16>(scope.clone(), "I16");
  scope.set("I16", i16_kind.into_value());

  let i32_kind = new_kind::<i32>(scope.clone(), "I32");
  scope.set("I32", i32_kind.into_value());

  let i64_kind = new_kind::<i64>(scope.clone(), "I64");
  scope.set("I64", i64_kind.into_value());

  let isize_kind = new_kind::<isize>(scope.clone(), "ISize");
  scope.set("ISize", isize_kind.into_value());

  // Float
  let f32_kind = new_kind::<f32>(scope.clone(), "F32");
  scope.set("F32", f32_kind.into_value());

  let f64_kind = new_kind::<f64>(scope.clone(), "F64");
  scope.set("F64", f64_kind.into_value());
}

#[inline]
pub fn usize_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("USize")
      .expect("failed to get USize Kind")
  }
}
#[inline]
pub fn new_usize(scope: Gc<Object<Scope>>, value: usize) -> Gc<Object<usize>> {
  new_object(scope.clone(), Object::new(usize_kind(scope), value))
}

#[inline]
pub fn i8_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("I8")
      .expect("failed to get I8 Kind")
  }
}
#[inline]
pub fn new_i8(scope: Gc<Object<Scope>>, value: i8) -> Gc<Object<i8>> {
  new_object(scope.clone(), Object::new(i8_kind(scope), value))
}

#[inline]
pub fn i16_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("I6")
      .expect("failed to get I16 Kind")
  }
}
#[inline]
pub fn new_i16(scope: Gc<Object<Scope>>, value: i16) -> Gc<Object<i16>> {
  new_object(scope.clone(), Object::new(i16_kind(scope), value))
}

#[inline]
pub fn i32_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("I32")
      .expect("failed to get I32 Kind")
  }
}
#[inline]
pub fn new_i32(scope: Gc<Object<Scope>>, value: i32) -> Gc<Object<i32>> {
  new_object(scope.clone(), Object::new(i32_kind(scope), value))
}

#[inline]
pub fn i64_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("I64")
      .expect("failed to get I64 Kind")
  }
}
#[inline]
pub fn new_i64(scope: Gc<Object<Scope>>, value: i64) -> Gc<Object<i64>> {
  new_object(scope.clone(), Object::new(i64_kind(scope), value))
}

#[inline]
pub fn isize_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("ISize")
      .expect("failed to get ISize Kind")
  }
}
#[inline]
pub fn new_isize(scope: Gc<Object<Scope>>, value: isize) -> Gc<Object<isize>> {
  new_object(scope.clone(), Object::new(isize_kind(scope), value))
}

#[inline]
pub fn u8_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("U8")
      .expect("failed to get U8 Kind")
  }
}
#[inline]
pub fn new_u8(scope: Gc<Object<Scope>>, value: u8) -> Gc<Object<u8>> {
  new_object(scope.clone(), Object::new(u8_kind(scope), value))
}

#[inline]
pub fn u16_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("U6")
      .expect("failed to get U16 Kind")
  }
}
#[inline]
pub fn new_u16(scope: Gc<Object<Scope>>, value: u16) -> Gc<Object<u16>> {
  new_object(scope.clone(), Object::new(u16_kind(scope), value))
}

#[inline]
pub fn u32_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("U32")
      .expect("failed to get U32 Kind")
  }
}
#[inline]
pub fn new_u32(scope: Gc<Object<Scope>>, value: u32) -> Gc<Object<u32>> {
  new_object(scope.clone(), Object::new(u32_kind(scope), value))
}

#[inline]
pub fn u64_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("U64")
      .expect("failed to get U64 Kind")
  }
}
#[inline]
pub fn new_u64(scope: Gc<Object<Scope>>, value: u64) -> Gc<Object<u64>> {
  new_object(scope.clone(), Object::new(u64_kind(scope), value))
}

// #[inline]
// pub fn f32_kind(mut scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
//     unsafe {
//         scope
//             .get_with_kind::<Kind>("F32")
//             .expect("failed to get F32 Kind")
//     }
// }
// #[inline]
// pub fn new_f32(mut scope: Gc<Object<Scope>>, value: f32) -> Gc<Object<f32>> {
//     unsafe { gc_allocator.alloc(Object::new(f32_kind(scope), value)) }
// }

// #[inline]
// pub fn f64_kind(mut scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
//     unsafe {
//         scope
//             .get_with_kind::<Kind>("F64")
//             .expect("failed to get F64 Kind")
//     }
// }
// #[inline]
// pub fn new_f64(mut scope: Gc<Object<Scope>>, value: f64) -> Gc<Object<f64>> {
//     unsafe { gc_allocator.alloc(Object::new(f64_kind(scope), value)) }
// }

// #[inline]
// pub fn new_nan_f32(mut scope: Gc<Object<Scope>>) -> Gc<Object<f32>> {
//     unsafe { gc_allocator.alloc(Object::new(f64_kind(scope), ::core::f32::NAN)) }
// }

// #[inline]
// pub fn new_nan_f64(mut scope: Gc<Object<Scope>>) -> Gc<Object<f64>> {
//     unsafe { gc_allocator.alloc(Object::new(f64_kind(scope), ::core::f64::NAN)) }
// }

#[inline]
pub fn bool_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Bool")
      .expect("failed to get Bool Kind")
  }
}
#[inline]
pub fn true_value(scope: Gc<Object<Scope>>) -> Gc<Object<bool>> {
  unsafe {
    scope
      .get_with_kind::<bool>("true")
      .expect("failed to get true value")
  }
}
#[inline]
pub fn false_value(scope: Gc<Object<Scope>>) -> Gc<Object<bool>> {
  unsafe {
    scope
      .get_with_kind::<bool>("false")
      .expect("failed to get false value")
  }
}
#[inline]
pub fn new_bool(scope: Gc<Object<Scope>>, value: bool) -> Gc<Object<bool>> {
  if value {
    true_value(scope)
  } else {
    false_value(scope)
  }
}

#[inline]
pub fn nil_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Nil")
      .expect("failed to get Nil Kind")
  }
}
#[inline]
pub fn nil_value(scope: Gc<Object<Scope>>) -> Gc<Object<()>> {
  unsafe {
    scope
      .get_with_kind::<()>("nil")
      .expect("failed to get nil value")
  }
}

#[inline]
pub fn char_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Char")
      .expect("failed to get Char Kind")
  }
}
#[inline]
pub fn new_char(scope: Gc<Object<Scope>>, value: char) -> Gc<Object<char>> {
  new_object(scope.clone(), Object::new(char_kind(scope), value))
}

#[inline]
pub fn string_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("String")
      .expect("failed to get String Kind")
  }
}
#[inline]
pub fn new_string<T>(scope: Gc<Object<Scope>>, value: T) -> Gc<Object<String>>
where
  T: ToString,
{
  new_object(
    scope.clone(),
    Object::new(string_kind(scope), value.to_string()),
  )
}
