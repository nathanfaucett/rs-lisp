use std::fs::{canonicalize, read_to_string};
use std::path::Path;
use std::string::ToString;

use gc::Gc;
use runtime::{
  add_external_macro, new_map, new_scope, new_string, nil_value, run, List, Map, Object, Scope,
  Symbol, Value,
};

#[inline]
pub fn new_module(
  scope: Gc<Object<Scope>>,
  mut parent: Option<Gc<Object<Map>>>,
) -> Gc<Object<Map>> {
  let mut module = new_map(scope.clone());
  module.set(
    new_string(scope.clone(), "parent").into_value(),
    parent
      .as_ref()
      .map(|parent| parent.clone().into_value())
      .unwrap_or_else(|| nil_value(scope.clone()).into_value()),
  );
  module.set(
    new_string(scope.clone(), "exports").into_value(),
    new_map(scope.clone()).into_value(),
  );
  let cache_string = new_string(scope.clone(), "cache").into_value();
  module.set(
    cache_string.clone(),
    parent
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
      .unwrap_or_else(|| new_map(scope.clone()))
      .into_value(),
  );
  module
}

#[inline]
pub fn import(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut caller_scope = scope.parent().expect("failed to get caller scope").clone();
  let parent_module = scope
    .get("module")
    .expect("module is not defined in the current Scope")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast current module to Scope");
  let filename = args
    .pop_back()
    .expect("filename is required")
    .downcast::<Object<String>>()
    .expect("filed to downcast filename to String");

  let root_scope = get_scope_root(scope.clone());
  let module = file_loader(root_scope.clone(), parent_module, filename.value());
  let exports = module
    .get(&new_string(scope.clone(), "exports").into_value())
    .expect("exports not defined in module")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast exports to Map");

  for arg in args.iter() {
    let import_name = arg
      .downcast_ref::<Object<Symbol>>()
      .expect("failed to downcast import_name to Symbol");
    let import_value = exports
      .get(&new_string(scope.clone(), import_name.value().inner()).into_value())
      .expect(&format!(
        "no such import {:?} defined in {:?}",
        import_name.value().inner(),
        filename.value()
      ))
      .clone();
    caller_scope.set(import_name.value().inner(), import_value);
  }

  nil_value(scope).into_value()
}

#[inline]
pub fn export(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
  let module = scope
    .get("module")
    .expect("module is not defined in the current Scope")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast module to Scope");
  let mut exports = module
    .get(&new_string(scope.clone(), "exports").into_value())
    .expect("exports not defined on module")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast exports to Map");

  for arg in args.iter() {
    let export_name = arg
      .downcast_ref::<Object<Symbol>>()
      .expect("failed to downcast import_name to Symbol");
    let export_value = scope
      .get(export_name.value().inner())
      .expect("no such value defined")
      .clone();
    exports.set(
      new_string(scope.clone(), export_name.value().inner()).into_value(),
      export_value,
    );
  }

  nil_value(scope).into_value()
}

#[inline]
pub fn file_loader<T>(
  scope: Gc<Object<Scope>>,
  parent_module: Gc<Object<Map>>,
  filename: T,
) -> Gc<Object<Map>>
where
  T: ToString,
{
  let parent_dirname_string = new_string(scope.clone(), "dirname").into_value();
  let parent_dirname = parent_module
    .get(&parent_dirname_string)
    .map(Clone::clone)
    .expect("parent dirname is nil")
    .downcast::<Object<String>>()
    .expect("Failed to downcast dirname to String");
  let filename_string = filename.to_string();
  let filename_path = Path::new(&filename_string);
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
    cache
      .get(&path_value)
      .map(Clone::clone)
      .and_then(|cache| cache.downcast::<Object<Map>>().ok())
      .expect("failed to get module from cache")
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

    module
  }
}

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
