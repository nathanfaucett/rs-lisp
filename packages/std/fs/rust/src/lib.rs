extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, Scope, Value};

#[no_mangle]
pub fn load(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {}
