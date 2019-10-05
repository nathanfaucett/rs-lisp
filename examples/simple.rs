extern crate lisp;

use std::fmt::Write;

use lisp::gc::Gc;
use lisp::runtime::{self, add_external_function, nil_value, List, Object, Scope, Value};

#[inline]
fn println(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut string = String::new();
  let mut index = args.value().len();

  for value in args.value() {
    write!(string, "{:?}", value).unwrap();

    index -= 1;
    if index != 0 {
      write!(string, ", ").unwrap();
    }
  }

  println!("{}", string);
  nil_value(scope).into_value()
}

fn main() {
  let scope = runtime::new_context();

  add_external_function(
    scope.clone(),
    scope.clone(),
    "println",
    vec!["...args"],
    println,
  );

  let raw = concat!("(do ", include_str!("simple.lisp"), ")");

  let output = runtime::run(scope, raw);
  println!("{:?}", output);
}
