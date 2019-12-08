use super::{add_external_function, new_kind, new_object, Kind, List, Object, Scope, Value};
use gc::Gc;

#[inline]
pub fn init_bool_kind(mut scope: Gc<Object<Scope>>) {
  let boolean_kind = new_kind::<bool>(scope.clone(), "Bool");
  let true_value = new_object(scope.clone(), Object::new(boolean_kind.clone(), true));
  let false_value = new_object(scope.clone(), Object::new(boolean_kind.clone(), false));

  scope.set("Bool", boolean_kind.into_value());
  scope.set("true", true_value.into_value());
  scope.set("false", false_value.into_value());
}

#[inline]
pub fn init_bool_scope(scope: Gc<Object<Scope>>) {
  add_external_function(scope, "bool.not", vec!["value"], bool_not);
}

#[inline]
pub fn bool_not(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let value = args
    .pop_front()
    .unwrap_or_else(|| false_value(scope.clone()).into_value())
    .downcast::<Object<bool>>()
    .expect("Failed to downcast value to bool");

  new_bool(scope, !*value.value()).into_value()
}

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
