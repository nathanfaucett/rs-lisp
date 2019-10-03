extern crate lisp;

use std::fmt::Write;

use lisp::gc::Gc;
use lisp::runtime::{self, nil_value, List, Object, Scope, Value};

fn add(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
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

  runtime::new_isize(scope, a.value() + b.value()).into_value()
}

fn eq(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let a = args.pop_front().expect("failed to get a value");
  let b = args.pop_front().expect("failed to get b value");
  runtime::new_bool(scope, a.eq(&b)).into_value()
}

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
  let scope = runtime::new();

  runtime::add_external_function(scope.clone(), "=", vec!["a", "b"], eq);
  runtime::add_external_function(scope.clone(), "+", vec!["a", "b"], add);
  runtime::add_external_function(scope.clone(), "println", vec!["...args"], println);

  let raw = concat!("(do ", include_str!("for_each.lisp"), ")");

  let output = runtime::run(scope, raw);
  println!("{:?}", output);
}
