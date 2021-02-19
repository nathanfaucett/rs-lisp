use std::fs::{canonicalize, read_to_string};
use std::path::Path;

use gc::Gc;
use runtime::{
    add_external_macro, get_scope_root, new_map, new_scope, new_string, nil_value, scope_set, Map,
    Object, Scope, Stack, Value, Vector,
};

use super::super::{export, import, new_module, run_in_scope};

#[inline]
pub fn file_loader_lisp_fn(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let module_value = args.get(0).expect("module not passed to file_loader");
    let module = module_value
        .downcast_ref::<Object<Map>>()
        .expect("Failed to downcast module to Map");
    let filename_value = args.get(1).expect("filename not passed to file_loader");
    let filename = filename_value
        .downcast_ref::<Object<String>>()
        .expect("Failed to downcast filename to String");

    file_loader(scope, module, filename.value())
        .map(|module| module.into_value())
        .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn file_loader(
    scope: &Gc<Object<Scope>>,
    parent_module: &Gc<Object<Map>>,
    filename: &String,
) -> Option<Gc<Object<Map>>> {
    if filename.starts_with(".") || filename.starts_with("/") || filename.starts_with("\\") {
        let parent_dirname_string = new_string(scope, "dirname").into_value();
        let parent_dirname = parent_module
            .get(&parent_dirname_string)
            .expect("parent dirname is nil")
            .downcast_ref::<Object<String>>()
            .expect("Failed to downcast dirname to String")
            .clone();

        let mut filename_path = Path::new(filename);
        let mut filename_with_ext = filename.clone();
        if filename_path.extension().is_none() {
            filename_with_ext.push_str(".lisp");
            filename_path = Path::new(&filename_with_ext);
        }

        let parent_dirname_path = Path::new(parent_dirname.value());
        let path = canonicalize(parent_dirname_path.join(filename_path))
            .expect("failed to find local path");
        let path_value = new_string(scope, path.clone().to_str().unwrap()).into_value();

        let mut cache = parent_module
            .get(&new_string(scope, "cache").into_value())
            .and_then(|cache| cache.downcast_ref::<Object<Map>>())
            .map(Clone::clone)
            .unwrap_or_else(|| new_map(scope));

        if cache.has(&path_value) {
            Some(
                cache
                    .get(&path_value)
                    .and_then(|cache| cache.downcast_ref::<Object<Map>>())
                    .map(Clone::clone)
                    .expect("failed to get module from cache"),
            )
        } else {
            let mut module = new_module(scope, Some(parent_module.clone()));
            let module_scope = new_scope(get_scope_root(scope));

            Stack::init_scope(&module_scope);

            cache.set(path_value.clone(), module.clone().into_value());

            module.set(
                new_string(scope, "filename").into_value(),
                path_value.clone(),
            );
            module.set(
                new_string(scope, "dirname").into_value(),
                new_string(
                    scope,
                    path.parent()
                        .unwrap_or(Path::new(""))
                        .to_str()
                        .unwrap_or(""),
                )
                .into_value(),
            );

            scope_set(&module_scope, "module", module.clone().into_value());
            scope_set(&module_scope, "__filename", path_value.clone());
            scope_set(
                &module_scope,
                "__dirname",
                new_string(
                    scope,
                    path.parent()
                        .unwrap_or_else(|| Path::new(""))
                        .to_str()
                        .unwrap(),
                )
                .into_value(),
            );

            add_external_macro(
                &module_scope,
                "import",
                vec!["...imports", "module_path"],
                import,
            );
            add_external_macro(&module_scope, "export", vec!["...exports"], export);

            run_in_scope(
                &module_scope,
                read_to_string(path.clone()).expect("failed to load local path"),
            );

            Some(module)
        }
    } else {
        None
    }
}
