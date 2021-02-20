use super::{
    add_external_function, new_kind, new_object, scope_get_with_kind, scope_set, Kind, Object,
    Scope, Value, Vector,
};
use gc::Gc;

#[inline]
pub fn init_bool_kind(scope: &Gc<Object<Scope>>) {
    let boolean_kind = new_kind::<bool>(scope, "Bool");
    let true_value = new_object(scope, Object::new(boolean_kind.clone(), true));
    let false_value = new_object(scope, Object::new(boolean_kind.clone(), false));

    scope_set(scope, "Bool", boolean_kind.into_value());
    scope_set(scope, "true", true_value.into_value());
    scope_set(scope, "false", false_value.into_value());
}

#[inline]
pub fn init_bool_scope(scope: &Gc<Object<Scope>>) {
    add_external_function(scope, "bool.not", vec!["value"], bool_not);
}

#[inline]
pub fn bool_not(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let value = args
        .front()
        .map(Clone::clone)
        .unwrap_or_else(|| false_value(&scope).clone().into_value());
    let boolean = value
        .downcast_ref::<Object<bool>>()
        .expect("Failed to downcast value to bool");

    new_bool(&scope, !*boolean.value()).clone().into_value()
}

#[inline]
pub fn bool_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Bool").expect("failed to get Bool Kind")
}

#[inline]
pub fn true_value(scope: &Gc<Object<Scope>>) -> Gc<Object<bool>> {
    scope_get_with_kind::<bool>(scope, "true").expect("failed to get true value")
}
#[inline]
pub fn false_value(scope: &Gc<Object<Scope>>) -> Gc<Object<bool>> {
    scope_get_with_kind::<bool>(scope, "false").expect("failed to get false value")
}

#[inline]
pub fn new_bool(scope: &Gc<Object<Scope>>, value: bool) -> Gc<Object<bool>> {
    if value {
        true_value(scope).clone()
    } else {
        false_value(scope).clone()
    }
}
