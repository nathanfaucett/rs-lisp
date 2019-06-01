extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

fn mul(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
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

    runtime::new_isize(&scope, a.value() * b.value()).into_value()
}

fn sub(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
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

    runtime::new_isize(&scope, a.value() - b.value()).into_value()
}

fn eq(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
    let a = args.pop_front().expect("failed to get a value");
    let b = args.pop_front().expect("failed to get b value");
    runtime::new_bool(&scope, a.eq(&b)).into_value()
}

fn main() {
    let mut scope = runtime::new();

    runtime::add_external_function(&mut scope, "=", eq);
    runtime::add_external_function(&mut scope, "-", sub);
    runtime::add_external_function(&mut scope, "*", mul);

    let raw = concat!("(do ", include_str!("fac.lisp"), ")");

    let output = runtime::run(&scope, raw);
    println!("{:?}", output);
}
