use alloc::boxed::Box;
use alloc::string::String;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::{fmt, ptr};

use gc::{Gc, Trace};

use super::{
    escape_kind, new_function, new_kind, new_list, new_list_from, new_macro, new_object, nil_value,
    read_value, scope_get_with_kind, scope_set, Escape, EvalState, Kind, List, Object, Reader,
    Scope, Stack, Symbol, Value, Vector,
};

pub struct SpecialForm(Box<dyn Fn(&mut Stack)>);

impl Trace for SpecialForm {
    #[inline]
    fn trace(&mut self, _marked: bool) {}
}

impl fmt::Debug for SpecialForm {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(":special-form")
    }
}

impl PartialEq for SpecialForm {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ::core::ptr::eq(self, other)
    }
}

impl PartialOrd for SpecialForm {
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Deref for SpecialForm {
    type Target = dyn Fn(&mut Stack);

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for SpecialForm {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl Eq for SpecialForm {}

impl Hash for SpecialForm {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state);
    }
}

impl SpecialForm {
    #[inline(always)]
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(&mut Stack),
    {
        SpecialForm(Box::new(f))
    }

    #[inline]
    pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
        let special_form_kind = new_kind::<SpecialForm>(scope, "SpecialForm");
        scope_set(scope, "SpecialForm", special_form_kind.into_value());
    }

    #[inline]
    pub(crate) fn init_scope(scope: &Gc<Object<Scope>>) {
        let if_function = new_special_form(scope, if_special_form).into_value();
        scope_set(scope, "if", if_function);

        let fn_function = new_special_form(scope, fn_special_form).into_value();
        scope_set(scope, "fn", fn_function);

        let macro_function = new_special_form(scope, macro_special_form).into_value();
        scope_set(scope, "macro", macro_function);

        let def_function = new_special_form(scope, def_special_form).into_value();
        scope_set(scope, "def", def_function);

        let do_function = new_special_form(scope, do_special_form).into_value();
        scope_set(scope, "do", do_function);

        let quote_function = new_special_form(scope, quote_special_form).into_value();
        scope_set(scope, "quote", quote_function);

        let eval_function = new_special_form(scope, eval_special_form).into_value();
        scope_set(scope, "eval", eval_function);

        let read_function = new_special_form(scope, read_special_form).into_value();
        scope_set(scope, "read", read_function);

        let expand_function = new_special_form(scope, expand_special_form).into_value();
        scope_set(scope, "expand", expand_function);

        let throw_function = new_special_form(scope, throw_special_form).into_value();
        scope_set(scope, "throw", throw_function);

        let try_function = new_special_form(scope, try_special_form).into_value();
        scope_set(scope, "try", try_function);
    }
}

impl<'a> FnOnce<(&'a mut Stack,)> for SpecialForm {
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, (stack,): (&mut Stack,)) -> Self::Output {
        (self.0)(stack)
    }
}

impl Fn<(&mut Stack,)> for SpecialForm {
    #[inline(always)]
    extern "rust-call" fn call(&self, (stack,): (&mut Stack,)) -> Self::Output {
        self.deref()(stack)
    }
}

impl FnMut<(&mut Stack,)> for SpecialForm {
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, (stack,): (&mut Stack,)) -> Self::Output {
        self.deref_mut()(stack)
    }
}

#[inline]
pub fn if_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for if");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed downcast arguments to Vector for if");

    let expr = args.get(0).expect("failed to get expr");
    let if_expr = args.get(1).expect("failed to get if expr");

    stack.state.push_front(EvalState::If);

    if let Some(value) = args.get(2) {
        stack.value.push_front(value.clone());
    } else {
        stack
            .value
            .push_front(nil_value(stack.scope.front().unwrap()).clone().into_value());
    }
    stack.value.push_front(if_expr.clone());

    stack.value.push_front(expr.clone());
    stack.state.push_front(EvalState::Eval);
}

#[inline]
pub fn def_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for def");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed downcast arguments to Vector for def");

    let key = args.get(0).expect("failed to get key for def");
    let value = args.get(1).expect("failed to get value for def");

    // returns nil
    stack
        .value
        .push_front(nil_value(stack.scope.front().unwrap()).clone().into_value());

    stack.value.push_front(key.clone());
    stack.value.push_front(value.clone());

    stack.state.push_front(EvalState::Def);
    stack.state.push_front(EvalState::Eval);
}

#[inline]
fn build_function(
    stack: &mut Stack,
) -> (
    Option<Gc<Object<Symbol>>>,
    Gc<Object<Vector>>,
    Gc<dyn Value>,
) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for function");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed downcast arguments to Vector for function");

    let (name, params) = {
        let first = args
            .get(0)
            .expect("failed to get function name/params for fn");

        match first.downcast_ref::<Object<Symbol>>() {
            Some(name) => {
                let params = args
                    .get(1)
                    .expect("failed to get function params")
                    .downcast_ref::<Object<Vector>>()
                    .expect("failed to downcast function params as Vector")
                    .clone();
                (Some(name.clone()), params)
            }
            None => match first.downcast_ref::<Object<Vector>>() {
                Some(params) => (None, params.clone()),
                None => panic!("invalid function params provided to fn {:?}", args),
            },
        }
    };
    let body = args
        .get(if name.is_some() { 2 } else { 1 })
        .expect("failed to get function body")
        .clone();

    (name, params, body)
}

#[inline]
pub fn fn_special_form(stack: &mut Stack) {
    let (name, params, body) = build_function(stack);

    stack
        .value
        .push_front(new_function(stack.scope.front().unwrap(), name, params, body).into_value());
}

#[inline]
pub fn macro_special_form(stack: &mut Stack) {
    let (name, params, body) = build_function(stack);

    stack
        .value
        .push_front(new_macro(stack.scope.front().unwrap(), name, params, body).into_value());
}

#[inline]
pub fn do_special_form(stack: &mut Stack) {
    let args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for do");
    let args = args_value
        .downcast_ref::<Object<Vector>>()
        .expect("failed to downcast do arguments to Vector");

    let mut first = false;

    for value in args.iter().rev() {
        if !first {
            first = true;
        } else {
            stack.state.push_front(EvalState::PopValue);
        }
        stack.state.push_front(EvalState::Eval);

        stack.value.push_front(value.clone());
    }
}

#[inline]
pub fn quote_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast quote arguments to Vector");

    if let Some(value) = args.get(0) {
        stack.value.push_front(value.clone());
    }
}

#[inline]
pub fn eval_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for eval");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast eval arguments to Vector");

    if let Some(value) = args.get(0) {
        stack.value.push_front(value.clone());
        stack.state.push_front(EvalState::Eval);
    }
}

#[inline]
pub fn read_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast quote arguments to Vector");

    if let Some(value) = args.get(0) {
        let string = value
            .downcast_ref::<Object<String>>()
            .expect("failed to downcast read argument to String");
        let char_list = string.chars().collect::<::alloc::vec::Vec<char>>();
        let mut reader = Reader::new(None, char_list);
        let value = read_value(
            stack.scope.front().expect("failed to get scope"),
            &mut reader,
        );

        stack.value.push_front(value.clone());
    } else {
        stack
            .value
            .push_front(nil_value(stack.scope.front().unwrap()).clone().into_value());
    }
}

#[inline]
pub fn expand_special_form(stack: &mut Stack) {
    let scope = stack.scope.front().expect("failed to get scope");
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for expand");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast expand arguments to Vector");
    let mut list = args.iter().collect::<List>();

    if let Some(value) = list.pop_front() {
        stack.state.push_front(EvalState::Expand);

        stack
            .value
            .push_front(new_list_from(scope, list).into_value());
        stack.value.push_front(new_list(scope).into_value());

        if value.kind() == escape_kind(scope) {
            let escape = value
                .downcast_ref::<Object<Escape>>()
                .expect("failed to downcast expand value to Escape");

            stack.state.push_front(EvalState::Eval);
            stack.value.push_front(escape.escape_value().clone());
        } else {
            stack.value.push_front(value.clone());
        }
    } else {
        stack
            .value
            .push_front(new_list_from(scope, list).into_value());
    }
}

#[inline]
pub fn throw_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast quote arguments to Vector");
    let value = args.get(0).map(Clone::clone).unwrap_or_else(|| {
        nil_value(stack.scope.front().expect("failed to get scope"))
            .clone()
            .into_value()
    });

    stack.state.push_front(EvalState::Throw);
    stack.state.push_front(EvalState::Eval);
    stack.value.push_front(value);
}

#[inline]
pub fn try_special_form(stack: &mut Stack) {
    let mut args_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote");
    let args = args_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast quote arguments to Vector");
    let block = args.get(0).map(Clone::clone).unwrap_or_else(|| {
        nil_value(stack.scope.front().expect("failed to get scope"))
            .clone()
            .into_value()
    });
    let handler = args.get(1).map(Clone::clone).unwrap_or_else(|| {
        nil_value(stack.scope.front().expect("failed to get scope"))
            .clone()
            .into_value()
    });

    stack.state.push_front(EvalState::Catch);
    stack.value.push_front(handler);
    stack.state.push_front(EvalState::Eval);
    stack.value.push_front(block);
}

#[inline]
pub fn special_form_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "SpecialForm").expect("failed to get SpecialForm Kind")
}

#[inline]
pub fn new_special_form<F>(scope: &Gc<Object<Scope>>, f: F) -> Gc<Object<SpecialForm>>
where
    F: 'static + Fn(&mut Stack),
{
    new_object(
        scope,
        Object::new(special_form_kind(scope).clone(), SpecialForm::new(f)),
    )
}
