use gc::Gc;
use runtime::{new_list, new_map, new_string, nil_value, List, Map, Object, Scope};

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
  let loaders_string = new_string(scope.clone(), "loaders").into_value();
  module.set(
    loaders_string.clone(),
    parent
      .as_mut()
      .map(|parent| {
        let has_loaders = parent.has(&loaders_string);

        if has_loaders {
          parent
            .get(&loaders_string)
            .unwrap()
            .clone()
            .downcast::<Object<List>>()
            .expect("failed to downcast loaders to List")
        } else {
          let loaders = new_list(scope.clone());
          parent.set(loaders_string.clone(), loaders.clone().into_value());
          loaders
        }
      })
      .unwrap_or_else(|| new_list(scope.clone()))
      .into_value(),
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
