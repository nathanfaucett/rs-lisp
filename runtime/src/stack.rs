use alloc::collections::LinkedList;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::{fmt, ptr};

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

#[derive(Clone, Eq)]
pub struct Stack {
  pub(crate) value: LinkedList<Gc<dyn Value>>,
  pub(crate) scope: LinkedList<Gc<Object<Scope>>>,
  pub(crate) state: LinkedList<EvalState>,
}

impl Trace for Stack {}

impl fmt::Debug for Stack {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(":stack")
  }
}

impl PartialEq for Stack {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value) && self.scope.eq(&other.scope) && self.state.eq(&other.state)
  }
}

impl PartialOrd for Stack {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl Hash for Stack {
  #[inline(always)]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
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
  pub(crate) fn push_scope_and_value(
    &mut self,
    scope: Gc<Object<Scope>>,
    value: Gc<dyn Value>,
  ) -> &mut Self {
    self.value.push_front(value);
    self.scope.push_front(scope);
    self.state.push_front(EvalState::Eval);
    self
  }
}
