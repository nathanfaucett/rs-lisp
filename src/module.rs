use std::ops::Deref;

use gc::Gc;
use runtime::{
  new_external_function, new_map, new_persistent_vector_from, new_string, new_symbol, new_vector,
  nil_value, scope_get, scope_get_mut, scope_parent, Map, Object, PersistentScope,
  PersistentVector, Symbol, Value, Vector,
};

use super::{dylib_loader_lisp_fn, file_loader_lisp_fn, get_scope_root, load};

#[inline]
pub fn new_module(
  scope: &Gc<Object<PersistentScope>>,
  mut parent: Option<Gc<Object<Map>>>,
) -> Gc<Object<Map>> {
  let mut module = new_map(scope);
  module.set(
    new_string(scope, "parent").into_value(),
    parent
      .as_ref()
      .map(|parent| parent.clone().into_value())
      .unwrap_or_else(|| nil_value(scope).clone().into_value()),
  );
  module.set(
    new_string(scope, "exports").into_value(),
    new_map(scope).into_value(),
  );
  let cache_string = new_string(scope, "cache").into_value();
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
            .downcast_ref::<Object<Map>>()
            .expect("failed to downcast cache to Map")
            .clone()
        } else {
          let cache = new_map(scope);
          parent.set(cache_string.clone(), cache.clone().into_value());
          cache
        }
      })
      .unwrap_or_else(|| new_map(scope))
      .into_value(),
  );
  let loaders_string = new_string(scope, "loaders").into_value();
  module.set(
    loaders_string.clone(),
    parent
      .as_mut()
      .map(|parent| {
        parent
          .get(&loaders_string)
          .expect("failed to get loaders from parent module")
          .downcast_ref::<Object<Vector>>()
          .expect("failed to downcast loaders to Vec")
          .clone()
      })
      .unwrap_or_else(|| {
        let mut loaders = new_vector(scope);

        let mut loader_params = PersistentVector::new();
        loader_params = loader_params.push_front(new_symbol(scope, "filename").into_value());
        loader_params = loader_params.push_front(new_symbol(scope, "module").into_value());
        let params = new_persistent_vector_from(scope, loader_params.clone());

        // Order matters here
        loaders.push(
          new_external_function(
            scope,
            Some(new_symbol(scope, "dylib_loader")),
            params.clone(),
            dylib_loader_lisp_fn,
          )
          .into_value(),
        );
        loaders.push(
          new_external_function(
            scope,
            Some(new_symbol(scope, "file_loader")),
            params,
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
pub fn import(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let caller_scope = scope_parent(scope)
    .expect("failed to get caller scope")
    .clone();
  let parent_module = scope_get(scope, "module")
    .expect("module is not defined in the current PersistentScope")
    .downcast_ref::<Object<Map>>()
    .expect("Failed to downcast current module to PersistentScope")
    .clone();
  let filename = args
    .back()
    .expect("filename is required")
    .downcast_ref::<Object<String>>()
    .expect("filed to downcast filename to String")
    .clone();

  let root_scope = get_scope_root(scope);
  let mut module = load(root_scope, parent_module, filename.clone())
    .expect(&format!("No Loader found for {}", filename.value()));
  let exports_value = module
    .get_mut(&new_string(scope, "exports").into_value())
    .expect("exports not defined in module");
  let exports = exports_value
    .downcast_mut::<Object<Map>>()
    .expect("Failed to downcast exports to Map");

  for import_name_value in args.iter() {
    let import_name = import_name_value
      .downcast_ref::<Object<Symbol>>()
      .expect("failed to downcast import_name to Symbol");
    let import_value = exports
      .get(&new_string(scope, import_name.value().deref()).into_value())
      .expect(&format!(
        "no such import {:?} defined in {:?}",
        import_name.value().deref(),
        filename.value()
      ))
      .clone();
    caller_scope.set(import_name.value().deref(), import_value);
  }

  nil_value(scope).clone().into_value()
}

#[inline]
pub fn export(
  scope: &Gc<Object<PersistentScope>>,
  args: &Gc<Object<PersistentVector>>,
) -> Gc<dyn Value> {
  let module_value =
    scope_get_mut(scope, "module").expect("module is not defined in the current PersistentScope");
  let module = module_value
    .downcast_mut::<Object<Map>>()
    .expect("Failed to downcast module to PersistentScope");
  let exports_value = module
    .get_mut(&new_string(scope, "exports").into_value())
    .expect("exports not defined on module");
  let exports = exports_value
    .downcast_mut::<Object<Map>>()
    .expect("Failed to downcast exports to Map");

  for export_name_value in args.iter() {
    let export_name = export_name_value
      .downcast_ref::<Object<Symbol>>()
      .expect("failed to downcast import_name to Symbol");
    let export_value = scope_get(scope, export_name.value().deref())
      .expect("no such value defined")
      .clone();
    exports.set(
      new_string(scope, export_name.value().deref()).into_value(),
      export_value,
    );
  }

  nil_value(scope).clone().into_value()
}
