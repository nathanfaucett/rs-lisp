use std::fmt::Write;
use std::path::Path;

use gc::Gc;
use runtime::{
  add_external_function, new_context, new_string, nil_value, List, Object, Scope, Value,
};

use super::{loader, new_module};

pub struct Lisp {
  scope: Gc<Object<Scope>>,
}

impl Lisp {
  #[inline]
  pub fn new() -> Self {
    let scope = new_context();

    add_external_function(scope.clone(), "println", vec!["...args"], println);

    Lisp { scope }
  }

  #[inline]
  pub fn run(&self, filename_path: &Path) {
    let mut module = new_module(self.scope.clone(), None);
    module.set(
      new_string(self.scope.clone(), "dirname").into_value(),
      new_string(self.scope.clone(), ".").into_value(),
    );
    loader::load(
      self.scope.clone(),
      module,
      new_string(
        self.scope.clone(),
        filename_path
          .to_str()
          .expect("failed to move Path to string"),
      ),
    )
    .expect(&format!("failed to load module {:?}", filename_path));
  }
}

#[inline]
fn println(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
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
  nil_value(scope).into_value()
}
