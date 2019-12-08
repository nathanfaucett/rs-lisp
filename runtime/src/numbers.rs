use core::ops::{Add, Div, Mul, Sub};

use super::{add_external_function, new_bool, new_kind, new_object, Kind, Object, Scope};
use gc::Gc;

#[inline]
pub fn init_numbers_kind(mut scope: Gc<Object<Scope>>) {
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
pub fn init_numbers_scope(scope: Gc<Object<Scope>>) {
  add_external_function(scope.clone(), "u8.add", vec!["a", "b"], u8_add);
  add_external_function(scope.clone(), "u8.sub", vec!["a", "b"], u8_sub);
  add_external_function(scope.clone(), "u8.mul", vec!["a", "b"], u8_mul);
  add_external_function(scope.clone(), "u8.div", vec!["a", "b"], u8_div);
  add_external_function(scope.clone(), "u8.eq", vec!["a", "b"], u8_eq);

  add_external_function(scope.clone(), "u16.add", vec!["a", "b"], u16_add);
  add_external_function(scope.clone(), "u16.sub", vec!["a", "b"], u16_sub);
  add_external_function(scope.clone(), "u16.mul", vec!["a", "b"], u16_mul);
  add_external_function(scope.clone(), "u16.div", vec!["a", "b"], u16_div);
  add_external_function(scope.clone(), "u16.eq", vec!["a", "b"], u16_eq);

  add_external_function(scope.clone(), "u32.add", vec!["a", "b"], u32_add);
  add_external_function(scope.clone(), "u32.sub", vec!["a", "b"], u32_sub);
  add_external_function(scope.clone(), "u32.mul", vec!["a", "b"], u32_mul);
  add_external_function(scope.clone(), "u32.div", vec!["a", "b"], u32_div);
  add_external_function(scope.clone(), "u32.eq", vec!["a", "b"], u32_eq);

  add_external_function(scope.clone(), "u64.add", vec!["a", "b"], u64_add);
  add_external_function(scope.clone(), "u64.sub", vec!["a", "b"], u64_sub);
  add_external_function(scope.clone(), "u64.mul", vec!["a", "b"], u64_mul);
  add_external_function(scope.clone(), "u64.div", vec!["a", "b"], u64_div);
  add_external_function(scope.clone(), "u64.eq", vec!["a", "b"], u64_eq);

  add_external_function(scope.clone(), "usize.add", vec!["a", "b"], usize_add);
  add_external_function(scope.clone(), "usize.sub", vec!["a", "b"], usize_sub);
  add_external_function(scope.clone(), "usize.mul", vec!["a", "b"], usize_mul);
  add_external_function(scope.clone(), "usize.div", vec!["a", "b"], usize_div);
  add_external_function(scope.clone(), "usize.eq", vec!["a", "b"], usize_eq);

  add_external_function(scope.clone(), "i8.add", vec!["a", "b"], i8_add);
  add_external_function(scope.clone(), "i8.sub", vec!["a", "b"], i8_sub);
  add_external_function(scope.clone(), "i8.mul", vec!["a", "b"], i8_mul);
  add_external_function(scope.clone(), "i8.div", vec!["a", "b"], i8_div);
  add_external_function(scope.clone(), "i8.eq", vec!["a", "b"], i8_eq);

  add_external_function(scope.clone(), "i16.add", vec!["a", "b"], i16_add);
  add_external_function(scope.clone(), "i16.sub", vec!["a", "b"], i16_sub);
  add_external_function(scope.clone(), "i16.mul", vec!["a", "b"], i16_mul);
  add_external_function(scope.clone(), "i16.div", vec!["a", "b"], i16_div);
  add_external_function(scope.clone(), "i16.eq", vec!["a", "b"], i16_eq);

  add_external_function(scope.clone(), "i32.add", vec!["a", "b"], i32_add);
  add_external_function(scope.clone(), "i32.sub", vec!["a", "b"], i32_sub);
  add_external_function(scope.clone(), "i32.mul", vec!["a", "b"], i32_mul);
  add_external_function(scope.clone(), "i32.div", vec!["a", "b"], i32_div);
  add_external_function(scope.clone(), "i32.eq", vec!["a", "b"], i32_eq);

  add_external_function(scope.clone(), "i64.add", vec!["a", "b"], i64_add);
  add_external_function(scope.clone(), "i64.sub", vec!["a", "b"], i64_sub);
  add_external_function(scope.clone(), "i64.mul", vec!["a", "b"], i64_mul);
  add_external_function(scope.clone(), "i64.div", vec!["a", "b"], i64_div);
  add_external_function(scope.clone(), "i64.eq", vec!["a", "b"], i64_eq);

  add_external_function(scope.clone(), "isize.add", vec!["a", "b"], isize_add);
  add_external_function(scope.clone(), "isize.sub", vec!["a", "b"], isize_sub);
  add_external_function(scope.clone(), "isize.mul", vec!["a", "b"], isize_mul);
  add_external_function(scope.clone(), "isize.div", vec!["a", "b"], isize_div);
  add_external_function(scope.clone(), "isize.eq", vec!["a", "b"], isize_eq);

  // add_external_function(scope.clone(), "f32.add", vec!["a", "b"], f32_add);
  // add_external_function(scope.clone(), "f32.sub", vec!["a", "b"], f32_sub);
  // add_external_function(scope.clone(), "f32.mul", vec!["a", "b"], f32_mul);
  // add_external_function(scope.clone(), "f32.div", vec!["a", "b"], f32_div);
  // add_external_function(scope.clone(), "f32.eq", vec!["a", "b"], f32_eq);

  // add_external_function(scope.clone(), "f64.add", vec!["a", "b"], f64_add);
  // add_external_function(scope.clone(), "f64.sub", vec!["a", "b"], f64_sub);
  // add_external_function(scope.clone(), "f64.mul", vec!["a", "b"], f64_mul);
  // add_external_function(scope.clone(), "f64.div", vec!["a", "b"], f64_div);
  // add_external_function(scope.clone(), "f64.eq", vec!["a", "b"], f64_eq);
}

macro_rules! binary {
  ($name:ident, $type:ty, $func:ident, $new_func:ident) => {
    #[inline]
    pub fn $name(
      scope: $crate::gc::Gc<$crate::Object<$crate::Scope>>,
      mut args: $crate::gc::Gc<$crate::Object<$crate::List>>,
    ) -> ::gc::Gc<dyn $crate::Value> {
      let a = args
        .pop_front()
        .expect("a is nil")
        .downcast::<$crate::Object<$type>>()
        .expect("Failed to downcast a");
      let b = args
        .pop_front()
        .expect("b is nil")
        .downcast::<$crate::Object<$type>>()
        .expect("Failed to downcast b");

      $new_func(scope, a.value().$func(b.value())).into_value()
    }
  };
}

binary!(u8_add, u8, add, new_u8);
binary!(u8_sub, u8, sub, new_u8);
binary!(u8_mul, u8, mul, new_u8);
binary!(u8_div, u8, div, new_u8);
binary!(u8_eq, u8, eq, new_bool);

binary!(u16_add, u16, add, new_u16);
binary!(u16_sub, u16, sub, new_u16);
binary!(u16_mul, u16, mul, new_u16);
binary!(u16_div, u16, div, new_u16);
binary!(u16_eq, u16, eq, new_bool);

binary!(u32_add, u32, add, new_u32);
binary!(u32_sub, u32, sub, new_u32);
binary!(u32_mul, u32, mul, new_u32);
binary!(u32_div, u32, div, new_u32);
binary!(u32_eq, u32, eq, new_bool);

binary!(u64_add, u64, add, new_u64);
binary!(u64_sub, u64, sub, new_u64);
binary!(u64_mul, u64, mul, new_u64);
binary!(u64_div, u64, div, new_u64);
binary!(u64_eq, u64, eq, new_bool);

binary!(usize_add, usize, add, new_usize);
binary!(usize_sub, usize, sub, new_usize);
binary!(usize_mul, usize, mul, new_usize);
binary!(usize_div, usize, div, new_usize);
binary!(usize_eq, usize, eq, new_bool);

binary!(i8_add, i8, add, new_i8);
binary!(i8_sub, i8, sub, new_i8);
binary!(i8_mul, i8, mul, new_i8);
binary!(i8_div, i8, div, new_i8);
binary!(i8_eq, i8, eq, new_bool);

binary!(i16_add, i16, add, new_i16);
binary!(i16_sub, i16, sub, new_i16);
binary!(i16_mul, i16, mul, new_i16);
binary!(i16_div, i16, div, new_i16);
binary!(i16_eq, i16, eq, new_bool);

binary!(i32_add, i32, add, new_i32);
binary!(i32_sub, i32, sub, new_i32);
binary!(i32_mul, i32, mul, new_i32);
binary!(i32_div, i32, div, new_i32);
binary!(i32_eq, i32, eq, new_bool);

binary!(i64_add, i64, add, new_i64);
binary!(i64_sub, i64, sub, new_i64);
binary!(i64_mul, i64, mul, new_i64);
binary!(i64_div, i64, div, new_i64);
binary!(i64_eq, isize, eq, new_bool);

binary!(isize_add, isize, add, new_isize);
binary!(isize_sub, isize, sub, new_isize);
binary!(isize_mul, isize, mul, new_isize);
binary!(isize_div, isize, div, new_isize);
binary!(isize_eq, isize, eq, new_bool);

// binary!(f32_add, f32, add, new_f32);
// binary!(f32_sub, f32, sub, new_f32);
// binary!(f32_mul, f32, mul, new_f32);
// binary!(f32_div, f32, div, new_f32);
// binary!(f32_eq, f32, eq, new_bool);

// binary!(f64_add, f64, add, new_f64);
// binary!(f64_sub, f64, sub, new_f64);
// binary!(f64_mul, f64, mul, new_f64);
// binary!(f64_div, f64, div, new_f64);
// binary!(f64_eq, f64, eq, new_bool);

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
