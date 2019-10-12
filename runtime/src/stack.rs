use alloc::collections::LinkedList;

use gc::{Gc, Trace};

use super::{new_kind, new_object, Kind, Object, Scope, Value};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct Stack {
  pub value: LinkedList<Gc<dyn Value>>,
  pub scope: LinkedList<Gc<Object<Scope>>>,
  pub state: LinkedList<EvalState>,
}

impl Trace for Stack {}

#[inline]
fn stack_value_eq(list_a: &LinkedList<Gc<dyn Value>>, list_b: &LinkedList<Gc<dyn Value>>) -> bool {
  if list_a.len() != list_b.len() {
    return false;
  }
  for a in list_a.iter() {
    for b in list_b.iter() {
      if a.equal(b.as_ref()) {
        return false;
      }
    }
  }
  true
}

impl PartialEq for Stack {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    stack_value_eq(&self.value, &other.value)
      && self.scope.eq(&other.scope)
      && self.state.eq(&other.state)
  }
}

impl Stack {
  #[inline]
  pub fn new() -> Self {
    Stack {
      value: LinkedList::new(),
      scope: LinkedList::new(),
      state: LinkedList::new(),
    }
  }

  #[inline]
  pub fn init(&mut self, scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> &mut Self {
    self.value.push_front(value);
    self.scope.push_front(scope);
    self.state.push_front(EvalState::Eval);
    self
  }

  #[inline]
  pub(crate) unsafe fn init_kind(mut scope: Gc<Object<Scope>>) {
    let stack_kind = new_kind::<Stack>(scope.clone(), "Stack");
    scope.set("Stack", stack_kind.into_value());
  }

  #[inline]
  pub(crate) fn init_scope(mut scope: Gc<Object<Scope>>) {
    let stack = new_stack(scope.clone());
    scope.set("__stack__", stack.into_value());
  }
}

#[inline]
pub fn stack_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Stack")
      .expect("failed to get Stack Kind")
  }
}

#[inline]
pub fn new_stack(scope: Gc<Object<Scope>>) -> Gc<Object<Stack>> {
  new_object(
    scope.clone(),
    Object::new(stack_kind(scope.clone()), Stack::new()),
  )
}
