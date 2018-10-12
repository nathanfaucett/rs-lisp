use std::collections::LinkedList;

use gc::Gc;

use super::{Function, Kind, Object, Scope, Value};

#[derive(Debug)]
pub enum State {
    Eval,
    EvalCall,
}

#[inline]
pub fn eval(scope: Gc<Object<Scope>>, value: Gc<Value>) -> Gc<Value> {
    let mut scope_stack: LinkedList<Gc<Object<Scope>>> = LinkedList::new();
    let mut stack: LinkedList<Gc<Value>> = LinkedList::new();
    let mut state_stack: LinkedList<State> = LinkedList::new();

    let nil_value = unsafe {
        scope
            .get_type::<()>("nil")
            .expect("failed to get nil value")
    };
    let symbol_kind = unsafe {
        scope
            .get_type::<Kind>("Symbol")
            .expect("failed to get Symbol kind")
    };
    let list_kind = unsafe {
        scope
            .get_type::<Kind>("List")
            .expect("failed to get List kind")
    };

    scope_stack.push_front(scope);
    stack.push_front(value);
    state_stack.push_front(State::Eval);

    loop {
        match state_stack.pop_front() {
            Some(state) => match state {
                State::Eval => {
                    let value = stack.pop_front().unwrap();

                    if value.kind() == &symbol_kind {
                        let string = value
                            .downcast_ref::<Object<String>>()
                            .expect("failed to get String");

                        if let Some(value) = scope_stack.front().unwrap().get(string.value()) {
                            stack.push_front(value.clone());
                        } else {
                            stack.push_front(nil_value.clone().into_value());
                        }
                    } else if value.kind() == &list_kind {
                        let mut list = value
                            .clone()
                            .downcast::<Object<LinkedList<Gc<Value>>>>()
                            .unwrap();

                        if let Some(first) = list.value_mut().pop_front() {
                            stack.push_front(list.into_value());
                            stack.push_front(first.clone());
                            state_stack.push_front(State::EvalCall);
                            state_stack.push_front(State::Eval);
                        } else {
                            stack.push_front(nil_value.clone().into_value());
                        }
                    } else {
                        stack.push_front(value);
                    }
                }
                State::EvalCall => {
                    let callable = stack.pop_front().unwrap();
                    let args = stack.pop_front().unwrap();
                    let callable_typ = callable.kind();
                }
            },
            None => break,
        }
    }

    stack.pop_front().unwrap()
}
