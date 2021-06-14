use alloc::collections::LinkedList;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::{fmt, ptr};

use gc::{Gc, Trace};

use super::{
    new_kind, new_object, scope_get, scope_get_with_kind, scope_set, Function, Kind, Object, Scope,
    Value,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EvalState {
    Eval,
    EvalVec,
    EvalMap,
    EvalMapKeyValue,
    Call,
    CallFunction,
    PopValue,
    PopScope,
    Throw,
    Catch,
    If,
    Def,
    Expand,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnwindResult {
    Caught(Gc<dyn Value>),
    Callable(Gc<Object<Function>>),
    Uncaught,
}

#[derive(Clone, Eq)]
pub struct Stack {
    pub(crate) value: LinkedList<Gc<dyn Value>>,
    pub(crate) scope: LinkedList<Gc<Object<Scope>>>,
    pub(crate) callable: LinkedList<Gc<Object<Function>>>,
    pub(crate) state: LinkedList<EvalState>,
}

impl fmt::Debug for Stack {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("").field(&"Stack").finish()
    }
}

impl Trace for Stack {
    #[inline]
    fn trace(&mut self, marked: bool) {
        for v in self.value.iter_mut() {
            v.trace(marked);
        }
        for v in self.scope.iter_mut() {
            v.trace(marked);
        }
        for v in self.callable.iter_mut() {
            v.trace(marked);
        }
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
            callable: LinkedList::new(),
            state: LinkedList::new(),
        }
    }

    #[inline]
    pub(crate) fn unwind(&mut self) -> UnwindResult {
        loop {
            match self.state.pop_front() {
                Some(EvalState::Catch) => {
                    return UnwindResult::Caught(
                        self.value
                            .pop_front()
                            .expect("no function was passed to caught block"),
                    );
                }
                Some(EvalState::PopScope) => {
                    let callable = self
                        .callable
                        .pop_front()
                        .expect("no function was in the stack")
                        .downcast::<Object<Function>>()
                        .expect("failed to downcast callable to function");
                    return UnwindResult::Callable(callable);
                }
                None => return UnwindResult::Uncaught,
                _ => {}
            }
        }
    }

    #[inline]
    pub(crate) fn pop_scope_and_value(&mut self) -> Option<(Gc<Object<Scope>>, Gc<dyn Value>)> {
        self.scope
            .pop_front()
            .and_then(|scope| self.value.pop_front().map(|value| (scope, value)))
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

    #[inline]
    pub(crate) fn init_kind(scope: &Gc<Object<Scope>>) {
        let stack_kind = new_kind::<Stack>(scope, "Stack");
        scope_set(scope, "Stack", stack_kind.into_value());
    }

    #[inline]
    pub fn init_scope(scope: &Gc<Object<Scope>>) {
        let stack = new_object(scope, Object::new(stack_kind(scope).clone(), Stack::new()));
        scope_set(&scope, "__stack", stack.into_value());
    }
}

#[inline]
pub fn stack_kind(scope: &Gc<Object<Scope>>) -> Gc<Object<Kind>> {
    scope_get_with_kind::<Kind>(scope, "Stack").expect("failed to get Stack Kind")
}

pub fn get_stack(scope: &Gc<Object<Scope>>) -> Gc<Object<Stack>> {
    scope_get(scope, "__stack")
        .unwrap()
        .downcast_ref::<Object<Stack>>()
        .expect("failed to downcast __stack to Stack Object")
        .clone()
}
