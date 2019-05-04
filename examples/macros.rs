extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

fn add(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let a = args
        .pop_front()
        .expect("failed to get a value")
        .downcast::<Object<isize>>()
        .expect("failed to downcast a to isize");
    let b = args
        .pop_front()
        .expect("failed to get b value")
        .downcast::<Object<isize>>()
        .expect("failed to downcast b to isize");

    runtime::new_isize(&scope, a.value() + b.value()).into_value()
}

fn main() {
    let mut scope = runtime::new();

    runtime::add_external_function(&mut scope, "+", add);

    let raw = concat!("(do ", include_str!("macros.lisp"), ")");

    let output = runtime::run(&scope, raw);
    println!("result {:?}", output);
}