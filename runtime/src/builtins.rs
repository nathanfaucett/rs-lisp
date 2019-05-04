use gc::Gc;

use super::{add_external_function, nil_value, Kind, List, Object, Scope, Value};

#[inline]
pub unsafe fn init_builtins(scope: &mut Gc<Object<Scope>>) {
    add_external_function(scope, "get-kind-data", get_kind_data);
    add_external_function(scope, "set-kind-data", set_kind_data);
}

#[inline]
fn get_kind_data(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let kind = args
        .pop_front()
        .expect("Invalid Argument provided for kind")
        .downcast::<Object<Kind>>()
        .expect("Invalid Argument provided for kind");
    let key = args.pop_front().expect("Invalid Argument provided for key");

    kind.get(&key)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(&scope).into_value())
}

#[inline]
fn set_kind_data(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let mut kind = args
        .pop_front()
        .expect("Invalid Argument provided for kind")
        .downcast::<Object<Kind>>()
        .expect("Invalid Argument provided for kind");
    let key = args.pop_front().expect("Invalid Argument provided for key");
    let value = args
        .pop_front()
        .expect("Invalid Argument provided for value");

    kind.set(key, value);

    nil_value(&scope).into_value()
}
