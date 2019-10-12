use std::path::{Path, PathBuf};

use gc::Gc;
use runtime::{eval, new_list, new_map, new_string, nil_value, Function, Map, Object, Scope};

#[inline]
pub fn resolve_module_path(parent_dirname_path: &Path, filename_path: &Path) -> PathBuf {
  if filename_path.starts_with(".") {
    parent_dirname_path
      .join(filename_path)
      .canonicalize()
      .expect("failed to resolve file")
  } else if filename_path.is_absolute() {
    filename_path.canonicalize().expect("file not found")
  } else {
    // TODO: implement module resolver
    unimplemented!()
  }
}

#[inline]
pub fn new_module(
  scope: Gc<Object<Scope>>,
  filename: Gc<Object<String>>,
  mut parent: Option<Gc<Object<Map>>>,
) -> Gc<Object<Map>> {
  let parent_dirname_path = parent
    .as_mut()
    .map(|parent| {
      parent
        .get(&new_string(scope.clone(), "dirname").into_value())
        .map(|dirname| {
          Path::new(
            dirname
              .downcast_ref::<Object<String>>()
              .expect("failed to downcast dirname to String")
              .value(),
          )
        })
        .unwrap_or_else(|| Path::new("."))
    })
    .unwrap_or_else(|| Path::new("."));
  let filename_path = Path::new(filename.value());

  let resolved_path = resolve_module_path(parent_dirname_path, filename_path);
  let dirname_path = resolved_path.parent().unwrap_or_else(|| Path::new("."));

  let loaders_string = new_string(scope.clone(), "loaders").into_value();
  let loaders = parent
    .as_mut()
    .map(|parent| {
      let has_loaders = parent.has(&loaders_string);

      if has_loaders {
        parent
          .get(&loaders_string)
          .unwrap()
          .clone()
          .downcast::<Object<Map>>()
          .expect("failed to downcast loaders to Map")
      } else {
        let loaders = new_map(scope.clone());
        parent.set(loaders_string.clone(), loaders.clone().into_value());
        loaders
      }
    })
    .unwrap_or_else(|| new_map(scope.clone()));
  let cache_string = new_string(scope.clone(), "cache").into_value();
  let cache = parent
    .as_mut()
    .map(|parent| {
      let has_cache = parent.has(&cache_string);

      if has_cache {
        parent
          .get(&cache_string)
          .unwrap()
          .clone()
          .downcast::<Object<Map>>()
          .expect("failed to downcast cache to Map")
      } else {
        let cache = new_map(scope.clone());
        parent.set(cache_string.clone(), cache.clone().into_value());
        cache
      }
    })
    .unwrap_or_else(|| new_map(scope.clone()));
  let mut module = new_map(scope.clone());
  module.set(
    new_string(scope.clone(), "parent").into_value(),
    parent
      .map(|parent| parent.into_value())
      .unwrap_or_else(|| nil_value(scope.clone()).into_value()),
  );
  module.set(
    new_string(scope.clone(), "filename").into_value(),
    new_string(scope.clone(), resolved_path.to_str().unwrap()).into_value(),
  );
  module.set(
    new_string(scope.clone(), "dirname").into_value(),
    new_string(scope, dirname_path.to_str().unwrap()).into_value(),
  );
  module.set(loaders_string, loaders.into_value());
  module.set(cache_string, cache.into_value());
  module
}

#[inline]
pub fn load_module(
  scope: Gc<Object<Scope>>,
  parent_module: Gc<Object<Map>>,
  filename: Gc<Object<String>>,
) -> Gc<Object<Map>> {
  let parent_dirname = parent_module
    .get(&new_string(scope.clone(), "dirname").into_value())
    .expect("failed to get dirname from module")
    .clone()
    .downcast::<Object<String>>()
    .expect("failed to downcast dirname to String");
  let cache = parent_module
    .get(&new_string(scope.clone(), "cache").into_value())
    .expect("failed to get cache from module")
    .clone()
    .downcast::<Object<Map>>()
    .expect("failed to downcast cache to Map");
  let resolved_path = resolve_module_path(
    Path::new(parent_dirname.value()),
    Path::new(filename.value()),
  );
  let filename = new_string(scope.clone(), resolved_path.to_str().unwrap());
  let filename_value = filename.clone().into_value();
  let in_cache = cache.has(&filename_value);

  if in_cache {
    cache
      .get(&filename_value)
      .unwrap()
      .clone()
      .downcast::<Object<Map>>()
      .expect("failed to downcast to Map")
  } else {
    let module = new_module(scope.clone(), filename.clone(), Some(parent_module.clone()));
    let loaders = parent_module
      .get(&new_string(scope.clone(), "loaders").into_value())
      .expect("failed to get loaders from module")
      .clone()
      .downcast::<Object<Map>>()
      .expect("failed to downcast loaders to Map");
    let ext = resolved_path
      .extension()
      .and_then(|ext| ext.to_str())
      .unwrap_or(".lisp");
    let loader = loaders
      .get(&new_string(scope.clone(), ext).into_value())
      .expect(&format!("no loader for {}", ext))
      .clone()
      .downcast::<Object<Function>>()
      .expect("failed to downcast to Function");
    let mut list = new_list(scope.clone());
    list.push_front(module.clone().into_value());
    list.push_front(loader.into_value());
    eval(scope, list.into_value());
    module
  }
}
