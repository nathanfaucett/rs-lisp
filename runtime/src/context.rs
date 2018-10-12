use std::collections::{HashMap, LinkedList};

use gc::Gc;

use super::{Function, Kind, Object, Scope, Value};

pub type List = LinkedList<Gc<Value>>;
pub type Vector = Vec<Gc<Value>>;
pub type Map = HashMap<Gc<Value>, Gc<Value>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Context {
    scope: Gc<Object<Scope>>,
}

impl Context {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut context = Context { scope: Gc::null() };
            context.init();
            context
        }
    }

    #[inline]
    pub unsafe fn init(&mut self) -> &mut Self {
        self.init_root()
            .init_nil()
            .init_function()
            .init_boolean()
            .init_character()
            .init_keyword()
            .init_string()
            .init_symbol()
            .init_numbers()
            .init_list()
            .init_vector()
            .init_map()
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
    unsafe fn init_root(&mut self) -> &mut Self {
        let type_kind = Kind::new_type_kind();
        let scope_kind = Gc::new(Kind::new_kind::<Scope>(type_kind.clone(), "Scope"));

        self.scope
            .set_from_value(Object::new(scope_kind.clone(), Scope::new_root()));

        self.scope.set("Type", type_kind.into_value());
        self.scope.set("Scope", scope_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_nil(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let nil_kind = Gc::new(Kind::new_kind::<()>(type_kind.clone(), "Nil"));
        let nil_value = Gc::new(Object::new(nil_kind.clone(), ()));

        self.scope.set("Nil", nil_kind.into_value());
        self.scope.set("nil", nil_value.into_value());

        self
    }

    #[inline]
    unsafe fn init_function(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let function_kind = Kind::new_kind::<Function>(type_kind.clone(), "Function");
        let macro_kind = Kind::new_kind::<Function>(type_kind.clone(), "Macro");

        self.scope
            .set("Function", Gc::new(function_kind).into_value());
        self.scope.set("Macro", Gc::new(macro_kind).into_value());

        self
    }

    #[inline]
    unsafe fn init_boolean(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let boolean_kind = Gc::new(Kind::new_kind::<bool>(type_kind, "Boolean"));
        let true_value = Gc::new(Object::new(boolean_kind.clone(), true));
        let false_value = Gc::new(Object::new(boolean_kind.clone(), false));

        self.scope.set("Boolean", boolean_kind.into_value());
        self.scope.set("true", true_value.into_value());
        self.scope.set("false", false_value.into_value());

        self
    }

    #[inline]
    unsafe fn init_character(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let character_kind = Gc::new(Kind::new_kind::<char>(type_kind, "Character"));
        self.scope.set("Character", character_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_keyword(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let keyword_kind = Gc::new(Kind::new_kind::<String>(type_kind, "Keyword"));
        self.scope.set("Keyword", keyword_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_string(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let string_kind = Gc::new(Kind::new_kind::<String>(type_kind, "String"));
        self.scope.set("String", string_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_symbol(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let symbol_kind = Gc::new(Kind::new_kind::<String>(type_kind, "Symbol"));
        self.scope.set("Symbol", symbol_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_numbers(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        // Unsigned
        let u8_kind = Gc::new(Kind::new_kind::<u8>(type_kind.clone(), "U8"));
        self.scope.set("U8", u8_kind.into_value());

        let u16_kind = Gc::new(Kind::new_kind::<u16>(type_kind.clone(), "U16"));
        self.scope.set("U16", u16_kind.into_value());

        let u32_kind = Gc::new(Kind::new_kind::<u32>(type_kind.clone(), "U32"));
        self.scope.set("U32", u32_kind.into_value());

        let u64_kind = Gc::new(Kind::new_kind::<u64>(type_kind.clone(), "U64"));
        self.scope.set("U64", u64_kind.into_value());

        let usize_kind = Gc::new(Kind::new_kind::<usize>(type_kind.clone(), "USize"));
        self.scope.set("USize", usize_kind.into_value());

        // Signed
        let i8_kind = Gc::new(Kind::new_kind::<i8>(type_kind.clone(), "I8"));
        self.scope.set("I8", i8_kind.into_value());

        let i16_kind = Gc::new(Kind::new_kind::<i16>(type_kind.clone(), "I16"));
        self.scope.set("I16", i16_kind.into_value());

        let i32_kind = Gc::new(Kind::new_kind::<i32>(type_kind.clone(), "I32"));
        self.scope.set("I32", i32_kind.into_value());

        let i64_kind = Gc::new(Kind::new_kind::<i64>(type_kind.clone(), "I64"));
        self.scope.set("I64", i64_kind.into_value());

        let isize_kind = Gc::new(Kind::new_kind::<isize>(type_kind.clone(), "ISize"));
        self.scope.set("ISize", isize_kind.into_value());

        // Float
        let f32_kind = Gc::new(Kind::new_kind::<f32>(type_kind.clone(), "f32"));
        self.scope.set("F32", f32_kind.into_value());

        let f64_kind = Gc::new(Kind::new_kind::<f64>(type_kind.clone(), "f64"));
        self.scope.set("F64", f64_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_list(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let list_kind = Gc::new(Kind::new_kind::<List>(type_kind, "List"));
        self.scope.set("List", list_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_vector(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let vector_kind = Gc::new(Kind::new_kind::<Vector>(type_kind, "Vector"));
        self.scope.set("Vector", vector_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_map(&mut self) -> &mut Self {
        let type_kind = self.scope.get_type::<Kind>("Type").unwrap();

        let map_kind = Gc::new(Kind::new_kind::<Map>(type_kind, "Map"));
        self.scope.set("Map", map_kind.into_value());

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let context = Context::new();
        println!("{:#?}", context);
        assert!(false);
    }
}
