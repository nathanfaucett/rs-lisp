use alloc::collections::LinkedList;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::ops::Deref;

use gc::Gc;

use super::{
  escape_kind, expand_special_form, function_kind, list_kind, macro_kind, map_kind, new_keyword,
  new_linked_map, new_list, new_list_from, new_map, new_scope, new_symbol, new_vector,
  new_vector_from, nil_value, read_value, special_form_kind, symbol_kind, vector_kind, Escape,
  EvalState, Function, FunctionKind, LinkedMap, List, Map, Object, PopResult, Reader, Scope,
  SpecialForm, Stack, Symbol, Value, Vector,
};

#[inline]
pub fn read<T>(scope: Gc<Object<Scope>>, string: T) -> Gc<dyn Value>
where
  T: ToString,
{
  let char_list = string.to_string().chars().collect::<Vec<char>>();
  let mut reader = Reader::new(char_list);
  read_value(scope, &mut reader)
}

#[inline]
pub fn run<T>(scope: Gc<Object<Scope>>, string: T) -> Gc<dyn Value>
where
  T: ToString,
{
  eval_raw(scope.clone(), read(scope, string))
}

#[inline]
pub fn eval(scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<dyn Value> {
  let mut stack = Stack::new();

  stack.push_scope_and_value(scope, value);

  loop {
    match stack.state.pop_front() {
      Some(state) => match state {
        EvalState::Eval => eval_eval_evaluated(&mut stack),
        EvalState::EvalVec => panic!("invalid state EvalVec"),
        EvalState::EvalMap => panic!("invalid state EvalMap"),
        EvalState::EvalMapKeyValue => panic!("invalid state EvalMapKeyValue"),
        EvalState::Call => eval_call_evaluated(&mut stack),
        EvalState::CallFunctionEvalArgs => panic!("invalid state CallFunctionEvalArgs"),
        EvalState::CallFunction => eval_call_function(&mut stack),
        EvalState::PopValue => eval_pop_value(&mut stack),
        EvalState::PopScope => eval_pop_scope(&mut stack),
        EvalState::Throw => eval_throw(&mut stack),
        EvalState::Catch => eval_catch(&mut stack),
        EvalState::If => eval_if(&mut stack),
        EvalState::Def => eval_def(&mut stack),
        EvalState::Expand => eval_expand(&mut stack),
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
fn eval_raw(scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> Gc<dyn Value> {
  let mut stack = Stack::new();

  stack.push_scope_and_value(scope, value);

  loop {
    match stack.state.pop_front() {
      Some(state) => match state {
        EvalState::Eval => eval_eval(&mut stack),
        EvalState::EvalVec => eval_eval_vec(&mut stack),
        EvalState::EvalMap => eval_eval_map(&mut stack),
        EvalState::EvalMapKeyValue => eval_eval_map_key_value(&mut stack),
        EvalState::Call => eval_call(&mut stack),
        EvalState::CallFunctionEvalArgs => eval_call_function_eval_arguments(&mut stack),
        EvalState::CallFunction => eval_call_function(&mut stack),
        EvalState::PopValue => eval_pop_value(&mut stack),
        EvalState::PopScope => eval_pop_scope(&mut stack),
        EvalState::Throw => eval_throw(&mut stack),
        EvalState::Catch => eval_catch(&mut stack),
        EvalState::If => eval_if(&mut stack),
        EvalState::Def => eval_def(&mut stack),
        EvalState::Expand => eval_expand(&mut stack),
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
fn eval_eval(stack: &mut Stack) {
  let value = stack.value.pop_front().expect("failed to get value");
  let scope = stack.scope.front().unwrap();

  if value.kind() == &symbol_kind(scope.clone()) {
    let string = value
      .downcast_ref::<Object<Symbol>>()
      .expect("failed to downcast value to Symbol");

    if let Some(value) = scope.get(string.value().deref()) {
      stack.value.push_front(value.clone());
    } else {
      stack
        .value
        .push_front(nil_value(scope.clone()).into_value());
    }
  } else if value.kind() == &list_kind(scope.clone()) {
    let mut list = new_list_from(
      scope.clone(),
      value
        .downcast::<Object<List>>()
        .expect("failed to downcast value to List")
        .value()
        .clone(),
    );

    if let Some(value) = list.pop_front() {
      stack.state.push_front(EvalState::Call);
      stack.value.push_front(list.into_value());

      stack.state.push_front(EvalState::Eval);
      stack.value.push_front(value);
    } else {
      stack.value.push_front(list.into_value());
    }
  } else if value.kind() == &vector_kind(scope.clone()) {
    let mut vec = new_vector_from(
      scope.clone(),
      value
        .downcast::<Object<Vector>>()
        .expect("failed to downcast value to Vec")
        .value()
        .clone(),
    );

    if let Some(value) = vec.pop() {
      stack.state.push_front(EvalState::EvalVec);

      stack.value.push_front(vec.into_value());
      stack
        .value
        .push_front(new_vector(scope.clone()).into_value());

      stack.state.push_front(EvalState::Eval);
      stack.value.push_front(value);
    } else {
      stack
        .value
        .push_front(new_vector(scope.clone()).into_value());
    }
  } else if value.kind() == &map_kind(scope.clone()) {
    let mut key_values = new_linked_map(
      scope.clone(),
      value
        .downcast::<Object<Map>>()
        .expect("failed to downcast value to Map")
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<LinkedList<(Gc<dyn Value>, Gc<dyn Value>)>>(),
    );

    if let Some((key, value)) = key_values.pop_back() {
      stack.state.push_front(EvalState::EvalMap);

      stack.value.push_front(key_values.into_value());
      stack.value.push_front(new_map(scope.clone()).into_value());

      stack.state.push_front(EvalState::EvalMapKeyValue);
      stack.state.push_front(EvalState::Eval);

      stack.value.push_front(key);
      stack.value.push_front(value);
    } else {
      stack.value.push_front(new_map(scope.clone()).into_value());
    }
  } else {
    stack.value.push_front(value);
  }
}

#[inline]
fn eval_eval_evaluated(stack: &mut Stack) {
  let value = stack.value.pop_front().expect("failed to get value");
  let scope = stack.scope.front().unwrap();

  if value.kind() == &list_kind(scope.clone()) {
    let mut list = new_list_from(
      scope.clone(),
      value
        .downcast::<Object<List>>()
        .expect("failed to downcast value to List")
        .value()
        .clone(),
    );

    if let Some(value) = list.pop_front() {
      stack.state.push_front(EvalState::Call);
      stack.value.push_front(list.into_value());

      stack.state.push_front(EvalState::Eval);
      stack.value.push_front(value);
    } else {
      stack.value.push_front(list.into_value());
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
    .downcast::<Object<Vector>>()
    .expect("failed to downcast evaluated vec to vec");
  let mut key_values = stack
    .value
    .pop_front()
    .expect("failed to get vec from stack")
    .downcast::<Object<Vector>>()
    .expect("failed to downcast vec to Vector");

  evaluated_vector.push(evaluated_value);

  if let Some(value) = key_values.pop() {
    stack.state.push_front(EvalState::EvalVec);

    stack.value.push_front(key_values.into_value());
    stack.value.push_front(evaluated_vector.into_value());

    stack.state.push_front(EvalState::Eval);
    stack.value.push_front(value);
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
    .downcast::<Object<Map>>()
    .expect("failed to downcast evaluated map to vec");
  let mut key_values = stack
    .value
    .pop_front()
    .expect("failed to get map from stack")
    .downcast::<Object<LinkedMap>>()
    .expect("failed to downcast map to Vec of key values");

  evaluated_map.set(evaluated_key, evaluated_value);

  if let Some((key, value)) = key_values.pop_front() {
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
    .expect("failed to get callable value");
  let mut arguments = stack
    .value
    .pop_front()
    .expect("failed to get arguments from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast arguments to List");
  let scope = stack.scope.front().unwrap();

  if callable.kind() == &function_kind(scope.clone()) {
    stack.state.push_front(EvalState::PopScope);
    stack.state.push_front(EvalState::CallFunction);

    stack.value.push_front(callable);

    if let Some(value) = arguments.pop_back() {
      stack.state.push_front(EvalState::CallFunctionEvalArgs);

      stack.value.push_front(arguments.into_value());
      stack.value.push_front(new_list(scope.clone()).into_value());

      stack.state.push_front(EvalState::Eval);
      stack.value.push_front(value);
    } else {
      stack.value.push_front(arguments.into_value());
    }
  } else if callable.kind() == &macro_kind(scope.clone()) {
    stack.state.push_front(EvalState::Eval);
    stack.state.push_front(EvalState::PopScope);
    stack.state.push_front(EvalState::CallFunction);

    stack.value.push_front(callable);
    stack.value.push_front(arguments.into_value());
  } else if callable.kind() == &special_form_kind(scope.clone()) {
    let special_form = callable
      .downcast::<Object<SpecialForm>>()
      .expect("failed downcast value to SpecialForm");
    stack.value.push_front(arguments.into_value());
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
    .expect("failed to get callable value");
  let arguments = stack
    .value
    .pop_front()
    .expect("failed to get arguments from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast arguments to List");
  let scope = stack.scope.front().unwrap();

  if callable.kind() == &function_kind(scope.clone()) {
    stack.state.push_front(EvalState::PopScope);
    stack.state.push_front(EvalState::CallFunction);

    stack.value.push_front(callable);
    stack.value.push_front(arguments.into_value());
  } else if callable.kind() == &macro_kind(scope.clone()) {
    stack.state.push_front(EvalState::Eval);
    stack.state.push_front(EvalState::PopScope);
    stack.state.push_front(EvalState::CallFunction);

    stack.value.push_front(callable);
    stack.value.push_front(arguments.into_value());
  } else if callable.kind() == &special_form_kind(scope.clone()) {
    let special_form = callable
      .downcast::<Object<SpecialForm>>()
      .expect("failed downcast value to SpecialForm");
    stack.value.push_front(arguments.into_value());
    (special_form.value().deref())(stack);
  } else {
    panic!("Failed to call non-callable value {:?}", callable);
  }
}

#[inline]
fn eval_call_function_eval_arguments(stack: &mut Stack) {
  let evaluated_arg = stack
    .value
    .pop_front()
    .expect("failed to get argument from stack");
  let mut evaluated_arguments = stack
    .value
    .pop_front()
    .expect("failed to get evaluated arguments from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast evaluated arguments to List");
  let mut arguments = stack
    .value
    .pop_front()
    .expect("failed to get arguments from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast arguments to List");

  evaluated_arguments.push_front(evaluated_arg);

  if let Some(value) = arguments.pop_back() {
    stack.state.push_front(EvalState::CallFunctionEvalArgs);

    stack.value.push_front(arguments.into_value());
    stack.value.push_front(evaluated_arguments.into_value());

    stack.value.push_front(value);
    stack.state.push_front(EvalState::Eval);
  } else {
    stack.value.push_front(evaluated_arguments.into_value());
  }
}

#[inline]
fn eval_call_function(stack: &mut Stack) {
  let arguments = stack
    .value
    .pop_front()
    .expect("failed to get arguments from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast arguments to List");
  let callable = stack
    .value
    .pop_front()
    .expect("failed to get callable from stack")
    .downcast::<Object<Function>>()
    .expect("failed to downcast callable to Function");
  let mut scope = new_scope(callable.scope().clone());

  scope.add("arguments", arguments.clone().into_value());

  if let Some(name) = callable.value().name() {
    scope.add(name.value().deref(), callable.clone().into_value());
  }

  let values = arguments
    .iter()
    .map(Clone::clone)
    .collect::<Vec<Gc<dyn Value>>>();
  let mut index = 0;
  let nil = nil_value(scope.clone()).into_value();

  for param in callable.value().params().value() {
    if let Some(key) = param.downcast_ref::<Object<Symbol>>() {
      scope.add(
        key.value().deref(),
        values.get(index).unwrap_or(&nil).clone(),
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
      let value = (&**body)(scope.clone(), arguments);
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
    .unwrap_or_else(|| nil_value(scope.clone()).into_value());

  let mut error = new_map(scope.clone());
  let mut stack_trace = new_vector(scope.clone());

  error.set(
    new_keyword(scope.clone(), "value").into_value(),
    error_value,
  );
  error.set(
    new_keyword(scope.clone(), "stack_trace").into_value(),
    stack_trace.clone().into_value(),
  );

  loop {
    match stack.pop() {
      PopResult::Callable(callable) => {
        stack_trace.push(
          callable
            .name()
            .map(Clone::clone)
            .unwrap_or_else(|| new_symbol(scope.clone(), "anonymous"))
            .into_value(),
        );
      }
      PopResult::Caught(value) => {
        stack.state.push_front(EvalState::Catch);
        stack.state.push_front(EvalState::Eval);
        stack.value.push_front(value);
      }
      PopResult::Uncaught => {
        break;
      }
    }
  }

  panic!("Uncaught Error: {:?}", error);
}

#[inline]
fn eval_catch(stack: &mut Stack) {
  let error = stack.value.pop_front();
  panic!("{:?}", error);
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
    .downcast::<Object<bool>>()
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
    .downcast::<Object<Symbol>>()
    .expect("failed to downcast name to Symbol");

  stack
    .scope
    .front_mut()
    .expect("failed to get scope")
    .set(name.value().deref(), value);
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
    .expect("failed to get evaluated vec from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast evaluated vec to vec");
  let mut list = stack
    .value
    .pop_front()
    .expect("failed to get vec from stack")
    .downcast::<Object<List>>()
    .expect("failed to downcast vec to Vec");
  let scope = stack.scope.front().unwrap();

  evaluated_list.push_back(evaluated_value);

  if let Some(value) = list.pop_front() {
    stack.state.push_front(EvalState::Expand);

    stack.value.push_front(list.into_value());
    stack.value.push_front(evaluated_list.into_value());

    if value.kind() == &escape_kind(scope.clone()) {
      let escape = value
        .downcast::<Object<Escape>>()
        .expect("failed to downcast expand value to Escape");

      stack.state.push_front(EvalState::Eval);
      stack.value.push_front(escape.escape_value().clone());
    } else if value.kind() == &list_kind(scope.clone()) {
      stack.value.push_front(
        new_list_from(
          scope.clone(),
          value
            .downcast::<Object<List>>()
            .expect("failed to downcast expand value to List")
            .value()
            .clone(),
        )
        .into_value(),
      );
      expand_special_form(stack);
    } else {
      stack.value.push_front(value);
    }
  } else {
    stack.value.push_front(evaluated_list.into_value());
  }
}
