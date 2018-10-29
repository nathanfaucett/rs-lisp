use gc::Gc;

use super::{
    def_special_form, do_special_form, fn_special_form, if_special_form, macro_special_form,
    quote_special_form, read_internal, unquote_special_form, Function, Kind, List, Map, Object,
    Scope, SpecialForm, Symbol, Value, Vector,
};

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
            .init_special_form()
            .init_boolean()
            .init_character()
            .init_string()
            .init_symbol()
            .init_numbers()
            .init_list()
            .init_vector()
            .init_map()
            .init_read()
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
            .set_from_value(Object::new(scope_kind.clone(), Scope::new(None)));

        self.scope.set("Type", type_kind.into_value());
        self.scope.set("Scope", scope_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_nil(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let nil_kind = Gc::new(Kind::new_kind::<()>(type_kind.clone(), "Nil"));
        let nil_value = Gc::new(Object::new(nil_kind.clone(), ()));

        self.scope.set("Nil", nil_kind.into_value());
        self.scope.set("nil", nil_value.into_value());

        self
    }

    #[inline]
    unsafe fn init_function(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let function_kind = Kind::new_kind::<Function>(type_kind.clone(), "Function");
        let macro_kind = Kind::new_kind::<Function>(type_kind.clone(), "Macro");

        self.scope
            .set("Function", Gc::new(function_kind).into_value());
        self.scope.set("Macro", Gc::new(macro_kind).into_value());

        self
    }

    #[inline]
    unsafe fn init_special_form(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let special_form_kind = Gc::new(Kind::new_kind::<SpecialForm>(
            type_kind.clone(),
            "SpecialForm",
        ));

        self.scope.set(
            "if",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(if_special_form),
            ))
            .into_value(),
        );
        self.scope.set(
            "fn",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(fn_special_form),
            ))
            .into_value(),
        );
        self.scope.set(
            "macro",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(macro_special_form),
            ))
            .into_value(),
        );
        self.scope.set(
            "def",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(def_special_form),
            ))
            .into_value(),
        );
        self.scope.set(
            "do",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(do_special_form),
            ))
            .into_value(),
        );
        self.scope.set(
            "quote",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(quote_special_form),
            ))
            .into_value(),
        );
        self.scope.set(
            "unquote",
            Gc::new(Object::new(
                special_form_kind.clone(),
                SpecialForm::new(unquote_special_form),
            ))
            .into_value(),
        );

        self.scope
            .set("SpecialForm", special_form_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_boolean(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

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
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let character_kind = Gc::new(Kind::new_kind::<char>(type_kind, "Character"));
        self.scope.set("Character", character_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_string(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let string_kind = Gc::new(Kind::new_kind::<String>(type_kind, "String"));
        self.scope.set("String", string_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_symbol(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let symbol_kind = Gc::new(Kind::new_kind::<Symbol>(type_kind, "Symbol"));
        self.scope.set("Symbol", symbol_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_numbers(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

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
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let mut list_kind = Gc::new(Kind::new_kind::<List>(type_kind, "List"));
        self.scope.set("List", list_kind.clone().into_value());

        List::init(&mut self.scope, &mut list_kind);

        self
    }

    #[inline]
    unsafe fn init_vector(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let vector_kind = Gc::new(Kind::new_kind::<Vector>(type_kind, "Vector"));
        self.scope.set("Vector", vector_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_map(&mut self) -> &mut Self {
        let type_kind = self.scope.get_with_type::<Kind>("Type").unwrap();

        let map_kind = Gc::new(Kind::new_kind::<Map>(type_kind, "Map"));
        self.scope.set("Map", map_kind.into_value());

        self
    }

    #[inline]
    unsafe fn init_read(&mut self) -> &mut Self {
        let read_fn = new_rust_function(&self.scope, "read", read_internal).into_value();
        self.scope.set("read", read_fn);
        self
    }
}

#[inline]
pub fn new_usize(scope: &Gc<Object<Scope>>, value: usize) -> Gc<Object<usize>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("USize")
            .expect("failed to find USize type");
        Gc::new(Object::new(kind, value))
    }
}

#[inline]
pub fn new_isize(scope: &Gc<Object<Scope>>, value: isize) -> Gc<Object<isize>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("ISize")
            .expect("failed to find ISize type");
        Gc::new(Object::new(kind, value))
    }
}

#[inline]
pub fn new_true(scope: &Gc<Object<Scope>>) -> Gc<Object<bool>> {
    unsafe {
        scope
            .get_with_type::<bool>("true")
            .expect("failed to find true value")
    }
}
#[inline]
pub fn new_false(scope: &Gc<Object<Scope>>) -> Gc<Object<bool>> {
    unsafe {
        scope
            .get_with_type::<bool>("false")
            .expect("failed to find false value")
    }
}
#[inline]
pub fn new_boolean(scope: &Gc<Object<Scope>>, value: bool) -> Gc<Object<bool>> {
    if value {
        new_true(scope)
    } else {
        new_false(scope)
    }
}
#[inline]
pub fn new_nil(scope: &Gc<Object<Scope>>) -> Gc<Object<()>> {
    unsafe {
        scope
            .get_with_type::<()>("nil")
            .expect("failed to find nil value")
    }
}

#[inline]
pub fn new_char(scope: &Gc<Object<Scope>>, value: char) -> Gc<Object<char>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Character")
            .expect("failed to find Character type");
        Gc::new(Object::new(kind, value))
    }
}

#[inline]
pub fn new_string<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<String>>
where
    T: ToString,
{
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("String")
            .expect("failed to find String type");
        Gc::new(Object::new(kind, value.to_string()))
    }
}

#[inline]
pub fn new_symbol<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<Symbol>>
where
    T: ToString,
{
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Symbol")
            .expect("failed to find Symbol type");
        Gc::new(Object::new(kind, Symbol::new(value.to_string())))
    }
}

#[inline]
pub fn new_list(scope: &Gc<Object<Scope>>) -> Gc<Object<List>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("List")
            .expect("failed to find List type");
        Gc::new(Object::new(kind, List::new()))
    }
}
#[inline]
pub fn new_vector(scope: &Gc<Object<Scope>>) -> Gc<Object<Vector>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Vector")
            .expect("failed to find Vector type");
        Gc::new(Object::new(kind, Vector::new()))
    }
}

#[inline]
pub fn new_function(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<String>>>,
    function_scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: Gc<Value>,
) -> Gc<Object<Function>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Function")
            .expect("failed to find Function type");
        Gc::new(Object::new(
            kind,
            Function::new(name, function_scope, params, body),
        ))
    }
}

#[inline]
pub fn new_macro(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<String>>>,
    function_scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: Gc<Value>,
) -> Gc<Object<Function>> {
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Macro")
            .expect("failed to find Function type");
        Gc::new(Object::new(
            kind,
            Function::new(name, function_scope, params, body),
        ))
    }
}

#[inline]
pub fn new_external_function<F>(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<String>>>,
    function_scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: F,
) -> Gc<Object<Function>>
where
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Function")
            .expect("failed to find Function type");
        Gc::new(Object::new(
            kind,
            Function::new_external(name, function_scope, params, body),
        ))
    }
}

#[inline]
pub fn new_external_macro<F>(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<String>>>,
    function_scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: F,
) -> Gc<Object<Function>>
where
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    unsafe {
        let kind = scope
            .get_with_type::<Kind>("Macro")
            .expect("failed to find Macro type");
        Gc::new(Object::new(
            kind,
            Function::new_external(name, function_scope, params, body),
        ))
    }
}

#[inline]
pub fn new_rust_function<T, F>(scope: &Gc<Object<Scope>>, name: T, func: F) -> Gc<Object<Function>>
where
    T: ToString,
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    let string = name.to_string();
    let name = new_string(scope, string.to_string());
    let mut params = new_list(scope);
    params.push_back(new_symbol(scope, "list").into_value());
    new_external_function(scope, Some(name.clone()), scope.clone(), params, func)
}

#[inline]
pub fn kind_add_function<T, F>(
    scope: &Gc<Object<Scope>>,
    kind: &mut Gc<Object<Kind>>,
    name: T,
    func: F,
) where
    T: ToString,
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    let string = name.to_string();
    let name = new_string(scope, string.to_string());
    let mut params = new_list(scope);
    params.push_back(new_symbol(scope, "list").into_value());
    let value = new_external_function(scope, Some(name.clone()), scope.clone(), params, func);
    kind.set(name.into_value(), value.into_value());
}
