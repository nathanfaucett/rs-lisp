use gc::Gc;

use super::{
    def_special_form, do_special_form, eval_special_form, expand_special_form, fn_special_form,
    if_special_form, macro_special_form, quote_special_form, read_special_form, Escape, Function,
    Keyword, Kind, List, Map, Object, Scope, SpecialForm, Stack, Symbol, Value, Vec,
};

#[inline]
pub fn new() -> Gc<Object<Scope>> {
    unsafe {
        let mut scope = Gc::null();

        init_root(&mut scope);
        init_nil(&mut scope);
        init_function(&mut scope);
        init_special_form(&mut scope);
        init_bool(&mut scope);
        init_character(&mut scope);
        init_string(&mut scope);
        init_symbol(&mut scope);
        init_keyword(&mut scope);
        init_escape(&mut scope);
        init_numbers(&mut scope);
        init_list(&mut scope);
        init_vec(&mut scope);
        init_map(&mut scope);
        init_functions(&mut scope);

        scope
    }
}

#[inline]
pub fn add_external_function<T, F>(scope: &mut Gc<Object<Scope>>, name: T, func: F)
where
    T: ToString,
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    let name_str = name.to_string();
    let name = new_symbol(&scope, name_str.clone());
    let params = new_list(&scope);

    let function = new_external_function(&scope, Some(name), params, func).into_value();
    scope.set(&name_str, function);
}

#[inline]
unsafe fn init_root(scope: &mut Gc<Object<Scope>>) {
    let type_kind = Kind::new_type_kind();
    let scope_kind = Gc::new(Kind::new_kind::<Scope>(type_kind.clone(), "Scope"));

    scope.set_from_value(Object::new(scope_kind.clone(), Scope::new(None)));

    scope.set("Type", type_kind.into_value());
    scope.set("Scope", scope_kind.into_value());
}

#[inline]
unsafe fn init_nil(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let nil_kind = Gc::new(Kind::new_kind::<()>(type_kind.clone(), "Nil"));
    let nil_value = Gc::new(Object::new(nil_kind.clone(), ()));

    scope.set("Nil", nil_kind.into_value());
    scope.set("nil", nil_value.into_value());
}

#[inline]
unsafe fn init_function(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let function_kind = Kind::new_kind::<Function>(type_kind.clone(), "Function");
    let macro_kind = Kind::new_kind::<Function>(type_kind.clone(), "Macro");

    scope.set("Function", Gc::new(function_kind).into_value());
    scope.set("Macro", Gc::new(macro_kind).into_value());
}

#[inline]
unsafe fn init_special_form(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let special_form_kind = Gc::new(Kind::new_kind::<SpecialForm>(
        type_kind.clone(),
        "SpecialForm",
    ));

    scope.set("SpecialForm", special_form_kind.into_value());

    let if_function = new_special_form(scope, if_special_form).into_value();
    scope.set("if", if_function);

    let fn_function = new_special_form(scope, fn_special_form).into_value();
    scope.set("fn", fn_function);

    let macro_function = new_special_form(scope, macro_special_form).into_value();
    scope.set("macro", macro_function);

    let def_function = new_special_form(scope, def_special_form).into_value();
    scope.set("def", def_function);

    let do_function = new_special_form(scope, do_special_form).into_value();
    scope.set("do", do_function);

    let quote_function = new_special_form(scope, quote_special_form).into_value();
    scope.set("quote", quote_function);

    let eval_function = new_special_form(scope, eval_special_form).into_value();
    scope.set("eval", eval_function);

    let read_function = new_special_form(scope, read_special_form).into_value();
    scope.set("read", read_function);

    let expand_function = new_special_form(scope, expand_special_form).into_value();
    scope.set("expand", expand_function);
}

#[inline]
unsafe fn init_bool(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let boolean_kind = Gc::new(Kind::new_kind::<bool>(type_kind, "Bool"));
    let true_value = Gc::new(Object::new(boolean_kind.clone(), true));
    let false_value = Gc::new(Object::new(boolean_kind.clone(), false));

    scope.set("Bool", boolean_kind.into_value());
    scope.set("true", true_value.into_value());
    scope.set("false", false_value.into_value());
}

#[inline]
unsafe fn init_character(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let character_kind = Gc::new(Kind::new_kind::<char>(type_kind, "Char"));
    scope.set("Char", character_kind.into_value());
}

#[inline]
unsafe fn init_string(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let string_kind = Gc::new(Kind::new_kind::<String>(type_kind, "String"));
    scope.set("String", string_kind.into_value());
}

#[inline]
unsafe fn init_symbol(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let symbol_kind = Gc::new(Kind::new_kind::<Symbol>(type_kind, "Symbol"));
    scope.set("Symbol", symbol_kind.into_value());
}

#[inline]
unsafe fn init_keyword(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let keyword_kind = Gc::new(Kind::new_kind::<Keyword>(type_kind, "Keyword"));
    scope.set("Keyword", keyword_kind.into_value());
}

#[inline]
unsafe fn init_escape(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let escape_kind = Gc::new(Kind::new_kind::<Keyword>(type_kind, "Escape"));
    scope.set("Escape", escape_kind.into_value());
}

#[inline]
unsafe fn init_numbers(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    // Unsigned
    let u8_kind = Gc::new(Kind::new_kind::<u8>(type_kind.clone(), "U8"));
    scope.set("U8", u8_kind.into_value());

    let u16_kind = Gc::new(Kind::new_kind::<u16>(type_kind.clone(), "U16"));
    scope.set("U16", u16_kind.into_value());

    let u32_kind = Gc::new(Kind::new_kind::<u32>(type_kind.clone(), "U32"));
    scope.set("U32", u32_kind.into_value());

    let u64_kind = Gc::new(Kind::new_kind::<u64>(type_kind.clone(), "U64"));
    scope.set("U64", u64_kind.into_value());

    let usize_kind = Gc::new(Kind::new_kind::<usize>(type_kind.clone(), "USize"));
    scope.set("USize", usize_kind.into_value());

    // Signed
    let i8_kind = Gc::new(Kind::new_kind::<i8>(type_kind.clone(), "I8"));
    scope.set("I8", i8_kind.into_value());

    let i16_kind = Gc::new(Kind::new_kind::<i16>(type_kind.clone(), "I16"));
    scope.set("I16", i16_kind.into_value());

    let i32_kind = Gc::new(Kind::new_kind::<i32>(type_kind.clone(), "I32"));
    scope.set("I32", i32_kind.into_value());

    let i64_kind = Gc::new(Kind::new_kind::<i64>(type_kind.clone(), "I64"));
    scope.set("I64", i64_kind.into_value());

    let isize_kind = Gc::new(Kind::new_kind::<isize>(type_kind.clone(), "ISize"));
    scope.set("ISize", isize_kind.into_value());

    // Float
    let f32_kind = Gc::new(Kind::new_kind::<f32>(type_kind.clone(), "f32"));
    scope.set("F32", f32_kind.into_value());

    let f64_kind = Gc::new(Kind::new_kind::<f64>(type_kind.clone(), "f64"));
    scope.set("F64", f64_kind.into_value());
}

#[inline]
unsafe fn init_list(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let mut list_kind = Gc::new(Kind::new_kind::<List>(type_kind, "List"));
    scope.set("List", list_kind.clone().into_value());

    List::init(scope, &mut list_kind);
}

#[inline]
unsafe fn init_vec(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let mut vec_kind = Gc::new(Kind::new_kind::<Vec>(type_kind, "Vec"));
    scope.set("Vec", vec_kind.clone().into_value());

    Vec::init(scope, &mut vec_kind);
}

#[inline]
unsafe fn init_map(scope: &mut Gc<Object<Scope>>) {
    let type_kind = scope.get_with_type::<Kind>("Type").unwrap();

    let map_kind = Gc::new(Kind::new_kind::<Map>(type_kind, "Map"));
    scope.set("Map", map_kind.into_value());
}

#[inline]
unsafe fn init_functions(scope: &mut Gc<Object<Scope>>) {
    add_external_function(scope, "kind-get", kind_get);
}

#[inline]
fn kind_get(scope: Gc<Object<Scope>>, mut args: Gc<Object<List>>) -> Gc<Value> {
    let key = args
        .pop_front()
        .expect("Invalid Argument provided for key")
        .downcast::<Object<Keyword>>()
        .expect("Invalid Argument provided for key");
    let kind = args
        .pop_front()
        .expect("Invalid Argument provided for kind")
        .downcast::<Object<Kind>>()
        .expect("Invalid Argument provided for kind");

    kind.get(&key.into_value())
        .map(Clone::clone)
        .unwrap_or_else(|| nil_value(&scope).into_value())
}

#[inline]
pub fn usize_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("USize")
            .expect("failed to get USize Kind")
    }
}
#[inline]
pub fn new_usize(scope: &Gc<Object<Scope>>, value: usize) -> Gc<Object<usize>> {
    unsafe { Gc::new(Object::new(usize_kind(scope), value)) }
}

#[inline]
pub fn i8_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("I8")
            .expect("failed to get I8 Kind")
    }
}
#[inline]
pub fn new_i8(scope: &Gc<Object<Scope>>, value: i8) -> Gc<Object<i8>> {
    unsafe { Gc::new(Object::new(i8_kind(scope), value)) }
}

#[inline]
pub fn i16_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("I6")
            .expect("failed to get I16 Kind")
    }
}
#[inline]
pub fn new_i16(scope: &Gc<Object<Scope>>, value: i16) -> Gc<Object<i16>> {
    unsafe { Gc::new(Object::new(i16_kind(scope), value)) }
}

#[inline]
pub fn i32_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("I32")
            .expect("failed to get I32 Kind")
    }
}
#[inline]
pub fn new_i32(scope: &Gc<Object<Scope>>, value: i32) -> Gc<Object<i32>> {
    unsafe { Gc::new(Object::new(i32_kind(scope), value)) }
}

#[inline]
pub fn i64_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("I64")
            .expect("failed to get I64 Kind")
    }
}
#[inline]
pub fn new_i64(scope: &Gc<Object<Scope>>, value: i64) -> Gc<Object<i64>> {
    unsafe { Gc::new(Object::new(i64_kind(scope), value)) }
}

#[inline]
pub fn isize_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("ISize")
            .expect("failed to get ISize Kind")
    }
}
#[inline]
pub fn new_isize(scope: &Gc<Object<Scope>>, value: isize) -> Gc<Object<isize>> {
    unsafe { Gc::new(Object::new(isize_kind(scope), value)) }
}

#[inline]
pub fn u8_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("U8")
            .expect("failed to get U8 Kind")
    }
}
#[inline]
pub fn new_u8(scope: &Gc<Object<Scope>>, value: u8) -> Gc<Object<u8>> {
    unsafe { Gc::new(Object::new(u8_kind(scope), value)) }
}

#[inline]
pub fn u16_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("U6")
            .expect("failed to get U16 Kind")
    }
}
#[inline]
pub fn new_u16(scope: &Gc<Object<Scope>>, value: u16) -> Gc<Object<u16>> {
    unsafe { Gc::new(Object::new(u16_kind(scope), value)) }
}

#[inline]
pub fn u32_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("U32")
            .expect("failed to get U32 Kind")
    }
}
#[inline]
pub fn new_u32(scope: &Gc<Object<Scope>>, value: u32) -> Gc<Object<u32>> {
    unsafe { Gc::new(Object::new(u32_kind(scope), value)) }
}

#[inline]
pub fn u64_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("U64")
            .expect("failed to get U64 Kind")
    }
}
#[inline]
pub fn new_u64(scope: &Gc<Object<Scope>>, value: u64) -> Gc<Object<u64>> {
    unsafe { Gc::new(Object::new(u64_kind(scope), value)) }
}

// #[inline]
// pub fn f32_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
//     unsafe {
//         scope
//             .get_with_type::<Kind>("F32")
//             .expect("failed to get F32 Kind")
//     }
// }
// #[inline]
// pub fn new_f32(scope: &Gc<Object<Scope>>, value: f32) -> Gc<Object<f32>> {
//     unsafe { Gc::new(Object::new(f32_kind(scope), value)) }
// }

// #[inline]
// pub fn f64_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
//     unsafe {
//         scope
//             .get_with_type::<Kind>("F64")
//             .expect("failed to get F64 Kind")
//     }
// }
// #[inline]
// pub fn new_f64(scope: &Gc<Object<Scope>>, value: f64) -> Gc<Object<f64>> {
//     unsafe { Gc::new(Object::new(f64_kind(scope), value)) }
// }

// #[inline]
// pub fn new_nan_f32(scope: &Gc<Object<Scope>>) -> Gc<Object<f32>> {
//     unsafe { Gc::new(Object::new(f64_kind(scope), ::std::f32::NAN)) }
// }

// #[inline]
// pub fn new_nan_f64(scope: &Gc<Object<Scope>>) -> Gc<Object<f64>> {
//     unsafe { Gc::new(Object::new(f64_kind(scope), ::std::f64::NAN)) }
// }

#[inline]
pub fn bool_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Bool")
            .expect("failed to get Bool Kind")
    }
}
#[inline]
pub fn true_value(scope: &Gc<Object<Scope>>) -> Gc<Object<bool>> {
    unsafe {
        scope
            .get_with_type::<bool>("true")
            .expect("failed to get true value")
    }
}
#[inline]
pub fn false_value(scope: &Gc<Object<Scope>>) -> Gc<Object<bool>> {
    unsafe {
        scope
            .get_with_type::<bool>("false")
            .expect("failed to get false value")
    }
}
#[inline]
pub fn new_bool(scope: &Gc<Object<Scope>>, value: bool) -> Gc<Object<bool>> {
    if value {
        true_value(scope)
    } else {
        false_value(scope)
    }
}

#[inline]
pub fn nil_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Nil")
            .expect("failed to get Nil Kind")
    }
}
#[inline]
pub fn nil_value(scope: &Gc<Object<Scope>>) -> Gc<Object<()>> {
    unsafe {
        scope
            .get_with_type::<()>("nil")
            .expect("failed to get nil value")
    }
}

#[inline]
pub fn char_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Char")
            .expect("failed to get Char Kind")
    }
}
#[inline]
pub fn new_char(scope: &Gc<Object<Scope>>, value: char) -> Gc<Object<char>> {
    unsafe { Gc::new(Object::new(char_kind(scope), value)) }
}

#[inline]
pub fn string_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("String")
            .expect("failed to get String Kind")
    }
}
#[inline]
pub fn new_string<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<String>>
where
    T: ToString,
{
    unsafe { Gc::new(Object::new(string_kind(scope), value.to_string())) }
}

#[inline]
pub fn keyword_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Keyword")
            .expect("failed to get Keyword Kind")
    }
}
#[inline]
pub fn new_keyword<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<Keyword>>
where
    T: ToString,
{
    unsafe {
        Gc::new(Object::new(
            keyword_kind(scope),
            Keyword::new(value.to_string()),
        ))
    }
}

#[inline]
pub fn escape_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Escape")
            .expect("failed to get Escape Kind")
    }
}
#[inline]
pub fn new_escape(scope: &Gc<Object<Scope>>, value: Gc<Value>) -> Gc<Object<Escape>> {
    unsafe { Gc::new(Object::new(escape_kind(scope), Escape::new(value))) }
}

#[inline]
pub fn symbol_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Symbol")
            .expect("failed to get Symbol Kind")
    }
}
#[inline]
pub fn new_symbol<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Object<Symbol>>
where
    T: ToString,
{
    unsafe {
        Gc::new(Object::new(
            symbol_kind(scope),
            Symbol::new(value.to_string()),
        ))
    }
}

#[inline]
pub fn list_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("List")
            .expect("failed to get List Kind")
    }
}
#[inline]
pub fn new_list(scope: &Gc<Object<Scope>>) -> Gc<Object<List>> {
    unsafe { Gc::new(Object::new(list_kind(scope), List::new())) }
}

#[inline]
pub fn vec_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Vec")
            .expect("failed to get Vec Kind")
    }
}
#[inline]
pub fn new_vec(scope: &Gc<Object<Scope>>) -> Gc<Object<Vec>> {
    unsafe { Gc::new(Object::new(vec_kind(scope), Vec::new())) }
}

#[inline]
pub fn map_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Map")
            .expect("failed to get Map Kind")
    }
}
#[inline]
pub fn new_map(scope: &Gc<Object<Scope>>) -> Gc<Object<Map>> {
    unsafe { Gc::new(Object::new(map_kind(scope), Map::new())) }
}

#[inline]
pub fn scope_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Scope")
            .expect("failed to get Scope Kind")
    }
}
#[inline]
pub fn new_scope(scope: &Gc<Object<Scope>>) -> Gc<Object<Scope>> {
    unsafe {
        Gc::new(Object::new(
            scope_kind(scope),
            Scope::new(Some(scope.clone())),
        ))
    }
}

#[inline]
pub fn special_form_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("SpecialForm")
            .expect("failed to get SpecialForm Kind")
    }
}
#[inline]
pub fn new_special_form<F>(scope: &Gc<Object<Scope>>, f: F) -> Gc<Object<SpecialForm>>
where
    F: 'static + Fn(&mut Stack),
{
    unsafe { Gc::new(Object::new(special_form_kind(scope), SpecialForm::new(f))) }
}

#[inline]
pub fn function_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Function")
            .expect("failed to get Function Kind")
    }
}
#[inline]
pub fn new_function(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<Symbol>>>,
    params: Gc<Object<List>>,
    body: Gc<Value>,
) -> Gc<Object<Function>> {
    unsafe {
        Gc::new(Object::new(
            function_kind(scope),
            Function::new(name, scope.clone(), params, body),
        ))
    }
}
#[inline]
pub fn new_external_function<F>(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<Symbol>>>,
    params: Gc<Object<List>>,
    body: F,
) -> Gc<Object<Function>>
where
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    unsafe {
        Gc::new(Object::new(
            function_kind(scope),
            Function::new_external(name, scope.clone(), params, body),
        ))
    }
}

#[inline]
pub fn macro_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    unsafe {
        scope
            .get_with_type::<Kind>("Macro")
            .expect("failed to get Macro Kind")
    }
}
#[inline]
pub fn new_macro(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<Symbol>>>,
    params: Gc<Object<List>>,
    body: Gc<Value>,
) -> Gc<Object<Function>> {
    unsafe {
        Gc::new(Object::new(
            macro_kind(scope),
            Function::new(name, scope.clone(), params, body),
        ))
    }
}
#[inline]
pub fn new_external_macro<F>(
    scope: &Gc<Object<Scope>>,
    name: Option<Gc<Object<Symbol>>>,
    params: Gc<Object<List>>,
    body: F,
) -> Gc<Object<Function>>
where
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    unsafe {
        Gc::new(Object::new(
            macro_kind(scope),
            Function::new_external(name, scope.clone(), params, body),
        ))
    }
}

#[inline]
pub fn add_kind_method<N, F>(
    scope: &Gc<Object<Scope>>,
    kind: &mut Gc<Object<Kind>>,
    name: N,
    func: F,
) where
    N: ToString,
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<Value>,
{
    let string = name.to_string();
    let key = new_keyword(scope, string.clone());
    let name = new_symbol(scope, string);
    let params = new_list(scope);

    let value = new_external_function(scope, Some(name), params, func);
    kind.set(key.into_value(), value.into_value());
}
