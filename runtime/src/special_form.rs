use std::fmt;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{
    new_function, new_macro, nil_value, read_value, EvalState, List, Object, Reader, Stack, Symbol,
    Value,
};

pub struct SpecialForm(Box<Fn(&mut Stack)>);

impl fmt::Debug for SpecialForm {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SpecialForm")
    }
}

impl PartialEq for SpecialForm {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self, other)
    }
}

impl Eq for SpecialForm {}

impl Hash for SpecialForm {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self as *const Self).hash(state)
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

    #[inline(always)]
    pub fn inner(&self) -> &Fn(&mut Stack) {
        &*self.0
    }
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut Fn(&mut Stack) {
        &mut *self.0
    }
}

impl<'a> FnOnce<(&'a mut Stack)> for SpecialForm {
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, stack: (&mut Stack)) -> Self::Output {
        (self.0)(stack)
    }
}

impl<'a> Fn<(&'a mut Stack)> for SpecialForm {
    #[inline(always)]
    extern "rust-call" fn call(&self, stack: (&mut Stack)) -> Self::Output {
        self.inner()(stack)
    }
}

impl<'a> FnMut<(&'a mut Stack)> for SpecialForm {
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, stack: (&mut Stack)) -> Self::Output {
        self.inner_mut()(stack)
    }
}

#[inline]
pub fn if_special_form(stack: &mut Stack) {
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for if")
        .downcast::<Object<List>>()
        .expect("failed downcast arguments as List for if");

    let expr = args.pop_front().expect("failed to get expr");
    let if_expr = args.pop_front().expect("failed to get if expr");

    stack.state.push_front(EvalState::If);

    if let Some(value) = args.pop_front() {
        stack.value.push_front(value);
    } else {
        stack
            .value
            .push_front(nil_value(stack.scope.front().unwrap()).into_value());
    }
    stack.value.push_front(if_expr);

    stack.value.push_front(expr);
    stack.state.push_front(EvalState::Eval);
}

#[inline]
pub fn def_special_form(stack: &mut Stack) {
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for def")
        .downcast::<Object<List>>()
        .expect("failed downcast arguments as List for def");

    let key = args.pop_front().expect("failed to get key for def");
    let value = args.pop_front().expect("failed to get value for def");

    // returns nil
    stack
        .value
        .push_front(nil_value(stack.scope.front().unwrap()).into_value());

    stack.value.push_front(key);
    stack.value.push_front(value);

    stack.state.push_front(EvalState::Def);
    stack.state.push_front(EvalState::Eval);
}

#[inline]
fn build_function(stack: &mut Stack) -> (Option<Gc<Object<Symbol>>>, Gc<Object<List>>, Gc<Value>) {
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for function")
        .downcast::<Object<List>>()
        .expect("failed downcast arguments as List for function");

    let (name, params) = {
        let first = args
            .pop_front()
            .expect("failed to get function name/params for fn");

        match first.downcast::<Object<Symbol>>() {
            Ok(name) => {
                let params = stack
                    .value
                    .pop_front()
                    .expect("failed to get function params")
                    .downcast::<Object<List>>()
                    .expect("failed to downcast function params as List");
                (Some(name), params)
            }
            Err(first) => match first.downcast::<Object<List>>() {
                Ok(params) => (None, params),
                Err(_) => panic!("invalid function params provided to fn"),
            },
        }
    };
    let body = args.pop_front().expect("failed to function get body");

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
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for do")
        .downcast::<Object<List>>()
        .expect("failed to downcast do arguments as List");

    let mut first = false;

    while let Some(value) = args.pop_back() {
        if !first {
            first = true;
        } else {
            stack.state.push_front(EvalState::PopValue);
        }
        stack.state.push_front(EvalState::Eval);

        stack.value.push_front(value);
    }
}

#[inline]
pub fn quote_special_form(stack: &mut Stack) {
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote")
        .downcast::<Object<List>>()
        .expect("failed to downcast quote arguments as List");

    if let Some(value) = args.pop_front() {
        stack.value.push_front(value);
    }
}

#[inline]
pub fn unquote_special_form(stack: &mut Stack) {
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote")
        .downcast::<Object<List>>()
        .expect("failed to downcast quote arguments as List");

    if let Some(value) = args.pop_front() {
        stack.value.push_front(value);
        stack.state.push_front(EvalState::Eval);
    }
}

#[inline]
pub fn read_special_form(stack: &mut Stack) {
    let mut args = stack
        .value
        .pop_front()
        .expect("failed to get arguments for quote")
        .downcast::<Object<List>>()
        .expect("failed to downcast quote arguments as List");

    if let Some(value) = args.pop_front() {
        let string = value
            .downcast::<Object<String>>()
            .expect("failed to downcast read argument to String");
        let char_list = string.chars().collect::<::std::vec::Vec<char>>();
        let mut reader = Reader::new(char_list);
        let value = read_value(
            stack.scope.front().expect("failed to get scope"),
            &mut reader,
        );

        stack.value.push_front(value);
    } else {
        stack
            .value
            .push_front(nil_value(stack.scope.front().unwrap()).into_value());
    }
}
