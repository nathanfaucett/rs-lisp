use std::fs::canonicalize;
use std::path::Path;

use gc::Gc;
use runtime::{new_map, new_string, nil_value, Map, Object, Scope, Value, Vector};

use super::super::{new_dylib, new_module};

#[inline]
pub fn dylib_loader_lisp_fn(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let module_value = args
        .get(0)
        .expect("module not passed to dylib_loader")
        .clone();
    let module = module_value
        .downcast_ref::<Object<Map>>()
        .expect("Failed to downcast module to Map");
    let filename_value = args
        .get(1)
        .expect("filename not passed to dylib_loader")
        .clone();
    let filename = filename_value
        .downcast_ref::<Object<String>>()
        .expect("Failed to downcast filename to String");

    dylib_loader(scope, module, filename.value())
        .map(|module| module.into_value())
        .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn dylib_loader(
    scope: &Gc<Object<Scope>>,
    parent_module: &Gc<Object<Map>>,
    filename: &String,
) -> Option<Gc<Object<Map>>> {
    if filename.ends_with(".so") {
        let parent_dirname_string = new_string(scope, "dirname").into_value();
        let parent_dirname_value = parent_module
            .get(&parent_dirname_string)
            .expect("parent dirname is nil");
        let parent_dirname = parent_dirname_value
            .downcast_ref::<Object<String>>()
            .expect("failed to downcast dirname to String");
        let filename_path = Path::new(filename);
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
            let exports_value = module
                .get_mut(&new_string(scope, "exports").into_value())
                .unwrap();
            let exports = exports_value.downcast_mut::<Object<Map>>().unwrap();

            exports.set(
                new_string(scope, path.file_stem().unwrap().to_str().unwrap()).into_value(),
                unsafe { new_dylib(scope, path.to_str().unwrap()).into_value() },
            );

            Some(module)
        }
    } else {
        None
    }
}
