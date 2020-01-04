use alloc::string::{String, ToString};

use super::{
  add_external_function, init_bool_kind, init_bool_scope, init_numbers_kind, init_numbers_scope,
  new_kind, new_object, Escape, Function, GcAllocator, Keyword, Kind, LinkedMap, List, Map, Object,
  PersistentList, PersistentMap, PersistentSet, PersistentVector, Scope, Set, SpecialForm, Symbol,
  Value, Vector,
};
use gc::Gc;

#[inline]
pub fn new_context() -> Gc<Object<Scope>> {
  unsafe {
    let scope = init_root_scope();

    init_nil(scope.clone());
    init_bool_kind(scope.clone());
    init_char(scope.clone());
    init_string(scope.clone());
    init_numbers_kind(scope.clone());
    Function::init_kind(scope.clone());
    SpecialForm::init_kind(scope.clone());
    Symbol::init_kind(scope.clone());
    Keyword::init_kind(scope.clone());
    Escape::init_kind(scope.clone());
    List::init_kind(scope.clone());
    LinkedMap::init_kind(scope.clone());
    Vector::init_kind(scope.clone());
    Map::init_kind(scope.clone());
    Set::init_kind(scope.clone());
    PersistentList::init_kind(scope.clone());
    PersistentMap::init_kind(scope.clone());
    PersistentSet::init_kind(scope.clone());
    PersistentVector::init_kind(scope.clone());

    init_numbers_scope(scope.clone());
    init_bool_scope(scope.clone());
    Value::init_scope(scope.clone());
    Kind::init_scope(scope.clone());
    GcAllocator::init_scope(scope.clone());
    Scope::init_scope(scope.clone());
    List::init_scope(scope.clone());
    LinkedMap::init_scope(scope.clone());
    Vector::init_scope(scope.clone());
    Map::init_scope(scope.clone());
    Set::init_scope(scope.clone());
    PersistentList::init_scope(scope.clone());
    PersistentMap::init_scope(scope.clone());
    PersistentSet::init_scope(scope.clone());
    PersistentVector::init_scope(scope.clone());

    add_external_function(
      scope.clone(),
      "global_error_handler",
      vec!["error"],
      global_error_handler,
    );

    scope
  }
}

#[inline]
fn global_error_handler(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let error = args
    .pop_front()
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  panic!("{:?}", error)
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
fn init_nil(mut scope: Gc<Object<Scope>>) {
  let nil_kind = new_kind::<()>(scope.clone(), "Nil");
  let nil_value = new_object(scope.clone(), Object::new(nil_kind.clone(), ()));

  scope.set("Nil", nil_kind.into_value());
  scope.set("nil", nil_value.into_value());
}

#[inline]
fn init_char(mut scope: Gc<Object<Scope>>) {
  let character_kind = new_kind::<char>(scope.clone(), "Char");
  scope.set("Char", character_kind.into_value());
}

#[inline]
fn init_string(mut scope: Gc<Object<Scope>>) {
  let string_kind = new_kind::<String>(scope.clone(), "String");
  scope.set("String", string_kind.into_value());
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
