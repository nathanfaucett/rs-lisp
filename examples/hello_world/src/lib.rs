extern crate lisp;

use lisp::{
  gc::Gc,
  runtime::{nil_value, Object, Scope, Value, Vector},
};

#[inline]
#[no_mangle]
pub fn lisp_hello_world(scope: &Gc<Object<Scope>>, _args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
  println!("Hello, world from Rust!");
  nil_value(scope).into_value()
}
