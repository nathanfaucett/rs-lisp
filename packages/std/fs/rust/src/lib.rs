extern crate lisp;

use lisp::gc::Gc;
use lisp::runtime::{self, List, Object, PersistentScope, Value};

#[no_mangle]
pub fn load(scope: &Gc<Object<PersistentScope>>, args: &Gc<Object<PersistentVector>>) -> Gc<dyn Value> {}
