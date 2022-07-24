use alloc::string::{String, ToString};
use hashbrown::HashMap;

use super::{
  add_external_function, init_bool_kind, init_bool_scope, init_numbers_kind, init_numbers_scope,
  new_kind, new_object, run_in_scope, scope_get_with_kind, scope_set, Atom, Escape, Function,
  GcAllocator, Keyword, Kind, List, Map, Object, Scope, Set, SpecialForm, Stack, Symbol, Value,
  Vector,
};
use gc::Gc;

#[inline]
pub fn new_context() -> Gc<Object<Scope>> {
  unsafe {
    let scope = init_root_scope();

    init_nil_kind(&scope);
    init_bool_kind(&scope);
    init_char_kind(&scope);
    init_string_kind(&scope);
    init_numbers_kind(&scope);
    Stack::init_kind(&scope);
    Atom::init_kind(&scope);
    Symbol::init_kind(&scope);
    Keyword::init_kind(&scope);
    Function::init_kind(&scope);
    SpecialForm::init_kind(&scope);
    Escape::init_kind(&scope);
    List::init_kind(&scope);
    Vector::init_kind(&scope);
    Map::init_kind(&scope);
    Set::init_kind(&scope);

    init_numbers_scope(&scope);
    init_bool_scope(&scope);
    Stack::init_scope(&scope);
    Atom::init_scope(&scope);
    <dyn Value>::init_scope(&scope);
    Kind::init_scope(&scope);
    GcAllocator::init_scope(&scope);
    SpecialForm::init_scope(&scope);
    List::init_scope(&scope);
    Vector::init_scope(&scope);
    Map::init_scope(&scope);
    Set::init_scope(&scope);

    add_external_function(
      &scope,
      "global_error_handler",
      vec!["error"],
      global_error_handler,
    );

    run_in_scope(&scope, include_str!("lisp/bootstrap.lisp"));

    scope
  }
}

#[inline]
fn global_error_handler(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
  let error = args
    .front()
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  panic!("{:#?}", error)
}

#[inline]
pub(crate) unsafe fn init_root_scope() -> Gc<Object<Scope>> {
  let mut gc_allocator = GcAllocator::new();
  let mut scope_builder = HashMap::default();

  let kind_kind = gc_allocator.unsafe_maintain(Kind::new_kind_kind());

  scope_builder.insert("Kind".to_string(), kind_kind.clone().into_value());

  let scope_kind =
    gc_allocator.unsafe_alloc(Kind::new_kind_object::<Scope>(kind_kind.clone(), "Scope"));
  scope_builder.insert("Scope".to_string(), scope_kind.clone().into_value());

  let gc_allocator_kind = gc_allocator.unsafe_alloc(Kind::new_kind_object::<GcAllocator>(
    kind_kind,
    "GcAllocator",
  ));
  scope_builder.insert(
    "GcAllocator".to_string(),
    gc_allocator_kind.clone().into_value(),
  );

  let mut gc_allocator_object = Gc::new(Object::new(gc_allocator_kind.clone(), gc_allocator));

  scope_builder.insert(
    "default_gc_allocator".to_string(),
    gc_allocator_object.clone().into_value(),
  );

  let scope = Gc::new(Object::new(
    scope_kind.clone(),
    Scope::new(scope_builder, None),
  ));

  gc_allocator_object.unsafe_maintain(scope)
}

#[inline]
fn init_nil_kind(scope: &Gc<Object<Scope>>) {
  let nil_kind = new_kind::<()>(&scope, "Nil");
  let nil_value = new_object(&scope, Object::new(nil_kind.clone(), ()));

  scope_set(&scope, "Nil", nil_kind.into_value());
  scope_set(&scope, "nil", nil_value.into_value());
}

#[inline]
fn init_char_kind(scope: &Gc<Object<Scope>>) {
  let character_kind = new_kind::<char>(&scope, "Char");
  scope_set(&scope, "Char", character_kind.into_value());
}

#[inline]
fn init_string_kind(scope: &Gc<Object<Scope>>) {
  let string_kind = new_kind::<String>(&scope, "String");
  scope_set(&scope, "String", string_kind.into_value());
}

#[inline]
pub fn nil_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Nil").expect("failed to get Nil Kind")
}
#[inline]
pub fn nil_value(scope: &Gc<Object<Scope>>) -> Gc<Object<()>> {
  scope_get_with_kind::<()>(scope, "nil").expect("failed to get nil value")
}

#[inline]
pub fn char_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Char").expect("failed to get Char Kind")
}
#[inline]
pub fn new_char(scope: &Gc<Object<Scope>>, value: char) -> Gc<Object<char>> {
  new_object(scope, Object::new(char_kind(scope).clone(), value))
}

#[inline]
pub fn string_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "String").expect("failed to get String Kind")
}
#[inline]
pub fn new_string<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<String>>
where
  T: ToString,
{
  new_object(
    scope,
    Object::new(string_kind(scope).clone(), value.to_string()),
  )
}
