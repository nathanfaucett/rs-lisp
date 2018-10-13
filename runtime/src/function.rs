use std::fmt;

use gc::Gc;

use super::{List, Object, Scope, Value};

#[derive(PartialEq, Eq, Hash)]
pub struct Function {
    name: Option<Gc<Object<String>>>,
    scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: Gc<Value>,
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
        name: Option<Gc<Object<String>>>,
        scope: Gc<Object<Scope>>,
        params: Gc<Object<List>>,
        body: Gc<Value>,
    ) -> Self {
        Function {
            name: name,
            scope: scope,
            params: params,
            body: body,
        }
    }

    #[inline(always)]
    pub fn name(&self) -> &Option<Gc<Object<String>>> {
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
    pub fn body(&self) -> &Gc<Value> {
        &self.body
    }
}
