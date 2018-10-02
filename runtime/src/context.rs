use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;
use std::hash::Hash;

use gc::Gc;

use super::{Kind, Module, Object, Scope, Value};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Context {
    module: Gc<Object<Module>>,
}

impl Context {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut context = Context { module: Gc::null() };
            context.init();
            context
        }
    }

    #[inline]
    pub unsafe fn init(&mut self) -> &mut Self {
        self.init_root()
            .init_boolean()
            .init_character()
            .init_keyword()
            .init_string()
            .init_numbers()
            .init_list()
            .init_vector()
            .init_map()
    }

    #[inline]
    pub fn root_module(&self) -> &Gc<Object<Module>> {
        &self.module
    }
    #[inline]
    pub fn root_module_mut(&mut self) -> &mut Gc<Object<Module>> {
        &mut self.module
    }

    #[inline]
    pub fn module(&mut self, path: &[&str]) -> Gc<Object<Module>> {
        let scope_kind = unsafe { self.get_global_type::<Kind>("Scope").unwrap() };
        let module_kind = unsafe { self.get_global_type::<Kind>("Module").unwrap() };

        let mut current = self.module.clone();

        for part in path {
            if let Some(module) = current.module(part).map(Clone::clone) {
                current = module;
            } else {
                let scope = unsafe {
                    Gc::new(Object::new(
                        scope_kind.clone(),
                        Scope::new(Some(current.scope().clone())),
                    ))
                };
                let module = unsafe {
                    Gc::new(Object::new(
                        module_kind.clone(),
                        Module::new(
                            (*part).to_owned(),
                            current.parent().map(Clone::clone),
                            scope,
                        ),
                    ))
                };
                current.add_module((*part).to_owned(), module.clone());
                current = module;
            }
        }

        current
    }

    #[inline]
    pub fn get_global(&self, ident: &str) -> Option<&Gc<Value>> {
        self.module.scope().get(ident)
    }
    #[inline]
    pub fn get_global_mut(&mut self, ident: &str) -> Option<&mut Gc<Value>> {
        self.module.scope_mut().get_mut(ident)
    }

    #[inline]
    pub unsafe fn get_global_type<T>(&self, ident: &str) -> Option<Gc<Object<T>>>
    where
        T: 'static + Hash + Debug + PartialEq,
    {
        self.module
            .scope()
            .get(ident)
            .map(|value| value.clone().into_object_unchecked())
    }

    #[inline]
    fn set_global(&mut self, ident: &str, value: Gc<Value>) -> &mut Self {
        self.module.scope_mut().set(ident, value);
        self
    }

    #[inline]
    unsafe fn init_root(&mut self) -> &mut Self {
        let type_kind = Kind::new_type_kind();
        let module_kind = Gc::new(Kind::new_kind::<Module>(type_kind.clone(), "Module"));
        let scope_kind = Gc::new(Kind::new_kind::<Scope>(type_kind.clone(), "Scope"));

        let root_module =
            Module::new_root(Gc::new(Object::new(scope_kind.clone(), Scope::new_root())));
        let root_module_object = Object::new(module_kind.clone(), root_module);

        self.module.set_if_null(root_module_object);

        self.set_global("Type", type_kind.into_value());
        self.set_global("Module", module_kind.into_value());
        self.set_global("Scope", scope_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_boolean(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let boolean_kind = Gc::new(Kind::new_kind::<bool>(type_kind, "Boolean"));
        self.set_global("Boolean", boolean_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_character(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let character_kind = Gc::new(Kind::new_kind::<char>(type_kind, "Character"));
        self.set_global("Character", character_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_keyword(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let keyword_kind = Gc::new(Kind::new_kind::<String>(type_kind, "Keyword"));
        self.set_global("Keyword", keyword_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_string(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let string_kind = Gc::new(Kind::new_kind::<String>(type_kind, "String"));
        self.set_global("String", string_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_numbers(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        // Unsigned
        let u8_kind = Gc::new(Kind::new_kind::<u8>(type_kind.clone(), "U8"));
        self.set_global("U8", u8_kind.into_value());

        let u16_kind = Gc::new(Kind::new_kind::<u16>(type_kind.clone(), "U16"));
        self.set_global("U16", u16_kind.into_value());

        let u32_kind = Gc::new(Kind::new_kind::<u32>(type_kind.clone(), "U32"));
        self.set_global("U32", u32_kind.into_value());

        let u64_kind = Gc::new(Kind::new_kind::<u64>(type_kind.clone(), "U64"));
        self.set_global("U64", u64_kind.into_value());

        let usize_kind = Gc::new(Kind::new_kind::<usize>(type_kind.clone(), "USize"));
        self.set_global("USize", usize_kind.into_value());

        // Signed
        let i8_kind = Gc::new(Kind::new_kind::<i8>(type_kind.clone(), "I8"));
        self.set_global("I8", i8_kind.into_value());

        let i16_kind = Gc::new(Kind::new_kind::<i16>(type_kind.clone(), "I16"));
        self.set_global("I16", i16_kind.into_value());

        let i32_kind = Gc::new(Kind::new_kind::<i32>(type_kind.clone(), "I32"));
        self.set_global("I32", i32_kind.into_value());

        let i64_kind = Gc::new(Kind::new_kind::<i64>(type_kind.clone(), "I64"));
        self.set_global("I64", i64_kind.into_value());

        let isize_kind = Gc::new(Kind::new_kind::<isize>(type_kind.clone(), "ISize"));
        self.set_global("ISize", isize_kind.into_value());

        // Float
        let f32_kind = Gc::new(Kind::new_kind::<f32>(type_kind.clone(), "f32"));
        self.set_global("F32", f32_kind.into_value());

        let f64_kind = Gc::new(Kind::new_kind::<f64>(type_kind.clone(), "f64"));
        self.set_global("F64", f64_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_list(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let list_kind = Gc::new(Kind::new_kind::<LinkedList<Gc<Value>>>(type_kind, "List"));
        self.set_global("List", list_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_vector(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let vector_kind = Gc::new(Kind::new_kind::<Vec<Gc<Value>>>(type_kind, "Vector"));
        self.set_global("Vector", vector_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_map(&mut self) -> &mut Self {
        let type_kind = self.get_global_type::<Kind>("Type").unwrap();

        let map_kind = Gc::new(Kind::new_kind::<HashMap<Gc<Value>, Gc<Value>>>(
            type_kind, "Map",
        ));
        self.set_global("Map", map_kind.into_value());

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut context = Context::new();
        println!("{:#?}", context);
        assert!(false);
    }
}
