use std::string::ToString;

use gc::Gc;
use runtime::{run, Object, Scope, Value};

#[inline]
pub fn run_in_scope<T>(scope: &Gc<Object<Scope>>, content: T) -> (Gc<Object<Scope>>, Gc<dyn Value>)
where
    T: ToString,
{
    let mut raw = String::from("(do ");
    raw.push_str(&content.to_string());
    raw.push(')');
    run(scope, raw)
}
