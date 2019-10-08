use std::fmt::Write;
use std::fs;
use std::path::Path;

use gc::Gc;
use runtime::{
  self, add_external_function, add_external_macro, new_context, new_map, new_scope, new_string,
  nil_value, List, Map, Object, Scope, Symbol, Value,
};

pub struct Lisp {
  scope: Gc<Object<Scope>>,
}

impl Lisp {
  #[inline]
  pub fn new() -> Self {
    let mut scope = new_context();

    add_external_function(scope.clone(), "println", vec!["...args"], println);
    let module_cache = new_map(scope.clone()).into_value();
    scope.set("_module_cache", module_cache);

    Lisp { scope }
  }

  #[inline]
  pub fn run(&self, filename_path: &Path) -> Gc<dyn Value> {
    add_module_scope(self.scope.clone(), filename_path);
    run_in_scope(self.scope.clone(), filename_path)
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

#[inline]
fn import(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<dyn Value> {
  let mut caller_scope = scope.parent().expect("failed to get caller scope").clone();
  let parent_module = scope
    .get("module")
    .expect("module is not defined in the current Scope")
    .clone()
    .downcast::<Object<Map>>()
    .expect("Failed to downcast current module to Scope");
  let dirname = parent_module
    .get(&new_string(scope.clone(), "dirname").into_value())
    .expect("dirname is not defined in parent_module")
    .downcast_ref::<Object<String>>()
    .expect("failed to downcast dirname to String");
  let filename = args
    .pop_back()
    .expect("filename is required")
    .downcast::<Object<String>>()
    .expect("filed to downcast filename to String");
  let full_filename = Path::new(dirname.value()).join(filename.value());
  let full_filename_str = full_filename
    .to_str()
    .expect("failed to get the full filename of the current module");
  let full_filename_value = new_string(scope.clone(), full_filename_str).into_value();

  let root_scope = get_scope_root(scope.clone());
  let mut _module_cache = root_scope
    .get("_module_cache")
    .expect("_module_cache is not defined")
    .clone()
    .downcast::<Object<Map>>()
    .expect("failed to downcast _module_cache to Map");

  let in_cache = _module_cache.has(&full_filename_value);

  let module = if in_cache {
    let module = _module_cache.get(&full_filename_value).unwrap();
    module
      .clone()
      .downcast::<Object<Map>>()
      .expect("failed to downcast module to Map")
  } else {
    let module_scope = new_scope(root_scope);
    let module = add_module_scope(module_scope.clone(), &full_filename);
    _module_cache.set(full_filename_value, module.clone().into_value());
    run_in_scope(module_scope.clone(), &full_filename);
    module
  };
  let exports = module
    .get(&new_string(scope.clone(), "exports").into_value())
    .expect("exports not defined on module")
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
        full_filename_str
      ))
      .clone();
    caller_scope.set(import_name.value().inner(), import_value);
  }

  nil_value(scope).into_value()
}

fn get_scope_root(scope: Gc<Object<Scope>>) -> Gc<Object<Scope>> {
  if let Some(parent) = scope.parent() {
    get_scope_root(parent.clone())
  } else {
    scope
  }
}

#[inline]
fn export(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<dyn Value> {
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
fn add_module_scope(mut scope: Gc<Object<Scope>>, filename_path: &Path) -> Gc<Object<Map>> {
  let mut module = new_map(scope.clone());

  scope.set("module", module.clone().into_value());

  module.set(
    new_string(scope.clone(), "dirname").into_value(),
    new_string(
      scope.clone(),
      filename_path
        .parent()
        .unwrap_or(Path::new(""))
        .to_str()
        .unwrap_or(""),
    )
    .into_value(),
  );
  module.set(
    new_string(scope.clone(), "filename").into_value(),
    new_string(scope.clone(), filename_path.to_str().unwrap_or("")).into_value(),
  );
  module.set(
    new_string(scope.clone(), "exports").into_value(),
    new_map(scope.clone()).into_value(),
  );

  add_external_macro(
    scope.clone(),
    "import",
    vec!["...imports", "module_path"],
    import,
  );
  add_external_macro(scope.clone(), "export", vec!["...exports"], export);

  module
}

#[inline]
fn run_in_scope(scope: Gc<Object<Scope>>, filename_path: &Path) -> Gc<dyn Value> {
  let mut raw = String::new();
  raw.push_str("(do ");
  raw.push_str(
    &fs::read_to_string(filename_path).expect(&format!("No such file {:?}", filename_path)),
  );
  raw.push(')');
  runtime::run(scope, raw)
}
