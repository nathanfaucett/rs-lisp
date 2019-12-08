use gc::Gc;
use runtime::{call_function, new_list, new_string, Function, Map, Object, Scope, Vec};

mod file_loader;
mod dylib;
mod dylib_loader;

pub use self::file_loader::*;
pub use self::dylib::*;
pub use self::dylib_loader::*;

#[inline]
pub fn load(
  scope: Gc<Object<Scope>>,
  parent_module: Gc<Object<Map>>,
  filename: Gc<Object<String>>,
) -> Option<Gc<Object<Map>>> {
  let loaders = parent_module
    .get(&new_string(scope.clone(), "loaders").into_value())
    .expect("Loaders is not defined in the current module")
    .clone()
    .downcast::<Object<Vec>>()
    .expect("Failed to downcast loaders to Vec");

  for value in loaders.iter() {
    let loader = value
      .clone()
      .downcast::<Object<Function>>()
      .expect("failed loader to downcast to Function");

    let mut loader_args = new_list(scope.clone());
    loader_args.push_front(filename.clone().into_value());
    loader_args.push_front(parent_module.clone().into_value());

    let result = call_function(scope.clone(), loader, loader_args)
      .downcast::<Object<Map>>()
      .ok();

    if result.is_some() {
      return result;
    }
  }

  None
}
