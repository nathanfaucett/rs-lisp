use core::fmt;
use core::hash::{Hash, Hasher};

use gc::{Gc, Trace};

use super::{FunctionKind, List, Object, Scope, Symbol, Value};

pub struct Function {
    name: Option<Gc<Object<Symbol>>>,
    scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: FunctionKind,
}

impl Trace for Function {
    #[inline]
    fn mark(&mut self) {
        if let Some(v) = &mut self.name {
            v.mark();
        }
        self.scope.mark();
        for v in self.params.iter_mut() {
            v.mark();
        }
    }
}

impl Hash for Function {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.params.hash(state);
        self.body.hash(state);
    }
}

impl PartialEq for Function {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.body.eq(&other.body)
    }
}

impl fmt::Debug for Function {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("Function");

        if let Some(name) = self.name.as_ref() {
            debug.field("name", name);
        }

        debug
            .field("params", &self.params)
            .field("body", &self.body)
            .finish()
    }
}

impl Function {
    #[inline(always)]
    pub fn new(
        name: Option<Gc<Object<Symbol>>>,
        scope: Gc<Object<Scope>>,
        params: Gc<Object<List>>,
        body: Gc<dyn Value>,
    ) -> Self {
        Function {
            name: name,
            scope: scope,
            params: params,
            body: FunctionKind::new_internal(body),
        }
    }

    #[inline(always)]
    pub fn new_external<F>(
        name: Option<Gc<Object<Symbol>>>,
        scope: Gc<Object<Scope>>,
        params: Gc<Object<List>>,
        body: F,
    ) -> Self
    where
        F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
    {
        Function {
            name: name,
            scope: scope,
            params: params,
            body: FunctionKind::new_external(body),
        }
    }

    #[inline(always)]
    pub fn name(&self) -> &Option<Gc<Object<Symbol>>> {
        &self.name
    }
    #[inline(always)]
    pub fn scope(&self) -> &Gc<Object<Scope>> {
        &self.scope
    }
    #[inline(always)]
    pub fn params(&self) -> &Gc<Object<List>> {
        &self.params
    }
    #[inline(always)]
    pub fn body(&self) -> &FunctionKind {
        &self.body
    }
}
