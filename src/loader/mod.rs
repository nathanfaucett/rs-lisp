use gc::Gc;
use runtime::{
  call_function, new_persistent_vector_from, new_string, Function, Map, Object, PersistentScope,
  PersistentVector, Vector,
};

mod dylib;
mod dylib_loader;
mod file_loader;

pub use self::dylib::*;
pub use self::dylib_loader::*;
pub use self::file_loader::*;

#[inline]
pub fn load(
  scope: &Gc<Object<PersistentScope>>,
  parent_module: Gc<Object<Map>>,
  filename: Gc<Object<String>>,
) -> Option<Gc<Object<Map>>> {
  let loaders_value = parent_module
    .get(&new_string(scope, "loaders").into_value())
    .expect("Loaders is not defined in the current module");
  let loaders = loaders_value
    .downcast_ref::<Object<Vector>>()
    .expect("Failed to downcast loaders to Vector");

  for value in loaders.iter() {
    let loader = value
      .downcast_ref::<Object<Function>>()
      .expect("failed loader to downcast to Function");

    let mut loader_args = PersistentVector::new();
    loader_args = loader_args.push(parent_module.clone().into_value());
    loader_args = loader_args.push(filename.clone().into_value());

    let (_new_scope, result_value) = call_function(
      scope,
      loader.clone(),
      new_persistent_vector_from(scope, loader_args),
    );
    let result = result_value.downcast_ref::<Object<Map>>();

    if result.is_some() {
      return result.map(Clone::clone);
    }
  }

  None
}
