use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{Object, Scope};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Module {
    name: String,
    parent: Option<Gc<Object<Module>>>,
    modules: HashMap<String, Gc<Object<Module>>>,
    scope: Gc<Object<Scope>>,
}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Module {
    #[inline]
    pub fn new_root(scope: Gc<Object<Scope>>) -> Self {
        Module {
            name: "::".into(),
            parent: None,
            modules: HashMap::new(),
            scope: scope,
        }
    }

    #[inline]
    pub fn new(name: String, parent: Option<Gc<Object<Module>>>, scope: Gc<Object<Scope>>) -> Self {
        Module {
            name: name,
            parent: parent,
            modules: HashMap::new(),
            scope: scope,
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<&Gc<Object<Module>>> {
        self.parent.as_ref()
    }

    #[inline]
    pub fn parent_mut(&mut self) -> Option<&mut Gc<Object<Module>>> {
        self.parent.as_mut()
    }

    #[inline]
    pub fn scope(&self) -> &Gc<Object<Scope>> {
        &self.scope
    }

    #[inline]
    pub fn scope_mut(&mut self) -> &mut Gc<Object<Scope>> {
        &mut self.scope
    }

    #[inline]
    pub fn module(&self, ident: &str) -> Option<&Gc<Object<Module>>> {
        self.modules.get(ident)
    }
    #[inline]
    pub fn module_mut(&mut self, ident: &str) -> Option<&mut Gc<Object<Module>>> {
        self.modules.get_mut(ident)
    }

    #[inline]
    pub(crate) fn add_module(&mut self, ident: String, module: Gc<Object<Module>>) -> &mut Self {
        self.modules.insert(ident, module);
        self
    }
}
