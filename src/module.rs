use gc::Gc;
use runtime::{
  new_external_function, new_list, new_map, new_string, new_symbol, new_vec, nil_value, List, Map,
  Object, Scope, Symbol, Value, Vec,
};

use super::{file_loader_lisp_fn, get_scope_root, load, dylib_loader_lisp_fn};

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
  let loaders_string = new_string(scope.clone(), "loaders").into_value();
  module.set(
    loaders_string.clone(),
    parent
      .as_mut()
      .map(|parent| {
        parent
          .get(&loaders_string)
          .expect("failed to get loaders from parent module")
          .clone()
          .downcast::<Object<Vec>>()
          .expect("failed to downcast loaders to Vec")
      })
      .unwrap_or_else(|| {
        let mut loaders = new_vec(scope.clone());

        let mut loader_params = new_list(scope.clone());
        loader_params.push_front(new_symbol(scope.clone(), "filename").into_value());
        loader_params.push_front(new_symbol(scope.clone(), "module").into_value());

        // Order matters here
        loaders.push(
          new_external_function(
            scope.clone(),
            Some(new_symbol(scope.clone(), "dylib_loader")),
            loader_params.clone(),
            dylib_loader_lisp_fn,
          )
          .into_value(),
        );
        loaders.push(
          new_external_function(
            scope.clone(),
            Some(new_symbol(scope.clone(), "file_loader")),
            loader_params,
            file_loader_lisp_fn,
          )
          .into_value(),
        );

        loaders
      })
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
  let module = load(root_scope.clone(), parent_module, filename.clone())
    .expect(&format!("No Loader found for {}", filename.value()));
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
