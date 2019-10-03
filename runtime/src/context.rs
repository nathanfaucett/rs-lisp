use alloc::string::{String, ToString};

use core::fmt::Debug;
use core::hash::Hash;

use super::{
  def_special_form, do_special_form, eval_special_form, expand_special_form, fn_special_form,
  if_special_form, macro_special_form, quote_special_form, read_special_form, Escape, Function,
  GcAllocator, Keyword, Kind, List, Map, Object, Scope, SpecialForm, Stack, Symbol, Value, Vec,
};
use gc::{Gc, Trace};

#[inline]
pub fn new() -> Gc<Object<Scope>> {
  unsafe {
    let mut scope = init_root_scope();

    init_nil(scope.clone());
    init_function(scope.clone());
    init_special_form(scope.clone());
    init_bool(scope.clone());
    init_char(scope.clone());
    init_string(scope.clone());
    init_symbol(scope.clone());
    init_keyword(scope.clone());
    init_escape(scope.clone());
    init_numbers(scope.clone());
    init_list(scope.clone());
    init_vec(scope.clone());
    init_map(scope.clone());

    GcAllocator::init_scope(scope.clone(), gc_allocator_kind(scope.clone()));
    Scope::init_scope(scope.clone(), scope_kind(scope.clone()));
    List::init_scope(scope.clone(), list_kind(scope.clone()));
    Vec::init_scope(scope.clone(), vec_kind(scope.clone()));
    Map::init_scope(scope.clone(), map_kind(scope.clone()));

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

  let type_kind = Kind::new_type_kind();

  gc_allocator.maintain(type_kind.clone());

  let scope_kind = gc_allocator.alloc(Kind::new_kind::<Scope>(type_kind.clone(), "Scope"));
  scope.set_from_value(Object::new(scope_kind.clone(), Scope::new(None)));

  gc_allocator.unsafe_set_scope(scope.clone());
  gc_allocator.maintain(scope.clone());

  scope.set("Type", type_kind.clone().into_value());
  scope.set("Scope", scope_kind.clone().into_value());
  let scope_value = scope.clone().into_value();
  scope.set("global", scope_value);

  let gc_allocator_kind =
    gc_allocator.alloc(Kind::new_kind::<GcAllocator>(type_kind, "GcAllocator"));
  let gc_allocator_value =
    gc_allocator.alloc_object(gc_allocator_kind.clone(), gc_allocator.clone());

  scope.set("GcAllocator", gc_allocator_kind.into_value());
  scope.set("default_gc_allocator", gc_allocator_value.into_value());

  scope
}

#[inline]
unsafe fn init_nil(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let nil_kind = gc_allocator.alloc(Kind::new_kind::<()>(type_kind.clone(), "Nil"));
  let nil_value = gc_allocator.alloc(Object::new(nil_kind.clone(), ()));

  scope.set("Nil", nil_kind.into_value());
  scope.set("nil", nil_value.into_value());
}

#[inline]
unsafe fn init_function(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let function_kind = Kind::new_kind::<Function>(type_kind.clone(), "Function");
  let macro_kind = Kind::new_kind::<Function>(type_kind.clone(), "Macro");

  scope.set("Function", gc_allocator.alloc(function_kind).into_value());
  scope.set("Macro", gc_allocator.alloc(macro_kind).into_value());
}

#[inline]
unsafe fn init_special_form(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let special_form_kind = gc_allocator.alloc(Kind::new_kind::<SpecialForm>(
    type_kind.clone(),
    "SpecialForm",
  ));

  scope.set("SpecialForm", special_form_kind.into_value());

  let if_function = new_special_form(scope.clone(), if_special_form).into_value();
  scope.set("if", if_function);

  let fn_function = new_special_form(scope.clone(), fn_special_form).into_value();
  scope.set("fn", fn_function);

  let macro_function = new_special_form(scope.clone(), macro_special_form).into_value();
  scope.set("macro", macro_function);

  let def_function = new_special_form(scope.clone(), def_special_form).into_value();
  scope.set("def", def_function);

  let do_function = new_special_form(scope.clone(), do_special_form).into_value();
  scope.set("do", do_function);

  let quote_function = new_special_form(scope.clone(), quote_special_form).into_value();
  scope.set("quote", quote_function);

  let eval_function = new_special_form(scope.clone(), eval_special_form).into_value();
  scope.set("eval", eval_function);

  let read_function = new_special_form(scope.clone(), read_special_form).into_value();
  scope.set("read", read_function);

  let expand_function = new_special_form(scope.clone(), expand_special_form).into_value();
  scope.set("expand", expand_function);
}

#[inline]
unsafe fn init_bool(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let boolean_kind = gc_allocator.alloc(Kind::new_kind::<bool>(type_kind, "Bool"));
  let true_value = gc_allocator.alloc(Object::new(boolean_kind.clone(), true));
  let false_value = gc_allocator.alloc(Object::new(boolean_kind.clone(), false));

  scope.set("Bool", boolean_kind.into_value());
  scope.set("true", true_value.into_value());
  scope.set("false", false_value.into_value());
}

#[inline]
unsafe fn init_char(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let character_kind = gc_allocator.alloc(Kind::new_kind::<char>(type_kind, "Char"));
  scope.set("Char", character_kind.into_value());
}

#[inline]
unsafe fn init_string(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let string_kind = gc_allocator.alloc(Kind::new_kind::<String>(type_kind, "String"));
  scope.set("String", string_kind.into_value());
}

#[inline]
unsafe fn init_symbol(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let symbol_kind = gc_allocator.alloc(Kind::new_kind::<Symbol>(type_kind, "Symbol"));
  scope.set("Symbol", symbol_kind.into_value());
}

#[inline]
unsafe fn init_keyword(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let keyword_kind = gc_allocator.alloc(Kind::new_kind::<Keyword>(type_kind, "Keyword"));
  scope.set("Keyword", keyword_kind.into_value());
}

#[inline]
unsafe fn init_escape(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let escape_kind = gc_allocator.alloc(Kind::new_kind::<Keyword>(type_kind, "Escape"));
  scope.set("Escape", escape_kind.into_value());
}

#[inline]
unsafe fn init_numbers(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  // Unsigned
  let u8_kind = gc_allocator.alloc(Kind::new_kind::<u8>(type_kind.clone(), "U8"));
  scope.set("U8", u8_kind.into_value());

  let u16_kind = gc_allocator.alloc(Kind::new_kind::<u16>(type_kind.clone(), "U16"));
  scope.set("U16", u16_kind.into_value());

  let u32_kind = gc_allocator.alloc(Kind::new_kind::<u32>(type_kind.clone(), "U32"));
  scope.set("U32", u32_kind.into_value());

  let u64_kind = gc_allocator.alloc(Kind::new_kind::<u64>(type_kind.clone(), "U64"));
  scope.set("U64", u64_kind.into_value());

  let usize_kind = gc_allocator.alloc(Kind::new_kind::<usize>(type_kind.clone(), "USize"));
  scope.set("USize", usize_kind.into_value());

  // Signed
  let i8_kind = gc_allocator.alloc(Kind::new_kind::<i8>(type_kind.clone(), "I8"));
  scope.set("I8", i8_kind.into_value());

  let i16_kind = gc_allocator.alloc(Kind::new_kind::<i16>(type_kind.clone(), "I16"));
  scope.set("I16", i16_kind.into_value());

  let i32_kind = gc_allocator.alloc(Kind::new_kind::<i32>(type_kind.clone(), "I32"));
  scope.set("I32", i32_kind.into_value());

  let i64_kind = gc_allocator.alloc(Kind::new_kind::<i64>(type_kind.clone(), "I64"));
  scope.set("I64", i64_kind.into_value());

  let isize_kind = gc_allocator.alloc(Kind::new_kind::<isize>(type_kind.clone(), "ISize"));
  scope.set("ISize", isize_kind.into_value());

  // Float
  let f32_kind = gc_allocator.alloc(Kind::new_kind::<f32>(type_kind.clone(), "f32"));
  scope.set("F32", f32_kind.into_value());

  let f64_kind = gc_allocator.alloc(Kind::new_kind::<f64>(type_kind.clone(), "f64"));
  scope.set("F64", f64_kind.into_value());
}

#[inline]
unsafe fn init_list(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let list_kind = gc_allocator.alloc(Kind::new_kind::<List>(type_kind, "List"));
  scope.set("List", list_kind.clone().into_value());
}

#[inline]
unsafe fn init_vec(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let vec_kind = gc_allocator.alloc(Kind::new_kind::<Vec>(type_kind, "Vec"));
  scope.set("Vec", vec_kind.clone().into_value());
}

#[inline]
unsafe fn init_map(mut scope: Gc<Object<Scope>>) {
  let type_kind = scope.get_with_type::<Kind>("Type").unwrap();
  let mut gc_allocator = scope
    .get_with_type::<GcAllocator>("default_gc_allocator")
    .unwrap();

  let map_kind = gc_allocator.alloc(Kind::new_kind::<Map>(type_kind, "Map"));
  scope.set("Map", map_kind.clone().into_value());
}

#[inline]
pub fn new_object<T>(scope: Gc<Object<Scope>>, object: Object<T>) -> Gc<Object<T>>
where
  T: PartialEq + Hash + Debug + Trace + 'static,
{
  unsafe {
    let mut gc_allocator = scope
      .get_with_type::<GcAllocator>("default_gc_allocator")
      .unwrap();
    gc_allocator.alloc(object)
  }
}

#[inline]
pub fn usize_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("USize")
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
      .get_with_type::<Kind>("I8")
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
      .get_with_type::<Kind>("I6")
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
      .get_with_type::<Kind>("I32")
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
      .get_with_type::<Kind>("I64")
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
      .get_with_type::<Kind>("ISize")
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
      .get_with_type::<Kind>("U8")
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
      .get_with_type::<Kind>("U6")
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
      .get_with_type::<Kind>("U32")
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
      .get_with_type::<Kind>("U64")
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
//             .get_with_type::<Kind>("F32")
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
//             .get_with_type::<Kind>("F64")
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
      .get_with_type::<Kind>("Bool")
      .expect("failed to get Bool Kind")
  }
}
#[inline]
pub fn true_value(scope: Gc<Object<Scope>>) -> Gc<Object<bool>> {
  unsafe {
    scope
      .get_with_type::<bool>("true")
      .expect("failed to get true value")
  }
}
#[inline]
pub fn false_value(scope: Gc<Object<Scope>>) -> Gc<Object<bool>> {
  unsafe {
    scope
      .get_with_type::<bool>("false")
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
      .get_with_type::<Kind>("Nil")
      .expect("failed to get Nil Kind")
  }
}
#[inline]
pub fn nil_value(scope: Gc<Object<Scope>>) -> Gc<Object<()>> {
  unsafe {
    scope
      .get_with_type::<()>("nil")
      .expect("failed to get nil value")
  }
}

#[inline]
pub fn char_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Char")
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
      .get_with_type::<Kind>("String")
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

#[inline]
pub fn keyword_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Keyword")
      .expect("failed to get Keyword Kind")
  }
}
#[inline]
pub fn new_keyword<T>(scope: Gc<Object<Scope>>, value: T) -> Gc<Object<Keyword>>
where
  T: ToString,
{
  new_object(
    scope.clone(),
    Object::new(keyword_kind(scope), Keyword::new(value.to_string())),
  )
}

#[inline]
pub fn escape_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Escape")
      .expect("failed to get Escape Kind")
  }
}
#[inline]
pub fn new_escape(scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<Object<Escape>> {
  new_object(
    scope.clone(),
    Object::new(escape_kind(scope), Escape::new(value)),
  )
}

#[inline]
pub fn symbol_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Symbol")
      .expect("failed to get Symbol Kind")
  }
}
#[inline]
pub fn new_symbol<T>(scope: Gc<Object<Scope>>, value: T) -> Gc<Object<Symbol>>
where
  T: ToString,
{
  new_object(
    scope.clone(),
    Object::new(symbol_kind(scope), Symbol::new(value.to_string())),
  )
}

#[inline]
pub fn list_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("List")
      .expect("failed to get List Kind")
  }
}
#[inline]
pub fn new_list(scope: Gc<Object<Scope>>) -> Gc<Object<List>> {
  new_object(scope.clone(), Object::new(list_kind(scope), List::new()))
}

#[inline]
pub fn vec_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Vec")
      .expect("failed to get Vec Kind")
  }
}
#[inline]
pub fn new_vec(scope: Gc<Object<Scope>>) -> Gc<Object<Vec>> {
  new_object(scope.clone(), Object::new(vec_kind(scope), Vec::new()))
}

#[inline]
pub fn map_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Map")
      .expect("failed to get Map Kind")
  }
}
#[inline]
pub fn new_map(scope: Gc<Object<Scope>>) -> Gc<Object<Map>> {
  new_object(scope.clone(), Object::new(map_kind(scope), Map::new()))
}

#[inline]
pub fn gc_allocator_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("GcAllocator")
      .expect("failed to get GcAllocator Kind")
  }
}

#[inline]
pub fn scope_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Scope")
      .expect("failed to get Scope Kind")
  }
}
#[inline]
pub fn new_scope(scope: Gc<Object<Scope>>) -> Gc<Object<Scope>> {
  new_object(
    scope.clone(),
    Object::new(scope_kind(scope.clone()), Scope::new(Some(scope))),
  )
}

#[inline]
pub fn special_form_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("SpecialForm")
      .expect("failed to get SpecialForm Kind")
  }
}
#[inline]
pub fn new_special_form<F>(scope: Gc<Object<Scope>>, f: F) -> Gc<Object<SpecialForm>>
where
  F: 'static + Fn(&mut Stack),
{
  new_object(
    scope.clone(),
    Object::new(special_form_kind(scope), SpecialForm::new(f)),
  )
}

#[inline]
pub fn function_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Function")
      .expect("failed to get Function Kind")
  }
}
#[inline]
pub fn new_function(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: Gc<dyn Value>,
) -> Gc<Object<Function>> {
  new_object(
    scope.clone(),
    Object::new(
      function_kind(scope.clone()),
      Function::new(name, scope, params, body),
    ),
  )
}
#[inline]
pub fn new_external_function<F>(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
{
  new_object(
    scope.clone(),
    Object::new(
      function_kind(scope.clone()),
      Function::new_external(name, scope, params, body),
    ),
  )
}

#[inline]
pub fn add_external_function<F, N>(
  mut scope: Gc<Object<Scope>>,
  name: N,
  params: ::alloc::vec::Vec<N>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
  N: ToString,
{
  let mut list = new_list(scope.clone());

  for param in params {
    list.push_back(new_symbol(scope.clone(), param).into_value());
  }

  let function = new_external_function(
    scope.clone(),
    Some(new_symbol(scope.clone(), name.to_string())),
    list,
    body,
  );
  scope.set(&(name.to_string()), function.clone().into_value());
  function
}

#[inline]
pub fn macro_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_type::<Kind>("Macro")
      .expect("failed to get Macro Kind")
  }
}
#[inline]
pub fn new_macro(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: Gc<dyn Value>,
) -> Gc<Object<Function>> {
  new_object(
    scope.clone(),
    Object::new(
      macro_kind(scope.clone()),
      Function::new(name, scope.clone(), params, body),
    ),
  )
}
#[inline]
pub fn new_external_macro<F>(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
{
  new_object(
    scope.clone(),
    Object::new(
      macro_kind(scope.clone()),
      Function::new_external(name, scope, params, body),
    ),
  )
}
