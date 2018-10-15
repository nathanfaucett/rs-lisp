use std::collections::LinkedList;

use gc::Gc;

use super::{Context, Function, Kind, List, Object, Scope, SpecialForm, Value};

#[derive(Debug)]
pub enum State {
    Eval,
    EvalList,
    EvalArgs,
    CallWithNewScope,
    Call,
    Return,
    If,
    Def,
}

#[derive(Debug)]
pub struct Stack {
    pub value: LinkedList<Gc<Value>>,
    pub scope: LinkedList<Gc<Object<Scope>>>,
    pub state: LinkedList<State>,
}

impl Stack {
    #[inline]
    pub fn new(scope: Gc<Object<Scope>>, value: Gc<Value>) -> Self {
        let mut stack = Stack {
            value: LinkedList::new(),
            scope: LinkedList::new(),
            state: LinkedList::new(),
        };

        stack.value.push_front(value);
        stack.scope.push_front(scope);
        stack.state.push_front(State::Eval);

        return stack;
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
        stack.value.push_front(
            Context::new_nil(stack.scope.back().expect("failed to get scope")).into_value(),
        ); // else expr
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
pub fn fn_special_form(stack: &mut Stack) {
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

    stack.value.push_front(
        Context::new_function(
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
        .expect("failed to downcast args as List");

    while let Some(value) = list.pop_back() {
        stack.value.push_front(value);
        stack.state.push_front(State::Eval);
    }
}

#[inline]
pub fn eval(scope: Gc<Object<Scope>>, value: Gc<Value>) -> Gc<Value> {
    let mut stack = Stack::new(scope.clone(), value.clone());

    let nil_value = unsafe {
        scope
            .get_with_type::<()>("nil")
            .expect("failed to get nil value")
    };
    let scope_kind = unsafe {
        scope
            .get_with_type::<Kind>("Scope")
            .expect("failed to get Scope kind")
    };
    let function_kind = unsafe {
        scope
            .get_with_type::<Kind>("Function")
            .expect("failed to get Function kind")
    };
    let macro_kind = unsafe {
        scope
            .get_with_type::<Kind>("Macro")
            .expect("failed to get Macro kind")
    };
    let special_form_kind = unsafe {
        scope
            .get_with_type::<Kind>("SpecialForm")
            .expect("failed to get SpecialForm kind")
    };
    let symbol_kind = unsafe {
        scope
            .get_with_type::<Kind>("Symbol")
            .expect("failed to get Symbol kind")
    };
    let list_kind = unsafe {
        scope
            .get_with_type::<Kind>("List")
            .expect("failed to get List kind")
    };

    loop {
        match stack.state.pop_front() {
            Some(state) => match state {
                State::Eval => {
                    let value = stack
                        .value
                        .pop_front()
                        .expect("failed to get value from stack");

                    if value.kind() == &symbol_kind {
                        let string = value
                            .downcast_ref::<Object<String>>()
                            .expect("failed to get String");

                        if let Some(value) = stack
                            .scope
                            .front()
                            .expect("failed to get scope")
                            .get(string.value())
                        {
                            stack.value.push_front(value.clone());
                        } else {
                            stack.value.push_front(nil_value.clone().into_value());
                        }
                    } else if value.kind() == &list_kind {
                        let mut list = value
                            .downcast::<Object<List>>()
                            .expect("failed to downcast value as List");

                        if list.value().is_empty() {
                            stack.value.push_front(list.into_value());
                        } else {
                            list = unsafe {
                                Gc::new(Object::new(
                                    list_kind.clone(),
                                    list.value().iter().map(Clone::clone).collect::<List>(),
                                ))
                            };
                            let first = list
                                .value_mut()
                                .pop_front()
                                .expect("failed get first value from list");

                            stack.value.push_front(list.into_value());
                            stack.value.push_front(first);
                            stack.state.push_front(State::EvalList);
                            stack.state.push_front(State::Eval);
                        }
                    } else {
                        stack.value.push_front(value);
                    }
                }
                State::EvalList => {
                    let mut callable = stack
                        .value
                        .pop_front()
                        .expect("failed to get first value from list");
                    let mut list = stack
                        .value
                        .pop_front()
                        .expect("failed to get list from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast list to List");

                    if callable.kind() == &function_kind {
                        stack.value.push_front(callable);
                        stack.state.push_front(State::Return);
                        stack.state.push_front(State::Call);
                        stack.state.push_front(State::CallWithNewScope);

                        if let Some(first) = list.pop_front() {
                            stack.value.push_front(list.into_value());
                            stack.value.push_front(
                                unsafe { Gc::new(Object::new(list_kind.clone(), List::new())) }
                                    .into_value(),
                            );
                            stack.state.push_front(State::EvalArgs);

                            stack.value.push_front(first);
                            stack.state.push_front(State::Eval);
                        } else {
                            stack.value.push_front(
                                unsafe { Gc::new(Object::new(list_kind.clone(), List::new())) }
                                    .into_value(),
                            );
                        }
                    } else if callable.kind() == &macro_kind {
                        stack.value.push_front(callable);
                        stack.value.push_front(list.into_value());
                        stack.state.push_front(State::Eval);
                        stack.state.push_front(State::Return);
                        stack.state.push_front(State::Call);
                        stack.state.push_front(State::CallWithNewScope);
                    } else if callable.kind() == &special_form_kind {
                        let special_form = callable
                            .downcast::<Object<SpecialForm>>()
                            .expect("failed downcast value to SpecialForm");
                        stack.value.push_front(list.into_value());
                        (special_form.value().inner())(&mut stack);
                    } else {
                        panic!("Failed to call value {:?}", callable);
                    }
                }
                State::EvalArgs => {
                    let mut evaluated_arg = stack
                        .value
                        .pop_front()
                        .expect("failed to get arg from stack");
                    let mut evaluated_args = stack
                        .value
                        .pop_front()
                        .expect("failed to get evaluated args from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast evaluated args to List");
                    let mut args = stack
                        .value
                        .pop_front()
                        .expect("failed to get args from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downdcast args to List");

                    evaluated_args.value_mut().push_front(evaluated_arg);

                    if let Some(first) = args.pop_front() {
                        stack.value.push_front(args.into_value());
                        stack.value.push_front(evaluated_args.into_value());
                        stack.state.push_front(State::EvalArgs);
                        stack.value.push_front(first);
                        stack.state.push_front(State::Eval);
                    } else {
                        stack.value.push_front(evaluated_args.into_value());
                    }
                }
                State::CallWithNewScope => {
                    let mut values = stack
                        .value
                        .pop_front()
                        .expect("failed to get call args from stack");
                    let mut callable = stack
                        .value
                        .pop_front()
                        .expect("failed to get callable from stack")
                        .downcast::<Object<Function>>()
                        .expect("failed to cast callable to Function");

                    let mut scope = unsafe {
                        Gc::new(Object::new(
                            scope_kind.clone(),
                            Scope::new(Some(callable.value().scope().clone())),
                        ))
                    };

                    stack.value.push_front(callable.into_value());
                    stack.value.push_front(values);
                    stack.scope.push_front(scope);
                }
                State::Call => {
                    let mut values = stack
                        .value
                        .pop_front()
                        .expect("failed to get values from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast values to List")
                        .iter()
                        .map(Clone::clone)
                        .collect::<Vec<Gc<Value>>>();
                    let mut callable = stack
                        .value
                        .pop_front()
                        .expect("failed to get callable from stack")
                        .downcast::<Object<Function>>()
                        .expect("failed to downcast callable to Function");
                    let mut scope = stack
                        .scope
                        .front_mut()
                        .expect("failed to get scope from stack");

                    let mut index = 0;
                    let nil = nil_value.clone().into_value();
                    for param in callable.value().params().value() {
                        if let Some(key) = param.downcast_ref::<Object<String>>() {
                            scope.set(key.value(), values.get(index).unwrap_or(&nil).clone());
                        }
                        index += 1;
                    }

                    if let Some(name) = callable.value().name() {
                        scope.set(name.value(), callable.clone().into_value());
                    }

                    stack.value.push_front(callable.value().body().clone());
                    stack.state.push_front(State::Eval);
                }
                State::Return => {
                    stack
                        .scope
                        .pop_front()
                        .expect("failed to pop scope from stack");
                }
                State::If => {
                    let expr = stack
                        .value
                        .pop_front()
                        .expect("failed to get expr from stack");
                    let if_expr = stack
                        .value
                        .pop_front()
                        .expect("failed to get if expr from stack");
                    let else_expr = stack
                        .value
                        .pop_front()
                        .expect("failed to get else expr form stack");

                    if expr
                        .downcast::<Object<bool>>()
                        .expect("failed to downcast expr as Boolean")
                        .value()
                        == &true
                    {
                        stack.value.push_front(if_expr);
                    } else {
                        stack.value.push_front(else_expr);
                    }
                    stack.state.push_front(State::Eval);
                }

                State::Def => {
                    let value = stack
                        .value
                        .pop_front()
                        .expect("failed to get if value from stack");
                    let key = stack
                        .value
                        .pop_front()
                        .expect("failed to get key from stack")
                        .downcast::<Object<String>>()
                        .expect("failed to downcast key to String");

                    stack
                        .scope
                        .front_mut()
                        .expect("failed to get scope")
                        .value_mut()
                        .set(key.value(), value);
                }
            },
            None => break,
        }
    }

    stack
        .value
        .pop_front()
        .expect("failed to get value from stack")
}

#[cfg(test)]
mod test {
    use super::*;

    use super::super::Context;

    #[test]
    fn test() {
        let context = Context::new();

        let input = lisp!(context.scope(), (do
            (def test (fn (a) (if a true false)))
            (test true)
        ))
        .into_value();
        let output = eval(context.scope().clone(), input);
        println!("{:?}", output);

        assert_eq!(
            output.downcast::<Object<bool>>().expect("failed").value(),
            &true
        );
    }
}
