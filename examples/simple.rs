extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

fn println(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
    println!("{:?}", args);
    runtime::nil_value(&scope).into_value()
}

fn main() {
    let mut scope = runtime::new();

    runtime::add_external_function(&mut scope, "println", println);

    let raw = concat!("(do ", include_str!("simple.lisp"), ")");

    let output = runtime::run(&scope, raw);
    println!("{:?}", output);
}
