use alloc::collections::LinkedList;

use gc::{Gc, Trace};

use super::{Object, Scope, Value};

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
  pub fn push_scope_value(&mut self, scope: Gc<Object<Scope>>, value: Gc<dyn Value>) -> &mut Self {
    self.value.push_front(value);
    self.scope.push_front(scope);
    self.state.push_front(EvalState::Eval);
    self
  }
}
