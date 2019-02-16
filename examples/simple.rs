extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

fn println(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<Value> {
    println!("println {:?}", args);
    runtime::nil_value(&scope).into_value()
}

fn main() {
    let mut scope = runtime::new();

    runtime::add_external_function(&mut scope, "println", println);

    let raw = concat!("(do ", include_str!("simple.lisp"), ")");

    let ast = runtime::read(&scope, raw);
    println!("ast before {:?}", ast);

    let output = runtime::eval(&scope, ast);
    println!("result {:?}", output);

    let ast = runtime::read(&scope, raw);
    println!("ast after {:?}", ast);
}
