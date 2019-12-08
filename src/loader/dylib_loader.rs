use std::fs::canonicalize;
use std::path::Path;

use gc::Gc;
use runtime::{new_map, new_string, nil_value, List, Map, Object, Scope, Value};

use super::super::{new_module, new_dylib};

#[inline]
pub fn dylib_loader_lisp_fn(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let module = args
    .pop_front()
    .expect("module not passed to dylib_loader")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast module to Map");
  let filename = args
    .pop_front()
    .expect("filename not passed to dylib_loader")
    .clone()
    .downcast::<Object<String>>()
    .expect("Failed to downcast filename to String");

  dylib_loader(scope.clone(), module, filename.value())
    .map(|module| module.into_value())
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn dylib_loader(
  scope: Gc<Object<Scope>>,
  parent_module: Gc<Object<Map>>,
  filename: &String,
) -> Option<Gc<Object<Map>>> {
  if filename.ends_with(".so") {
    let parent_dirname_string = new_string(scope.clone(), "dirname").into_value();
    let parent_dirname = parent_module
      .get(&parent_dirname_string)
      .map(Clone::clone)
      .expect("parent dirname is nil")
      .downcast::<Object<String>>()
      .expect("Failed to downcast dirname to String");
    let filename_path = Path::new(filename);
    let parent_dirname_path = Path::new(parent_dirname.value());
    let path =
      canonicalize(parent_dirname_path.join(filename_path)).expect("failed to find local path");
    let path_value = new_string(scope.clone(), path.clone().to_str().unwrap()).into_value();

    let mut cache = parent_module
      .get(&new_string(scope.clone(), "cache").into_value())
      .map(Clone::clone)
      .and_then(|cache| cache.downcast::<Object<Map>>().ok())
      .unwrap_or_else(|| new_map(scope.clone()));

    if cache.has(&path_value) {
      Some(
        cache
          .get(&path_value)
          .map(Clone::clone)
          .and_then(|cache| cache.downcast::<Object<Map>>().ok())
          .expect("failed to get module from cache"),
      )
    } else {
      let mut module = new_module(scope.clone(), Some(parent_module));

      cache.set(path_value.clone(), module.clone().into_value());

      module.set(
        new_string(scope.clone(), "filename").into_value(),
        path_value.clone(),
      );
      module.set(
        new_string(scope.clone(), "dirname").into_value(),
        new_string(
          scope.clone(),
          path
            .parent()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap_or(""),
        )
        .into_value(),
      );
      let mut exports = module
        .get(&new_string(scope.clone(), "exports").into_value())
        .unwrap()
        .clone()
        .downcast::<Object<Map>>()
        .unwrap();

      exports.set(
        new_string(scope.clone(), path.file_stem().unwrap().to_str().unwrap()).into_value(),
        new_dylib(scope.clone(), path.to_str().unwrap()).into_value(),
      );

      Some(module)
    }
  } else {
    None
  }
}
