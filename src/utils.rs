use std::string::ToString;

use gc::Gc;
use runtime::{run, Object, Scope, Value};

#[inline]
pub fn get_scope_root(scope: Gc<Object<Scope>>) -> Gc<Object<Scope>> {
  if let Some(parent) = scope.parent() {
    get_scope_root(parent.clone())
  } else {
    scope
  }
}

#[inline]
pub fn run_in_scope<T>(scope: Gc<Object<Scope>>, content: T) -> Gc<dyn Value>
where
  T: ToString,
{
  let mut raw = String::new();
  raw.push_str("(do ");
  raw.push_str(&content.to_string());
  raw.push(')');
  run(scope, raw)
}
