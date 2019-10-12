extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

macro_rules! mul {
  ($name:ident, $right_type:ty, $left_type:ty) => {
    fn mul_isize_isize(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
      let a = args
        .pop_front()
        .expect("failed to get a value")
        .downcast::<Object<$right_type>>()
        .expect("failed to downcast a");
      let b = args
        .pop_front()
        .expect("failed to get b value")
        .downcast::<Object<$left_type>>()
        .expect("failed to downcast b");

      runtime::new_isize(scope, a.value() * b.value() as $right_type).into_value()
    }
  };
}

mul!(mul_usize_usize, usize, usize);
