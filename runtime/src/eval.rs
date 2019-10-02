use alloc::collections::LinkedList;
use alloc::string::ToString;

use gc::{Gc, Trace};

use super::{
    escape_kind, expand_special_form, function_kind, list_kind, macro_kind, map_kind, new_list,
    new_map, new_scope, new_vec, nil_kind, nil_value, read_value, special_form_kind, symbol_kind,
    vec_kind, Escape, Function, FunctionKind, List, Map, Object, Reader, Scope, SpecialForm,
    Symbol, Value, Vec,
};

#[derive(Debug)]
pub enum EvalState {
    Eval,
    EvalVec,
    EvalMap,
    EvalMapKeyValue,
    Call,
    CallFunction,
    CallFunctionEvalArgs,
    PopValue,
    PopScope,
    If,
    Def,
    Expand,
    Throw,
    Catch,
}

#[derive(Debug)]
pub struct Stack {
    pub value: LinkedList<Gc<dyn Value>>,
    pub scope: LinkedList<Gc<Object<Scope>>>,
    pub state: LinkedList<EvalState>,
}

impl Stack {
    #[inline]
    pub fn new(scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> Self {
        let mut stack = Stack {
            value: LinkedList::new(),
            scope: LinkedList::new(),
            state: LinkedList::new(),
        };

        stack.value.push_front(value);
        stack.scope.push_front(scope);
        stack.state.push_front(EvalState::Eval);

        return stack;
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct LinkedMap(LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>);

impl Trace for LinkedMap {
    fn mark(&mut self) {
        for (k, v) in self.0.iter_mut() {
            k.mark();
            v.mark();
        }
    }
}

#[inline]
pub fn eval(scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<dyn Value> {
    let mut stack = Stack::new(scope.clone(), value);

    loop {
        match stack.state.pop_front() {
            Some(state) => match state {
                EvalState::Eval => {
                    let value = stack.value.pop_front().expect("failed to get value");

                    if value.kind() == &symbol_kind(stack.scope.front().unwrap().clone()) {
                        let string = value
                            .downcast_ref::<Object<Symbol>>()
                            .expect("failed to downcast value to Symbol");

                        if let Some(value) =
                            stack.scope.front().unwrap().get(string.value().inner())
                        {
                            stack.value.push_front(value.clone());
                        } else {
                            stack.value.push_front(
                                nil_value(stack.scope.front().unwrap().clone()).into_value(),
                            );
                        }
                    } else if value.kind() == &list_kind(stack.scope.front().unwrap().clone()) {
                        let mut list = value
                            .downcast::<Object<List>>()
                            .expect("failed to downcast value to List")
                            .clone_ref();

                        if let Some(value) = list.pop_front() {
                            stack.state.push_front(EvalState::Call);
                            stack.value.push_front(list.into_value());

                            stack.state.push_front(EvalState::Eval);
                            stack.value.push_front(value);
                        } else {
                            stack.value.push_front(list.into_value());
                        }
                    } else if value.kind() == &vec_kind(stack.scope.front().unwrap().clone()) {
                        let mut vec = value
                            .downcast::<Object<Vec>>()
                            .expect("failed to downcast value to Vec")
                            .clone_ref();

                        if let Some(value) = vec.pop() {
                            stack.state.push_front(EvalState::EvalVec);

                            stack.value.push_front(vec.into_value());
                            stack.value.push_front(
                                new_vec(stack.scope.front().unwrap().clone()).into_value(),
                            );

                            stack.state.push_front(EvalState::Eval);
                            stack.value.push_front(value);
                        } else {
                            stack.value.push_front(
                                new_vec(stack.scope.front().unwrap().clone()).into_value(),
                            );
                        }
                    } else if value.kind() == &map_kind(stack.scope.front().unwrap().clone()) {
                        let mut key_values = unsafe {
                            Gc::new(Object::new(
                                nil_kind(stack.scope.front().unwrap().clone()),
                                LinkedMap(
                                    value
                                        .downcast::<Object<Map>>()
                                        .expect("failed to downcast value to Map")
                                        .iter()
                                        .map(|(k, v)| (k.clone(), v.clone()))
                                        .collect::<LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>>(),
                                ),
                            ))
                        };

                        if let Some((key, value)) = key_values.0.pop_back() {
                            stack.state.push_front(EvalState::EvalMap);

                            stack.value.push_front(key_values.into_value());
                            stack.value.push_front(
                                new_map(stack.scope.front().unwrap().clone()).into_value(),
                            );

                            stack.state.push_front(EvalState::EvalMapKeyValue);
                            stack.state.push_front(EvalState::Eval);

                            stack.value.push_front(key);
                            stack.value.push_front(value);
                        } else {
                            stack.value.push_front(
                                new_map(stack.scope.front().unwrap().clone()).into_value(),
                            );
                        }
                    } else {
                        stack.value.push_front(value);
                    }
                }
                EvalState::EvalVec => {
                    let evaluated_value = stack
                        .value
                        .pop_front()
                        .expect("failed to get value from stack");
                    let mut evaluated_vec = stack
                        .value
                        .pop_front()
                        .expect("failed to get evaluated vec from stack")
                        .downcast::<Object<Vec>>()
                        .expect("failed to downcast evaluated vec to vec");
                    let mut key_values = stack
                        .value
                        .pop_front()
                        .expect("failed to get vec from stack")
                        .downcast::<Object<Vec>>()
                        .expect("failed to downcast vec to Vec");

                    evaluated_vec.push_front(evaluated_value);

                    if let Some(value) = key_values.pop() {
                        stack.state.push_front(EvalState::EvalVec);

                        stack.value.push_front(key_values.into_value());
                        stack.value.push_front(evaluated_vec.into_value());

                        stack.state.push_front(EvalState::Eval);
                        stack.value.push_front(value);
                    } else {
                        stack.value.push_front(evaluated_vec.into_value());
                    }
                }
                EvalState::EvalMap => {
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
                        .downcast::<Object<Map>>()
                        .expect("failed to downcast evaluated map to vec");
                    let mut key_values = stack
                        .value
                        .pop_front()
                        .expect("failed to get map from stack")
                        .downcast::<Object<LinkedMap>>()
                        .expect("failed to downcast map to Vec of key values");

                    evaluated_map.set(evaluated_key, evaluated_value);

                    if let Some((key, value)) = key_values.0.pop_front() {
                        stack.state.push_front(EvalState::EvalMap);

                        stack.value.push_front(key_values.into_value());
                        stack.value.push_front(evaluated_map.into_value());

                        stack.state.push_front(EvalState::EvalMapKeyValue);
                        stack.state.push_front(EvalState::Eval);

                        stack.value.push_front(key);
                        stack.value.push_front(value);
                    } else {
                        stack.value.push_front(evaluated_map.into_value());
                    }
                }
                EvalState::EvalMapKeyValue => {
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
                EvalState::Call => {
                    let callable = stack
                        .value
                        .pop_front()
                        .expect("failed to get callable value");
                    let mut args = stack
                        .value
                        .pop_front()
                        .expect("failed to get args from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast args to List");

                    if callable.kind() == &function_kind(stack.scope.front().unwrap().clone()) {
                        stack.state.push_front(EvalState::PopScope);
                        stack.state.push_front(EvalState::CallFunction);

                        stack.value.push_front(callable);

                        if let Some(value) = args.pop_back() {
                            stack.state.push_front(EvalState::CallFunctionEvalArgs);

                            stack.value.push_front(args.into_value());
                            stack.value.push_front(
                                new_list(stack.scope.front().unwrap().clone()).into_value(),
                            );

                            stack.state.push_front(EvalState::Eval);
                            stack.value.push_front(value);
                        } else {
                            stack.value.push_front(args.into_value());
                        }
                    } else if callable.kind() == &macro_kind(stack.scope.front().unwrap().clone()) {
                        stack.state.push_front(EvalState::Eval);
                        stack.state.push_front(EvalState::PopScope);
                        stack.state.push_front(EvalState::CallFunction);

                        stack.value.push_front(callable);
                        stack.value.push_front(args.into_value());
                    } else if callable.kind()
                        == &special_form_kind(stack.scope.front().unwrap().clone())
                    {
                        let special_form = callable
                            .downcast::<Object<SpecialForm>>()
                            .expect("failed downcast value to SpecialForm");
                        stack.value.push_front(args.into_value());
                        (special_form.value().inner())(&mut stack);
                    } else {
                        panic!("Failed to call non-callable value {:?}", callable);
                    }
                }
                EvalState::CallFunctionEvalArgs => {
                    let evaluated_arg = stack
                        .value
                        .pop_front()
                        .expect("failed to get argument from stack");
                    let mut evaluated_args = stack
                        .value
                        .pop_front()
                        .expect("failed to get evaluated arguments from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast evaluated arguments to List");
                    let mut args = stack
                        .value
                        .pop_front()
                        .expect("failed to get arguments from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast arguments to List");

                    evaluated_args.push_front(evaluated_arg);

                    if let Some(value) = args.pop_back() {
                        stack.state.push_front(EvalState::CallFunctionEvalArgs);

                        stack.value.push_front(args.into_value());
                        stack.value.push_front(evaluated_args.into_value());

                        stack.value.push_front(value);
                        stack.state.push_front(EvalState::Eval);
                    } else {
                        stack.value.push_front(evaluated_args.into_value());
                    }
                }
                EvalState::CallFunction => {
                    let arguments = stack
                        .value
                        .pop_front()
                        .expect("failed to get values from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast values to List");
                    let values = arguments.to_vec();
                    let callable = stack
                        .value
                        .pop_front()
                        .expect("failed to get callable from stack")
                        .downcast::<Object<Function>>()
                        .expect("failed to downcast callable to Function");
                    let mut scope = new_scope(callable.scope().clone());

                    if let Some(name) = callable.value().name() {
                        scope.set(name.value().inner(), callable.clone().into_value());
                    }

                    let mut index = 0;
                    let nil = nil_value(scope.clone()).into_value();

                    for param in callable.value().params().value() {
                        if let Some(key) = param.downcast_ref::<Object<Symbol>>() {
                            scope.set(
                                key.value().inner(),
                                values.get(index).unwrap_or(&nil).clone(),
                            );
                        }
                        index += 1;
                    }

                    stack.scope.push_front(scope.clone());

                    match callable.value().body() {
                        &FunctionKind::Internal(ref body) => {
                            stack.value.push_front(body.clone());
                            stack.state.push_front(EvalState::Eval);
                        }
                        &FunctionKind::External(ref body) => {
                            let value = (&**body)(scope.clone(), arguments);
                            stack.value.push_front(value);
                        }
                    }
                }
                EvalState::PopValue => {
                    stack.value.pop_front().expect("failed to pop value");
                }
                EvalState::PopScope => {
                    stack.scope.pop_front().expect("failed to pop scope");
                }
                EvalState::If => {
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
                    stack.state.push_front(EvalState::Eval);
                }
                EvalState::Def => {
                    let value = stack
                        .value
                        .pop_front()
                        .expect("failed to get if value from stack");
                    let key = stack
                        .value
                        .pop_front()
                        .expect("failed to get key from stack")
                        .downcast::<Object<Symbol>>()
                        .expect("failed to downcast key to Symbol");

                    stack
                        .scope
                        .front_mut()
                        .expect("failed to get scope")
                        .set(key.value().inner(), value);
                }
                EvalState::Expand => {
                    let evaluated_value = stack
                        .value
                        .pop_front()
                        .expect("failed to get value from stack");
                    let mut evaluated_list = stack
                        .value
                        .pop_front()
                        .expect("failed to get evaluated vec from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast evaluated vec to vec");
                    let mut list = stack
                        .value
                        .pop_front()
                        .expect("failed to get vec from stack")
                        .downcast::<Object<List>>()
                        .expect("failed to downcast vec to Vec");

                    evaluated_list.push_back(evaluated_value);

                    if let Some(value) = list.pop_front() {
                        stack.state.push_front(EvalState::Expand);

                        stack.value.push_front(list.into_value());
                        stack.value.push_front(evaluated_list.into_value());

                        if value.kind() == &escape_kind(stack.scope.front().unwrap().clone()) {
                            let escape = value
                                .downcast::<Object<Escape>>()
                                .expect("failed to downcast expand value to Escape");

                            stack.state.push_front(EvalState::Eval);
                            stack.value.push_front(escape.inner().clone());
                        } else if value.kind() == &list_kind(stack.scope.front().unwrap().clone()) {
                            stack.value.push_front(
                                value
                                    .downcast::<Object<List>>()
                                    .expect("failed to downcast expand value to List")
                                    .clone_ref()
                                    .into_value(),
                            );
                            expand_special_form(&mut stack);
                        } else {
                            stack.value.push_front(value);
                        }
                    } else {
                        stack.value.push_front(evaluated_list.into_value());
                    }
                }
                EvalState::Catch => {}
                EvalState::Throw => {}
            },
            None => break,
        }
    }

    stack
        .value
        .pop_front()
        .expect("failed to get value from stack")
}

#[inline]
pub fn read<T>(scope: Gc<Object<Scope>>, string: T) -> Gc<dyn Value>
where
    T: ToString,
{
    let char_list = string
        .to_string()
        .chars()
        .collect::<::alloc::vec::Vec<char>>();
    let mut reader = Reader::new(char_list);
    read_value(scope, &mut reader)
}

#[inline]
pub fn run<T>(scope: Gc<Object<Scope>>, string: T) -> Gc<dyn Value>
where
    T: ToString,
{
    eval(scope.clone(), read(scope, string))
}
