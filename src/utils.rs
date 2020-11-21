use std::string::ToString;

use gc::Gc;
use runtime::{run, Object, PersistentScope, Value};

#[inline]
pub fn run_in_scope<T>(
  scope: &Gc<Object<PersistentScope>>,
  content: T,
) -> (Gc<Object<PersistentScope>>, Gc<dyn Value>)
where
  T: ToString,
{
  let mut raw = String::new();
  raw.push_str("(do ");
  raw.push_str(&content.to_string());
  raw.push(')');
  run(scope, raw)
}
