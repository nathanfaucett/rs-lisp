extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, PersistentScope, Value};

macro_rules! mul {
  ($name:ident, $right_kind:ty, $left_kind:ty) => {
    fn mul_isize_isize(
      scope: &Gc<Object<PersistentScope>>,
      args: &Gc<Object<PersistentVector>>,
    ) -> Gc<dyn Value> {
      let a = args
        .pop_front()
        .expect("failed to get a value")
        .downcast_ref::<Object<$right_kind>>()
        .expect("failed to downcast a");
      let b = args
        .pop_front()
        .expect("failed to get b value")
        .downcast_ref::<Object<$left_kind>>()
        .expect("failed to downcast b");

      runtime::new_isize(scope, a.value() * b.value() as $right_kind).into_value()
    }
  };
}

mul!(mul_usize_usize, usize, usize);
