use std::fmt::Write;
use std::path::Path;

use gc::Gc;
use runtime::{
  add_external_function, new_context, new_string, nil_value, Object, PersistentScope,
  PersistentVector, Value,
};

use super::{loader, new_module, DyLib};

pub struct Lisp {
  scope: Gc<Object<PersistentScope>>,
}

impl Lisp {
  #[inline]
  pub fn new() -> Self {
    let mut scope = new_context();

    scope = DyLib::init_kind(&scope);
    scope = DyLib::init_scope(&scope);

    scope = add_external_function(&scope, "println", vec!["...args"], println);

    Lisp { scope }
  }

  #[inline]
  pub fn run(&self, filename_path: &Path) {
    let mut module = new_module(&self.scope, None);
    module.set(
      new_string(&self.scope, "dirname").into_value(),
      new_string(&self.scope, ".").into_value(),
    );
    loader::load(
      &self.scope,
      module,
      new_string(
        &self.scope,
        filename_path
          .to_str()
          .expect("failed to move Path to string"),
      ),
    )
    .expect(&format!("failed to load module {:?}", filename_path));
  }
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
