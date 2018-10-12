use gc::Gc;

use super::{List, Object, Scope, Value};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Function {
    name: Option<Gc<Object<String>>>,
    scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: Gc<Value>,
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
