use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::ptr;

use gc::{Gc, Trace};
use hashbrown::hash_map::{IntoIter, Iter, IterMut};
use hashbrown::HashMap;

use super::{
    add_external_function, new_bool, new_kind, new_object, new_usize, nil_value,
    scope_get_with_kind, scope_set, Kind, Object, Scope, Value, Vector,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Map(pub HashMap<Gc<dyn Value>, Gc<dyn Value>>);

impl PartialOrd for Map {
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Trace for Map {
    #[inline]
    fn trace(&mut self, marked: bool) {
        for (k, v) in self.0.iter_mut() {
            unsafe {
                k.unsafe_as_mut().trace(marked);
            }
            v.trace(marked);
        }
    }
}

impl From<HashMap<Gc<dyn Value>, Gc<dyn Value>>> for Map {
    #[inline]
    fn from(map: HashMap<Gc<dyn Value>, Gc<dyn Value>>) -> Self {
        Map(map)
    }
}

impl Hash for Map {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl fmt::Debug for Map {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('{')?;
        let mut index = self.len();

        for (key, value) in self.0.iter() {
            write!(f, "{:?} {:?}", key, value)?;

            index -= 1;
            if index != 0 {
                write!(f, ", ")?;
            }
        }

        f.write_char('}')
    }
}

impl IntoIterator for Map {
    type Item = (Gc<dyn Value>, Gc<dyn Value>);
    type IntoIter = IntoIter<Gc<dyn Value>, Gc<dyn Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (&'a Gc<dyn Value>, &'a Gc<dyn Value>);
    type IntoIter = Iter<'a, Gc<dyn Value>, Gc<dyn Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Map {
    type Item = (&'a Gc<dyn Value>, &'a mut Gc<dyn Value>);
    type IntoIter = IterMut<'a, Gc<dyn Value>, Gc<dyn Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Deref for Map {
    type Target = HashMap<Gc<dyn Value>, Gc<dyn Value>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    #[inline]
    pub fn new() -> Self {
        Map(HashMap::default())
    }

    #[inline]
    pub fn set(&mut self, key: Gc<dyn Value>, value: Gc<dyn Value>) -> &mut Self {
        self.0.insert(key, value);
        self
    }

    #[inline]
    pub fn has(&self, key: &Gc<dyn Value>) -> bool {
        self.0.contains_key(key)
    }

    #[inline]
    pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
        let map_kind = new_kind::<Map>(scope, "Map");
        scope_set(scope, "Map", map_kind.into_value());
    }

    #[inline]
    pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
        add_external_function(scope, "map.is_empty", vec!["map"], map_is_empty);
        add_external_function(scope, "map.len", vec!["map"], map_len);
        add_external_function(scope, "map.get", vec!["map", "key"], map_get);
        add_external_function(scope, "map.remove", vec!["map", "key"], map_remove);
        add_external_function(scope, "map.has", vec!["map", "key"], map_has);
        add_external_function(scope, "map.set", vec!["map", "key", "value"], map_set);
    }
}

#[inline]
pub fn map_is_empty(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let map = args
        .front()
        .expect("Map is nil")
        .downcast_ref::<Object<Map>>()
        .expect("Failed to downcast to Map");

    new_bool(scope, map.is_empty()).into_value()
}

#[inline]
pub fn map_len(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let map = args
        .front()
        .expect("Map is nil")
        .downcast_ref::<Object<Map>>()
        .expect("Failed to downcast to Map");

    new_usize(scope, map.len()).into_value()
}

#[inline]
pub fn map_has(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let map_value = args.front().expect("Map is nil").clone();
    let map = map_value
        .downcast_ref::<Object<Map>>()
        .expect("Failed to downcast to Map");
    let key = args.get(1).expect("key is nil");

    new_bool(scope, map.has(key)).into_value()
}

#[inline]
pub fn map_get(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let map_value = args.front().expect("Map is nil").clone();
    let map = map_value
        .downcast_ref::<Object<Map>>()
        .expect("Failed to downcast to Map");
    let key = args.get(1).expect("key is nil");

    map.get(key)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn map_remove(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let mut map_value = args.front().expect("Map is nil").clone();
    let map = map_value
        .downcast_mut::<Object<Map>>()
        .expect("Failed to downcast to Map");
    let key = args.get(1).expect("key is nil");

    map.remove(&key)
        .unwrap_or_else(|| nil_value(scope).clone().into_value())
}

#[inline]
pub fn map_set(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let mut map_value = args.front().expect("Map is nil").clone();
    let map = map_value
        .downcast_mut::<Object<Map>>()
        .expect("Failed to downcast to Map");
    let key = args.get(1).expect("key is nil").clone();
    let value = args
        .get(2)
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(scope).clone().into_value());

    map.set(key, value);
    map_value
}

#[inline]
pub fn map_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Map").expect("failed to get Map Kind")
}
#[inline]
pub fn new_map(scope: &Gc<Object<Scope>>) -> Gc<Object<Map>> {
    new_map_from(scope, Map::new())
}

#[inline]
pub fn new_map_from(scope: &Gc<Object<Scope>>, map: Map) -> Gc<Object<Map>> {
    new_object(scope, Object::new(map_kind(scope).clone(), map))
}

#[inline]
pub fn new_map_from_with_meta(
    scope: &Gc<Object<Scope>>,
    map: Map,
    meta: Option<Gc<Object<Map>>>,
) -> Gc<Object<Map>> {
    new_object(
        scope,
        Object::new_with_meta(map_kind(scope).clone(), map, meta),
    )
}
