use std::collections::LinkedList;

use gc::Gc;

use super::{Function, Kind, List, Object, Scope, Value};

#[derive(Debug)]
pub enum State {
    Eval,
    EvalList,
    EvalArgs,
    CallWithNewScope,
    Call,
    Return,
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
                    let value = stack.value.pop_front().unwrap();

                    if value.kind() == &symbol_kind {
                        let string = value
                            .downcast_ref::<Object<String>>()
                            .expect("failed to get String");

                        if let Some(value) = stack.scope.front().unwrap().get(string.value()) {
                            stack.value.push_front(value.clone());
                        } else {
                            stack.value.push_front(nil_value.clone().into_value());
                        }
                    } else if value.kind() == &list_kind {
                        let mut list = value.downcast::<Object<List>>().unwrap();

                        if let Some(first) = list.value_mut().pop_front() {
                            stack.value.push_front(list.into_value());
                            stack.value.push_front(first.clone());
                            stack.state.push_front(State::EvalList);
                            stack.state.push_front(State::Eval);
                        } else {
                            stack.value.push_front(list.into_value());
                        }
                    } else {
                        stack.value.push_front(value);
                    }
                }
                State::EvalList => {
                    let mut callable = stack.value.pop_front().unwrap();
                    let mut list = stack
                        .value
                        .pop_front()
                        .unwrap()
                        .downcast::<Object<List>>()
                        .unwrap();

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
                    } else {
                        panic!("Failed to call value {:?}", callable);
                    }
                }
                State::EvalArgs => {
                    let mut evaluated_arg = stack.value.pop_front().unwrap();
                    let mut evaluated_args = stack
                        .value
                        .pop_front()
                        .unwrap()
                        .downcast::<Object<List>>()
                        .unwrap();
                    let mut args = stack
                        .value
                        .pop_front()
                        .unwrap()
                        .downcast::<Object<List>>()
                        .unwrap();

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
                    let mut values = stack.value.pop_front().unwrap();
                    let mut callable = stack
                        .value
                        .pop_front()
                        .unwrap()
                        .downcast::<Object<Function>>()
                        .unwrap();

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
                        .unwrap()
                        .downcast::<Object<List>>()
                        .unwrap()
                        .iter()
                        .map(Clone::clone)
                        .collect::<Vec<Gc<Value>>>();
                    let mut callable = stack
                        .value
                        .pop_front()
                        .unwrap()
                        .downcast::<Object<Function>>()
                        .unwrap();
                    let mut scope = stack.scope.front_mut().unwrap();

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
                    stack.scope.pop_front().unwrap();
                }
            },
            None => break,
        }
    }

    stack.value.pop_front().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    use super::super::Context;

    #[test]
    fn test() {
        let mut context = Context::new();

        let hello_name = lisp!(context.scope_mut(), str "hello")
            .downcast::<Object<String>>()
            .unwrap();
        let hello_body = lisp!(context.scope_mut(), str "Hello, world!");
        let hello_params = lisp!(context.scope_mut(), ())
            .downcast::<Object<List>>()
            .unwrap();

        let function_kind = unsafe { context.scope().get_with_type::<Kind>("Function").unwrap() };
        let function = unsafe {
            Gc::new(Object::new(
                function_kind,
                Function::new(
                    Some(hello_name),
                    context.scope().clone(),
                    hello_params,
                    hello_body,
                ),
            ))
        };

        context.scope_mut().set("hello", function.into_value());

        let expr = lisp!(context.scope_mut(), (hello));
        let output = eval(context.scope().clone(), expr);

        assert_eq!(
            output.downcast::<Object<String>>().unwrap().value(),
            "Hello, world!"
        );
    }
}
