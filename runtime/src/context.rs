use alloc::string::{String, ToString};

use super::{
  add_external_function, init_bool_kind, init_bool_scope, init_numbers_kind, init_numbers_scope,
  new_kind, new_object, scope_get_with_kind, scope_set, Escape, Function, GcAllocator, Keyword,
  Kind, LinkedMap, List, Map, Object, PersistentList, PersistentMap, PersistentScope,
  PersistentSet, PersistentVector, Set, SpecialForm, Symbol, Value, Vector,
};
use gc::Gc;

#[inline]
pub fn new_context() -> Gc<Object<PersistentScope>> {
  unsafe {
    let mut scope = init_root_scope();

    scope = init_nil_kind(&scope);
    scope = init_bool_kind(&scope);
    scope = init_char_kind(&scope);
    scope = init_string_kind(&scope);
    scope = init_numbers_kind(&scope);
    scope = Symbol::init_kind(&scope);
    scope = Keyword::init_kind(&scope);
    scope = Function::init_kind(&scope);
    scope = SpecialForm::init_kind(&scope);
    scope = Escape::init_kind(&scope);
    scope = List::init_kind(&scope);
    scope = LinkedMap::init_kind(&scope);
    scope = Vector::init_kind(&scope);
    scope = Map::init_kind(&scope);
    scope = Set::init_kind(&scope);
    scope = PersistentList::init_kind(&scope);
    scope = PersistentMap::init_kind(&scope);
    scope = PersistentSet::init_kind(&scope);
    scope = PersistentVector::init_kind(&scope);

    scope = init_numbers_scope(&scope);
    scope = init_bool_scope(&scope);
    scope = Value::init_scope(&scope);
    scope = Kind::init_scope(&scope);
    scope = GcAllocator::init_scope(&scope);
    scope = PersistentScope::init_scope(&scope);
    scope = SpecialForm::init_scope(&scope);
    scope = List::init_scope(&scope);
    scope = LinkedMap::init_scope(&scope);
    scope = Vector::init_scope(&scope);
    scope = Map::init_scope(&scope);
    scope = Set::init_scope(&scope);
    scope = PersistentList::init_scope(&scope);
    scope = PersistentMap::init_scope(&scope);
    scope = PersistentSet::init_scope(&scope);
    scope = PersistentVector::init_scope(&scope);

    scope = add_external_function(
      &scope,
      "global_error_handler",
      vec!["error"],
      global_error_handler,
    );

    scope
  }
}

#[inline]
fn global_error_handler(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let error = args
    .front()
    .map(Clone::clone)
    .unwrap_or_else(|| nil_value(scope).clone().into_value());

  panic!("{:?}", error)
}

#[inline]
unsafe fn init_root_scope() -> Gc<Object<PersistentScope>> {
  let mut scope: Gc<Object<PersistentScope>> = Gc::null();
  let mut gc_allocator = GcAllocator::new();
  let mut persistent_scope = PersistentScope::default();

  let kind_kind = gc_allocator.unsafe_maintain(Kind::new_kind_kind());

  persistent_scope = persistent_scope.set("Kind", kind_kind.clone().into_value());

  let persistent_scope_kind = gc_allocator.unsafe_alloc(Kind::new_kind_object::<PersistentScope>(
    kind_kind.clone(),
    "PersistentScope",
  ));

  persistent_scope = persistent_scope.set(
    "PersistentScope",
    persistent_scope_kind.clone().into_value(),
  );

  let gc_allocator_kind = gc_allocator.unsafe_alloc(Kind::new_kind_object::<GcAllocator>(
    kind_kind,
    "GcAllocator",
  ));

  persistent_scope = persistent_scope.set("GcAllocator", gc_allocator_kind.clone().into_value());

  let gc_allocator_object = Gc::new(Object::new(gc_allocator_kind.clone(), gc_allocator));

  persistent_scope = persistent_scope.set(
    "default_gc_allocator",
    gc_allocator_object.clone().into_value(),
  );

  scope.set_from_value(Object::new(persistent_scope_kind.clone(), persistent_scope));

  scope
}

#[inline]
fn init_nil_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
  let nil_kind = new_kind::<()>(&scope, "Nil");
  let nil_value = new_object(&scope, Object::new(nil_kind.clone(), ()));

  let scope = scope_set(&scope, "Nil", nil_kind.into_value());
  scope_set(&scope, "nil", nil_value.into_value())
}

#[inline]
fn init_char_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
  let character_kind = new_kind::<char>(&scope, "Char");
  scope_set(&scope, "Char", character_kind.into_value())
}

#[inline]
fn init_string_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
  let string_kind = new_kind::<String>(&scope, "String");
  scope_set(&scope, "String", string_kind.into_value())
}

#[inline]
pub fn nil_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Nil").expect("failed to get Nil Kind")
}
#[inline]
pub fn nil_value(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<()>> {
  scope_get_with_kind::<()>(scope, "nil").expect("failed to get nil value")
}

#[inline]
pub fn char_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Char").expect("failed to get Char Kind")
}
#[inline]
pub fn new_char(scope: &Gc<Object<PersistentScope>>, value: char) -> Gc<Object<char>> {
  new_object(scope, Object::new(char_kind(scope).clone(), value))
}

#[inline]
pub fn string_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "String").expect("failed to get String Kind")
}
#[inline]
pub fn new_string<T>(scope: &Gc<Object<PersistentScope>>, value: T) -> Gc<Object<String>>
where
  T: ToString,
{
  new_object(
    scope,
    Object::new(string_kind(scope).clone(), value.to_string()),
  )
}
