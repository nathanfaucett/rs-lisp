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

  runtime::new_isize(scope, a.value() * b.value()).into_value()
}
