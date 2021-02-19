use alloc::boxed::Box;
use core::{
    cmp, fmt, hash,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicPtr, Ordering},
};

use gc::{Gc, Trace};

use super::{
    add_external_function, new_kind, new_object, new_vector_from, nil_value, scope_get_with_kind,
    scope_set, Kind, Map, Object, Scope, Value, Vector,
};

pub struct Atom {
    atomic_ptr: AtomicPtr<Gc<dyn Value>>,
}

impl Trace for Atom {
    #[inline]
    fn trace(&mut self, marked: bool) {
        self.deref_mut().trace(marked)
    }
}

impl hash::Hash for Atom {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl PartialOrd for Atom {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl PartialEq for Atom {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl Eq for Atom {}

impl fmt::Debug for Atom {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Into<Gc<dyn Value>> for Atom {
    #[inline]
    fn into(self) -> Gc<dyn Value> {
        self.deref().clone()
    }
}

impl Deref for Atom {
    type Target = Gc<dyn Value>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl DerefMut for Atom {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl Atom {
    #[inline]
    pub fn new(value: Gc<dyn Value>) -> Self {
        Atom {
            atomic_ptr: AtomicPtr::from(Box::into_raw(Box::new(value))),
        }
    }

    #[inline(always)]
    fn inner(&self) -> &Gc<dyn Value> {
        unsafe { &*self.atomic_ptr.load(Ordering::Relaxed) }
    }

    #[inline(always)]
    fn inner_mut(&mut self) -> &mut Gc<dyn Value> {
        unsafe { &mut *self.atomic_ptr.load(Ordering::SeqCst) }
    }

    #[inline(always)]
    fn update(&self, value: Gc<dyn Value>) -> &Self {
        self.atomic_ptr
            .store(Box::into_raw(Box::new(value)), Ordering::Relaxed);
        self
    }

    #[inline]
    pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
        let atom_kind = new_kind::<Atom>(scope, "Atom");
        scope_set(scope, "Atom", atom_kind.into_value());
    }

    #[inline]
    pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
        add_external_function(scope, "atom.new", vec!["value"], atom_new);
        add_external_function(scope, "atom.get", vec!["atom"], atom_get);
        add_external_function(scope, "atom.set", vec!["atom", "value"], atom_set);
    }
}

#[inline]
pub fn atom_kind(scope: &Gc<Object<Scope>>) -> &Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Atom").expect("failed to get Atom Kind")
}

#[inline]
pub fn new_atom(scope: &Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<Object<Atom>> {
    new_atom_with_meta(scope, value, None)
}

#[inline]
pub fn new_atom_with_meta(
    scope: &Gc<Object<Scope>>,
    value: Gc<dyn Value>,
    meta: Option<Gc<Object<Map>>>,
) -> Gc<Object<Atom>> {
    new_object(
        scope,
        Object::new_with_meta(atom_kind(scope).clone(), Atom::new(value), meta),
    )
}

#[inline]
pub fn atom_new(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    new_atom(scope, atom_value_from_args(scope, args)).into_value()
}

fn atom_value_from_args(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    if args.is_empty() {
        nil_value(scope).clone().into_value()
    } else if args.len() == 1 {
        args.front().unwrap().clone()
    } else {
        args.clone().into_value()
    }
}

#[inline]
pub fn atom_get(_scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    args.front()
        .expect("Atom is nil")
        .downcast_ref::<Object<Atom>>()
        .expect("Failed to downcast to Atom")
        .inner()
        .clone()
}

#[inline]
pub fn atom_set(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let mut new_args = new_vector_from(scope, args.value().clone());
    let atom_value = new_args.pop_front().expect("Atom is nil");
    let atom = atom_value
        .downcast_ref::<Object<Atom>>()
        .expect("Failed to downcast to Atom");

    atom.update(atom_value_from_args(scope, &new_args));
    atom.clone().into_value()
}
