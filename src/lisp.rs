use std::fmt::Write;
use std::path::Path;

use gc::Gc;
use runtime::{
  add_external_function, new_context, new_string, nil_value, Object, PersistentScope,
  PersistentVector, Value,
};

use super::{loader, new_module, DyLib};

pub fn new() -> Gc<Object<PersistentScope>> {
  let mut scope = new_context();

  scope = DyLib::init_kind(&scope);
  scope = DyLib::init_scope(&scope);

  scope = add_external_function(&scope, "println", vec!["...args"], println);

  scope
}

#[inline]
pub fn run_path(scope: &Gc<Object<PersistentScope>>, filename_path: &Path) {
  let mut module = new_module(scope, None);
  module.set(
    new_string(scope, "dirname").into_value(),
    new_string(scope, ".").into_value(),
  );
  loader::load(
    scope,
    module,
    new_string(
      scope,
      filename_path
        .to_str()
        .expect("failed to move Path to string"),
    ),
  )
  .expect(&format!("failed to load module {:?}", filename_path));
}

#[inline]
fn println(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
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
  nil_value(scope).clone().into_value()
}
