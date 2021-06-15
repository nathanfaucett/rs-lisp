use alloc::string::ToString;
use alloc::vec::Vec;
use core::ops::Deref;

use gc::Gc;

use super::{
    escape_kind, expand_special_form, function_kind, get_stack, list_kind, macro_kind, map_kind,
    new_keyword, new_list, new_list_from, new_map, new_scope, new_string, new_usize, new_vector,
    new_vector_from, nil_value, read_value, scope_get, scope_set, special_form_kind, symbol_kind,
    vector_kind, Escape, EvalState, Function, FunctionKind, List, Map, Object, Reader, Scope,
    SpecialForm, Stack, Symbol, UnwindResult, Value, Vector,
};

#[inline]
pub fn read<T>(scope: &Gc<Object<Scope>>, string: T) -> Gc<dyn Value>
where
    T: ToString,
{
    let char_list = string.to_string().chars().collect::<Vec<char>>();
    let mut reader = Reader::new(None, char_list);
    read_value(scope, &mut reader)
}

#[inline]
pub fn run<T>(scope: &Gc<Object<Scope>>, string: T) -> (Gc<Object<Scope>>, Gc<dyn Value>)
where
    T: ToString,
{
    eval_raw(scope, read(scope, string))
}

#[inline]
pub fn eval(scope: &Gc<Object<Scope>>, value: Gc<dyn Value>) -> (Gc<Object<Scope>>, Gc<dyn Value>) {
    let mut stack = get_stack(scope).clone();

    stack.push_scope_and_value(scope.clone(), value);

    loop {
        match stack.state.pop_front() {
            Some(state) => match state {
                EvalState::Eval => eval_eval_evaluated(&mut stack),
                EvalState::EvalVec => panic!("invalid state EvalVec"),
                EvalState::EvalMap => panic!("invalid state EvalMap"),
                EvalState::EvalMapKeyValue => panic!("invalid state EvalMapKeyValue"),
                EvalState::Call => eval_call_evaluated(&mut stack),
                EvalState::CallFunction => eval_call_function(&mut stack),
                EvalState::PopValue => eval_pop_value(&mut stack),
                EvalState::PopScope => eval_pop_scope(&mut stack),
                EvalState::Throw => eval_throw(&mut stack),
                EvalState::Catch => panic!("invalid state Catch"),
                EvalState::If => eval_if(&mut stack),
                EvalState::Def => eval_def(&mut stack),
                EvalState::Expand => eval_expand(&mut stack),
            },
            None => break,
        }
    }

    stack
        .pop_scope_and_value()
        .expect("failed to get value from stack")
}

#[inline]
fn eval_raw(scope: &Gc<Object<Scope>>, value: Gc<dyn Value>) -> (Gc<Object<Scope>>, Gc<dyn Value>) {
    let mut stack = get_stack(scope).clone();

    stack.push_scope_and_value(scope.clone(), value);

    loop {
        match stack.state.pop_front() {
            Some(state) => match state {
                EvalState::Eval => eval_eval(&mut stack),
                EvalState::EvalVec => eval_eval_vec(&mut stack),
                EvalState::EvalMap => eval_eval_map(&mut stack),
                EvalState::EvalMapKeyValue => eval_eval_map_key_value(&mut stack),
                EvalState::Call => eval_call(&mut stack),
                EvalState::CallFunction => eval_call_function(&mut stack),
                EvalState::PopValue => eval_pop_value(&mut stack),
                EvalState::PopScope => eval_pop_scope(&mut stack),
                EvalState::Throw => eval_throw(&mut stack),
                EvalState::Catch => panic!("invalid state Catch"),
                EvalState::If => eval_if(&mut stack),
                EvalState::Def => eval_def(&mut stack),
                EvalState::Expand => eval_expand(&mut stack),
            },
            None => break,
        }
    }

    stack
        .pop_scope_and_value()
        .expect("failed to get value from stack")
}

#[inline]
fn eval_eval(stack: &mut Stack) {
    let value = stack.value.pop_front().expect("failed to get value");
    let scope = stack.scope.front().unwrap();

    if value.kind() == symbol_kind(scope) {
        let symbol = value
            .downcast_ref::<Object<Symbol>>()
            .expect("failed to downcast value to Symbol");

        if let Some(value) = scope_get(scope, symbol.value().deref()) {
            stack.value.push_front(value.clone());
        } else {
            stack
                .value
                .push_front(nil_value(scope).clone().into_value());
        }
    } else if value.kind() == list_kind(scope) {
        let mut list = value
            .downcast_ref::<Object<List>>()
            .expect("failed to downcast value to List")
            .value()
            .clone();

        if let Some(value) = list.pop_front() {
            stack.state.push_front(EvalState::Call);
            stack
                .value
                .push_front(new_vector_from(scope, list.iter().collect::<Vector>()).into_value());

            stack.state.push_front(EvalState::Eval);
            stack.value.push_front(value.clone());
        } else {
            stack.value.push_front(value);
        }
    } else if value.kind() == vector_kind(scope) {
        let mut vector = value
            .downcast_ref::<Object<Vector>>()
            .expect("failed to downcast value to Vector")
            .value()
            .clone();

        if let Some(value) = vector.pop_front() {
            stack.state.push_front(EvalState::EvalVec);

            stack
                .value
                .push_front(new_vector_from(scope, vector).into_value());
            stack.value.push_front(new_vector(scope).into_value());

            stack.state.push_front(EvalState::Eval);
            stack.value.push_front(value);
        } else {
            stack.value.push_front(value);
        }
    } else if value.kind() == map_kind(scope) {
        let mut key_values = new_list_from(
            scope,
            value
                .downcast_ref::<Object<Map>>()
                .expect("failed to downcast value to Map")
                .iter()
                .map(|(k, v)| {
                    let mut key_value = new_list(scope);
                    key_value.push_back(k.clone());
                    key_value.push_back(v.clone());
                    key_value.into_value()
                })
                .collect::<List>(),
        );

        if let Some(mut key_value) = key_values
            .pop_back()
            .and_then(|key_value| key_value.downcast_ref::<Object<List>>().map(Clone::clone))
        {
            stack.state.push_front(EvalState::EvalMap);

            stack.value.push_front(key_values.into_value());
            stack.value.push_front(new_map(scope).into_value());

            stack.state.push_front(EvalState::EvalMapKeyValue);
            stack.state.push_front(EvalState::Eval);

            stack.value.push_front(key_value.pop_front().unwrap());
            stack.value.push_front(key_value.pop_front().unwrap());
        } else {
            stack.value.push_front(value);
        }
    } else {
        stack.value.push_front(value);
    }
}

#[inline]
fn eval_eval_evaluated(stack: &mut Stack) {
    let value = stack.value.pop_front().expect("failed to get value");
    let scope = stack.scope.front().unwrap();

    if value.kind() == list_kind(scope) {
        let mut list = value
            .downcast_ref::<Object<List>>()
            .map(Clone::clone)
            .expect("failed to downcast value to List");
        if let Some(value) = list.pop_front() {
            stack.state.push_front(EvalState::Call);
            stack
                .value
                .push_front(new_vector_from(scope, list.iter().collect::<Vector>()).into_value());

            stack.state.push_front(EvalState::Eval);
            stack.value.push_front(value.clone());
        } else {
            stack.value.push_front(value);
        }
    } else {
        stack.value.push_front(value);
    }
}

#[inline]
fn eval_eval_vec(stack: &mut Stack) {
    let evaluated_value = stack
        .value
        .pop_front()
        .expect("failed to get value from stack");
    let mut evaluated_vector = stack
        .value
        .pop_front()
        .expect("failed to get evaluated vec from stack")
        .downcast_ref::<Object<Vector>>()
        .expect("failed to downcast evaluated vector to Vector")
        .clone();
    let mut vector = stack
        .value
        .pop_front()
        .expect("failed to get vector from stack")
        .downcast_ref::<Object<Vector>>()
        .expect("failed to downcast vector to Vector")
        .clone();

    evaluated_vector.push(evaluated_value);

    if let Some(value) = vector.pop_front() {
        stack.state.push_front(EvalState::EvalVec);

        stack.value.push_front(vector.clone().into_value());
        stack
            .value
            .push_front(evaluated_vector.clone().into_value());

        stack.state.push_front(EvalState::Eval);
        stack.value.push_front(value.clone());
    } else {
        stack.value.push_front(evaluated_vector.into_value());
    }
}

#[inline]
fn eval_eval_map(stack: &mut Stack) {
    let evaluated_key = stack
        .value
        .pop_front()
        .expect("failed to get key from stack");
    let evaluated_value = stack
        .value
        .pop_front()
        .expect("failed to get value from stack");
    let mut evaluated_map = stack
        .value
        .pop_front()
        .expect("failed to get evaluated map from stack")
        .downcast_ref::<Object<Map>>()
        .expect("failed to downcast evaluated map to vec")
        .clone();
    let mut key_values = stack
        .value
        .pop_front()
        .expect("failed to get map from stack")
        .downcast_ref::<Object<List>>()
        .expect("failed to downcast map to Vec of key values")
        .clone();

    evaluated_map.set(evaluated_key, evaluated_value);

    if let Some(mut key_value) = key_values
        .pop_front()
        .and_then(|key_value| key_value.downcast_ref::<Object<List>>().map(Clone::clone))
    {
        stack.state.push_front(EvalState::EvalMap);

        stack.value.push_front(key_values.into_value());
        stack.value.push_front(evaluated_map.into_value());

        stack.state.push_front(EvalState::EvalMapKeyValue);
        stack.state.push_front(EvalState::Eval);

        stack.value.push_front(key_value.pop_front().unwrap());
        stack.value.push_front(key_value.pop_front().unwrap());
    } else {
        stack.value.push_front(evaluated_map.into_value());
    }
}

#[inline]
fn eval_eval_map_key_value(stack: &mut Stack) {
    let value = stack
        .value
        .pop_front()
        .expect("failed to get key from stack");
    let key = stack
        .value
        .pop_front()
        .expect("failed to get key from stack");

    stack.state.push_front(EvalState::Eval);

    stack.value.push_front(value);
    stack.value.push_front(key);
}

#[inline]
fn eval_call(stack: &mut Stack) {
    let callable = stack
        .value
        .pop_front()
        .expect("failed to get callable value")
        .clone();
    let mut arguments_value = stack
        .value
        .pop_front()
        .expect("failed to get arguments from stack")
        .clone();
    let arguments = arguments_value
        .downcast_mut::<Object<Vector>>()
        .expect("failed to downcast arguments to Vector");
    let scope = stack.scope.front().unwrap();

    if callable.kind() == function_kind(scope) {
        stack.state.push_front(EvalState::PopScope);
        stack.state.push_front(EvalState::CallFunction);

        stack.value.push_front(callable);

        if let Some(value) = arguments.pop_front() {
            stack.state.push_front(EvalState::EvalVec);

            stack.value.push_front(arguments.clone().into_value());
            stack.value.push_front(new_vector(scope).into_value());

            stack.state.push_front(EvalState::Eval);
            stack.value.push_front(value);
        } else {
            stack.value.push_front(arguments_value);
        }
    } else if callable.kind() == macro_kind(scope) {
        stack.state.push_front(EvalState::Eval);
        stack.state.push_front(EvalState::PopScope);
        stack.state.push_front(EvalState::CallFunction);

        stack.value.push_front(callable);
        stack.value.push_front(arguments.clone().into_value());
    } else if callable.kind() == special_form_kind(scope) {
        let special_form = callable
            .downcast_ref::<Object<SpecialForm>>()
            .expect("failed downcast value to SpecialForm");
        stack.value.push_front(arguments.clone().into_value());
        (special_form.value().deref())(stack);
    } else {
        panic!("Failed to call non-callable value {:?}", callable);
    }
}

#[inline]
fn eval_call_evaluated(stack: &mut Stack) {
    let callable = stack
        .value
        .pop_front()
        .expect("failed to get callable value")
        .clone();
    let arguments = stack
        .value
        .pop_front()
        .expect("failed to get arguments from stack")
        .downcast_ref::<Object<Vector>>()
        .expect("failed to downcast arguments to Vector")
        .clone();
    let scope = stack.scope.front().unwrap();

    if callable.kind() == function_kind(scope) {
        stack.state.push_front(EvalState::PopScope);
        stack.state.push_front(EvalState::CallFunction);

        stack.value.push_front(callable);
        stack.value.push_front(arguments.into_value());
    } else if callable.kind() == macro_kind(scope) {
        stack.state.push_front(EvalState::Eval);
        stack.state.push_front(EvalState::PopScope);
        stack.state.push_front(EvalState::CallFunction);

        stack.value.push_front(callable);
        stack.value.push_front(arguments.into_value());
    } else if callable.kind() == special_form_kind(scope) {
        let special_form = callable
            .downcast_ref::<Object<SpecialForm>>()
            .expect("failed downcast value to SpecialForm");
        stack.value.push_front(arguments.into_value());
        (special_form.value().deref())(stack);
    } else {
        panic!("Failed to call non-callable value {:?}", callable);
    }
}

#[inline]
fn eval_call_function(stack: &mut Stack) {
    let stack_scope = stack.scope.front().expect("failed to get scope from stack");
    let arguments = stack
        .value
        .pop_front()
        .expect("failed to get arguments from stack")
        .downcast_ref::<Object<Vector>>()
        .expect("failed to downcast arguments to Vector")
        .clone();
    let callable = stack
        .value
        .pop_front()
        .expect("failed to get callable from stack")
        .downcast_ref::<Object<Function>>()
        .expect("failed to downcast callable to Function")
        .clone();
    let scope = if callable.kind() == macro_kind(stack_scope) {
        new_scope(stack_scope)
    } else {
        new_scope(callable.scope())
    };

    scope_set(&scope, "arguments", arguments.clone().into_value());

    if let Some(name) = callable.value().name() {
        scope_set(&scope, name.value().deref(), callable.clone().into_value());
    }

    let mut index = 0;
    let nil = nil_value(&scope).clone().into_value();

    for param in callable.value().params().value() {
        if let Some(key) = param.downcast_ref::<Object<Symbol>>() {
            scope_set(
                &scope,
                key.value().deref(),
                arguments.get(index).unwrap_or(&nil).clone(),
            );
        }
        index += 1;
    }

    stack.scope.push_front(scope.clone());
    stack.callable.push_front(callable.clone());

    match callable.value().body() {
        &FunctionKind::Internal(ref body) => {
            stack.value.push_front(body.clone());
            stack.state.push_front(EvalState::Eval);
        }
        &FunctionKind::External(ref body) => {
            let value = (&**body)(&scope, &arguments);
            stack.value.push_front(value);
        }
    }
}

#[inline]
fn eval_pop_value(stack: &mut Stack) {
    stack.value.pop_front().expect("failed to pop value");
}

#[inline]
fn eval_pop_scope(stack: &mut Stack) {
    stack.callable.pop_front().expect("failed to pop callable");
    stack.scope.pop_front().expect("failed to pop scope");
}

#[inline]
fn eval_throw(stack: &mut Stack) {
    let scope = stack
        .scope
        .front()
        .expect("failed to get scope in throw")
        .clone();
    let error_value = stack
        .value
        .pop_front()
        .unwrap_or_else(|| nil_value(&scope).clone().into_value());

    let mut error = new_map(&scope);
    let mut stack_trace = new_vector(&scope);

    error.set(new_keyword(&scope, "value").into_value(), error_value);
    error.set(
        new_keyword(&scope, "stack_trace").into_value(),
        stack_trace.clone().into_value(),
    );

    loop {
        match stack.unwind() {
            UnwindResult::Callable(callable) => {
                stack_trace.push(
                    new_string(
                        &scope,
                        format!(
                            "{}, {}",
                            callable
                                .name()
                                .map(|name| new_string(&scope, name))
                                .unwrap_or_else(|| new_string(&scope, "anonymous")),
                            callable
                                .meta()
                                .map(|meta| {
                                    format!(
                                        "{} {:?}:{:?}",
                                        meta.get(&new_keyword(&scope, "filename").into_value())
                                            .and_then(|filename| filename
                                                .downcast_ref::<Object<String>>()
                                                .map(|object| object.value())
                                                .map(Clone::clone))
                                            .unwrap_or_else(String::new),
                                        meta.get(&new_keyword(&scope, "line").into_value())
                                            .map(Clone::clone)
                                            .unwrap_or_else(|| new_usize(&scope, 1).into_value()),
                                        meta.get(&new_keyword(&scope, "col").into_value())
                                            .map(Clone::clone)
                                            .unwrap_or_else(|| new_usize(&scope, 1).into_value())
                                    )
                                })
                                .unwrap_or_else(String::new)
                        ),
                    )
                    .into_value(),
                );
            }
            UnwindResult::Caught(handler) => {
                let mut args = new_vector(&scope);

                args.push(error.into_value());

                stack.state.push_front(EvalState::Call);
                stack.value.push_front(args.into_value());
                stack.state.push_front(EvalState::Eval);
                stack.value.push_front(handler);
                break;
            }
            UnwindResult::Uncaught => {
                panic!("Uncaught Error: {:?}", error.into_value());
            }
        }
    }
}

#[inline]
fn eval_if(stack: &mut Stack) {
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
        .downcast_ref::<Object<bool>>()
        .expect("failed to downcast expr as Bool")
        .value()
        == &true
    {
        stack.value.push_front(if_expr);
    } else {
        stack.value.push_front(else_expr);
    }
    stack.state.push_front(EvalState::Eval);
}

#[inline]
fn eval_def(stack: &mut Stack) {
    let value = stack
        .value
        .pop_front()
        .expect("failed to get def value from stack");
    let name = stack
        .value
        .pop_front()
        .expect("failed to get def name from stack")
        .downcast_ref::<Object<Symbol>>()
        .expect("failed to downcast name to Symbol")
        .clone();

    let scope = stack.scope.pop_front().expect("failed to get scope");
    scope_set(&scope, name.value().deref(), value);
    stack.scope.push_front(scope);
}

#[inline]
fn eval_expand(stack: &mut Stack) {
    let evaluated_value = stack
        .value
        .pop_front()
        .expect("failed to get value from stack");
    let mut evaluated_list = stack
        .value
        .pop_front()
        .expect("failed to get evaluated list from stack")
        .downcast_ref::<Object<List>>()
        .expect("failed to downcast evaluated list to List")
        .clone();
    let mut list = stack
        .value
        .pop_front()
        .expect("failed to get list from stack")
        .downcast_ref::<Object<List>>()
        .expect("failed to downcast list to List")
        .clone();
    let scope = stack.scope.front().unwrap();

    evaluated_list.push_back(evaluated_value);

    if let Some(value) = list.pop_front() {
        stack.state.push_front(EvalState::Expand);

        stack.value.push_front(list.clone().into_value());
        stack.value.push_front(evaluated_list.into_value());

        if value.kind() == escape_kind(scope) {
            let escape = value
                .downcast_ref::<Object<Escape>>()
                .expect("failed to downcast expand value to Escape");

            stack.state.push_front(EvalState::Eval);
            stack.value.push_front(escape.escape_value().clone());
        } else if value.kind() == list_kind(scope) {
            stack.value.push_front(
                new_vector_from(
                    scope,
                    value
                        .downcast_ref::<Object<List>>()
                        .expect("failed to downcast expand value to List")
                        .value()
                        .iter()
                        .collect::<Vector>(),
                )
                .into_value(),
            );
            expand_special_form(stack);
        } else {
            stack.value.push_front(value.clone());
        }
    } else {
        stack.value.push_front(evaluated_list.into_value());
    }
}

#[inline]
pub fn run_in_scope<T>(scope: &Gc<Object<Scope>>, content: T) -> (Gc<Object<Scope>>, Gc<dyn Value>)
where
    T: ToString,
{
    let mut raw = content.to_string();
    raw.push(')');
    raw.insert_str(0, "(do ");
    run(scope, raw)
}
