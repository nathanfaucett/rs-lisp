use std::fs::{canonicalize, read_to_string};
use std::path::Path;

use gc::Gc;
use runtime::{
  add_external_macro, new_map, new_scope, new_string, nil_value, List, Map, Object, Scope, Value,
};

use super::super::{export, get_scope_root, import, new_module, run_in_scope};

#[inline]
pub fn file_loader_lisp_fn(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let module = args
    .pop_front()
    .expect("module not passed to file_loader")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast module to Map");
  let filename = args
    .pop_front()
    .expect("filename not passed to file_loader")
    .clone()
    .downcast::<Object<String>>()
    .expect("Failed to downcast filename to String");

  file_loader(scope.clone(), module, filename.value())
    .map(|module| module.into_value())
    .unwrap_or_else(|| nil_value(scope).into_value())
}

#[inline]
pub fn file_loader(
  scope: Gc<Object<Scope>>,
  parent_module: Gc<Object<Map>>,
  filename: &String,
) -> Option<Gc<Object<Map>>> {
  if filename.starts_with(".") || filename.starts_with("/") || filename.starts_with("\\") {
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
      let mut module_scope = new_scope(get_scope_root(scope.clone()));

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

      module_scope.set("module", module.clone().into_value());
      module_scope.set("__filename", path_value.clone());
      module_scope.set(
        "__dirname",
        new_string(
          scope.clone(),
          path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap(),
        )
        .into_value(),
      );

      add_external_macro(
        module_scope.clone(),
        "import",
        vec!["...imports", "module_path"],
        import,
      );
      add_external_macro(module_scope.clone(), "export", vec!["...exports"], export);

      run_in_scope(
        module_scope,
        read_to_string(path.clone()).expect("failed to load local path"),
      );

      Some(module)
    }
  } else {
    None
  }
}
