use alloc::string::{String, ToString};
use core::cmp::Ordering;
use core::fmt::{self, Debug, Write};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{self, AtomicPtr};

use gc::{Gc, Trace};
use hashbrown::hash_map::{IntoIter, Iter};
use hashbrown::HashMap;

use super::{new_object, Kind, Object, Value};

pub struct Scope {
    map: AtomicPtr<HashMap<String, Gc<dyn Value>>>,
    parent: Option<Gc<Object<Scope>>>,
}

impl Default for Scope {
    #[inline]
    fn default() -> Self {
        Scope::new(HashMap::default(), None)
    }
}

impl Drop for Scope {
    #[inline]
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.map.load(atomic::Ordering::SeqCst)) };
    }
}

impl Clone for Scope {
    #[inline]
    fn clone(&self) -> Self {
        Scope::new(self.inner().clone(), self.parent.clone())
    }
}

impl PartialEq for Scope {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner().eq(other.inner())
    }
}

impl Eq for Scope {}

impl PartialOrd for Scope {
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Trace for Scope {
    #[inline]
    fn trace(&mut self, marked: bool) {
        for (_k, v) in self.inner_mut().iter_mut() {
            v.trace(marked);
        }
        self.parent.trace(marked);
    }
}

impl Hash for Scope {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl fmt::Debug for Scope {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('{')?;
        let mut index = self.len();

        for (key, value) in self.inner().iter() {
            write!(f, "{:?} {:?}", key, value)?;

            index -= 1;
            if index != 0 {
                write!(f, ", ")?;
            }
        }

        f.write_char('}')
    }
}

impl IntoIterator for Scope {
    type Item = (String, Gc<dyn Value>);
    type IntoIter = IntoIter<String, Gc<dyn Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner().clone().into_iter()
    }
}

impl<'a> IntoIterator for &'a Scope {
    type Item = (&'a String, &'a Gc<dyn Value>);
    type IntoIter = Iter<'a, String, Gc<dyn Value>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner().iter()
    }
}

impl Deref for Scope {
    type Target = HashMap<String, Gc<dyn Value>>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl DerefMut for Scope {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl Scope {
    #[inline]
    pub fn new(map: HashMap<String, Gc<dyn Value>>, parent: Option<Gc<Object<Scope>>>) -> Self {
        Scope {
            map: AtomicPtr::from(Box::into_raw(Box::new(map))),
            parent,
        }
    }

    #[inline(always)]
    fn inner(&self) -> &HashMap<String, Gc<dyn Value>> {
        unsafe { &*self.map.load(atomic::Ordering::Relaxed) }
    }

    #[inline(always)]
    fn inner_mut(&self) -> &mut HashMap<String, Gc<dyn Value>> {
        unsafe { &mut *self.map.load(atomic::Ordering::SeqCst) }
    }

    #[inline]
    pub fn set(&self, key: &str, value: Gc<dyn Value>) -> &Self {
        self.inner_mut().insert(key.to_string(), value);
        self
    }

    #[inline]
    pub fn remove(&self, key: &str) -> &Self {
        self.inner_mut().remove(key);
        self
    }

    #[inline]
    pub fn has(&self, key: &str) -> bool {
        self.inner().contains_key(key)
    }

    #[inline]
    pub fn get_mut(&self, key: &str) -> Option<&mut Gc<dyn Value>> {
        self.inner_mut().get_mut(key)
    }
}

#[inline]
pub fn scope_kind(scope: &Gc<Object<Scope>>) -> &Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Scope").expect("failed to get Scope Kind")
}
#[inline]
pub fn new_scope(parent: &Gc<Object<Scope>>) -> Gc<Object<Scope>> {
    new_object(
        parent,
        Object::new(
            scope_kind(parent).clone(),
            Scope::new(HashMap::default(), Some(parent.clone())),
        ),
    )
}

#[inline]
pub fn scope_parent(scope: &Gc<Object<Scope>>) -> Option<&Gc<Object<Scope>>> {
    scope.parent.as_ref()
}

#[inline]
pub fn get_scope_root(scope: &Gc<Object<Scope>>) -> &Gc<Object<Scope>> {
    if let Some(parent) = scope_parent(scope) {
        get_scope_root(parent)
    } else {
        scope
    }
}

#[inline]
pub fn scope_get_by_value<'a>(
    scope: &'a Gc<Object<Scope>>,
    ident: &str,
) -> Option<&'a Gc<dyn Value>> {
    if let Some(value) = scope.get(ident) {
        return Some(value);
    } else if let Some(parent) = scope_parent(scope) {
        return scope_get_by_value(parent, ident);
    } else {
        None
    }
}
#[inline]
pub fn scope_get_mut_by_value<'a>(
    scope: &'a Gc<Object<Scope>>,
    ident: &str,
) -> Option<&'a mut Gc<dyn Value>> {
    if let Some(value) = scope.get_mut(ident) {
        return Some(value);
    } else if let Some(ref parent) = scope_parent(scope) {
        return scope_get_mut_by_value(parent, ident);
    } else {
        None
    }
}

#[inline]
pub fn scope_get<'a>(scope: &'a Gc<Object<Scope>>, ident: &str) -> Option<&'a Gc<dyn Value>> {
    scope_get_by_value(scope, ident)
}

#[inline]
pub fn scope_get_mut<'a>(
    scope: &'a Gc<Object<Scope>>,
    ident: &str,
) -> Option<&'a mut Gc<dyn Value>> {
    scope_get_mut_by_value(scope, ident)
}

#[inline]
pub fn scope_set<'a>(
    scope: &'a Gc<Object<Scope>>,
    ident: &str,
    value: Gc<dyn Value>,
) -> &'a Gc<Object<Scope>> {
    scope.set(ident, value);
    scope
}

#[inline]
pub fn scope_get_with_kind<'a, T>(
    scope: &'a Gc<Object<Scope>>,
    ident: &str,
) -> Option<&'a Gc<Object<T>>>
where
    T: 'static + Hash + Debug + PartialEq + PartialOrd + Trace,
{
    scope_get(scope, ident).and_then(|value| value.downcast_ref::<Object<T>>())
}

#[inline]
pub fn scope_get_mut_with_kind<'a, T>(
    scope: &'a Gc<Object<Scope>>,
    ident: &str,
) -> Option<&'a mut Gc<Object<T>>>
where
    T: 'static + Hash + Debug + PartialEq + PartialOrd + Trace,
{
    scope_get_mut(scope, ident).and_then(|value| value.downcast_mut::<Object<T>>())
}
