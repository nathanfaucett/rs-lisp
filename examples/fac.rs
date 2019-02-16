extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

fn mul(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let a = args
        .pop_front()
        .expect("failed to get a value")
        .downcast::<Object<usize>>()
        .expect("failed to downcast a to usize");
    let b = args
        .pop_front()
        .expect("failed to get b value")
        .downcast::<Object<usize>>()
        .expect("failed to downcast b to usize");

    runtime::new_usize(&scope, a.value() * b.value()).into_value()
}

fn sub(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let a = args
        .pop_front()
        .expect("failed to get a value")
        .downcast::<Object<usize>>()
        .expect("failed to downcast a to usize");
    let b = args
        .pop_front()
        .expect("failed to get b value")
        .downcast::<Object<usize>>()
        .expect("failed to downcast b to usize");

    runtime::new_usize(&scope, a.value() - b.value()).into_value()
}

fn eq(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
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

    let ast = runtime::read(&scope, raw);
    println!("ast {:?}", ast);

    let output = runtime::eval(&scope, ast);
    println!("result {:?}", output);
}
