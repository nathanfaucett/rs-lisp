use std::fmt;
use std::hash::{Hash, Hasher};

use gc::Gc;

use super::{new_function, new_macro, new_nil, List, Object, Stack, State, Value};

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
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for if")
        .downcast::<Object<List>>()
        .expect("failed downcast args as List for if");

    stack
        .value
        .push_front(list.pop_front().expect("failed to get expr")); // expr
    stack
        .value
        .push_front(list.pop_front().expect("failed to get if expr")); // if expr
    if let Some(value) = list.pop_front() {
        stack.value.push_front(value); // else expr
    } else {
        stack
            .value
            .push_front(new_nil(stack.scope.back().expect("failed to get scope")).into_value()); // else expr
    }

    stack.state.push_front(State::If);
    stack.state.push_front(State::Eval);
}

#[inline]
pub fn def_special_form(stack: &mut Stack) {
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for def")
        .downcast::<Object<List>>()
        .expect("failed downcast args as List for def");

    let key = list.pop_front().expect("failed to get key for def");
    let value = list.pop_front().expect("failed to get value for def");

    stack.value.push_front(key);
    stack.value.push_front(value);

    stack.state.push_front(State::Def);
    stack.state.push_front(State::Eval);
}

#[inline]
fn build_function(stack: &mut Stack) -> (Option<Gc<Object<String>>>, Gc<Object<List>>, Gc<Value>) {
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for fn")
        .downcast::<Object<List>>()
        .expect("failed downcast args as List for fn");

    let (name, params) = {
        let first = list.pop_front().expect("failed to get name/params for fn");

        match first.downcast::<Object<String>>() {
            Ok(name) => {
                let params = stack
                    .value
                    .pop_front()
                    .expect("failed to get params")
                    .downcast::<Object<List>>()
                    .expect("failed to downcast params as List");
                (Some(name), params)
            }
            Err(first) => match first.downcast::<Object<List>>() {
                Ok(params) => (None, params),
                Err(_) => panic!("invalid params provided to fn"),
            },
        }
    };
    let body = list.pop_front().expect("failed tot get body");

    (name, params, body)
}

#[inline]
pub fn fn_special_form(stack: &mut Stack) {
    let (name, params, body) = build_function(stack);

    stack.value.push_front(
        new_function(
            stack.scope.back().expect("failed to get root scope"),
            name,
            stack.scope.front().expect("failed to get scope").clone(),
            params,
            body,
        )
        .into_value(),
    );
}

#[inline]
pub fn macro_special_form(stack: &mut Stack) {
    let (name, params, body) = build_function(stack);

    stack.value.push_front(
        new_macro(
            stack.scope.back().expect("failed to get root scope"),
            name,
            stack.scope.front().expect("failed to get scope").clone(),
            params,
            body,
        )
        .into_value(),
    );
}

#[inline]
pub fn do_special_form(stack: &mut Stack) {
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for do")
        .downcast::<Object<List>>()
        .expect("failed to downcast do args as List");

    while let Some(value) = list.pop_back() {
        stack.value.push_front(value);
        stack.state.push_front(State::Eval);
    }
}

#[inline]
pub fn dot_special_form(stack: &mut Stack) {
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for dot")
        .downcast::<Object<List>>()
        .expect("failed to downcast dot args as List");
    let object = list.pop_front().expect("failed to get Object");
    let key = list.pop_front().expect("failed to get key");

    stack.value.push_front(key);
    stack.value.push_front(object);
    stack.state.push_front(State::Lookup);
    stack.state.push_front(State::Eval);
    stack.state.push_front(State::Eval);
}

#[inline]
pub fn quote_special_form(stack: &mut Stack) {
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for quote")
        .downcast::<Object<List>>()
        .expect("failed to downcast quote args as List");

    if let Some(value) = list.pop_front() {
        stack.value.push_front(value);
    }
}

#[inline]
pub fn unquote_special_form(stack: &mut Stack) {
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get args for quote")
        .downcast::<Object<List>>()
        .expect("failed to downcast quote args as List");

    if let Some(value) = list.pop_front() {
        stack.value.push_front(value);
        stack.state.push_front(State::Eval);
    }
}
