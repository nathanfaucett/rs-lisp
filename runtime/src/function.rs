use alloc::string::ToString;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::{Gc, Trace};

use super::{
  eval, new_kind, new_object, new_persistent_list_from, new_persistent_vector_from, new_symbol,
  scope_get_with_kind, scope_set, FunctionKind, Kind, Object, PersistentList, PersistentScope,
  PersistentVector, Symbol, Value,
};

#[derive(Eq)]
pub struct Function {
  name: Option<Gc<Object<Symbol>>>,
  scope: Gc<Object<PersistentScope>>,
  params: Gc<Object<PersistentVector>>,
  body: FunctionKind,
}

impl Trace for Function {
  #[inline]
  fn trace(&mut self, marked: bool) {
    self.name.trace(marked);
    self.scope.trace(marked);
    self.params.trace(marked);
    self.body.trace(marked);
  }
}

impl Hash for Function {
  #[inline(always)]
  fn hash<H: Hasher>(&self, state: &mut H) {
    ptr::hash(self, state)
  }
}

impl PartialOrd for Function {
  #[inline]
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    None
  }
}

impl PartialEq for Function {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.name.eq(&other.name) && self.body.eq(&other.body)
  }
}

impl fmt::Debug for Function {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut debug = f.debug_tuple("");

    debug.field(&"fn");

    if let Some(name) = self.name.as_ref() {
      debug.field(name);
    }

    debug.field(&self.params).field(&self.body).finish()
  }
}

impl Function {
  #[inline(always)]
  pub fn new(
    name: Option<Gc<Object<Symbol>>>,
    scope: Gc<Object<PersistentScope>>,
    params: Gc<Object<PersistentVector>>,
    body: Gc<dyn Value>,
  ) -> Self {
    Function {
      name: name,
      scope: scope,
      params: params,
      body: FunctionKind::new_internal(body),
    }
  }

  #[inline(always)]
  pub fn new_external<F>(
    name: Option<Gc<Object<Symbol>>>,
    scope: Gc<Object<PersistentScope>>,
    params: Gc<Object<PersistentVector>>,
    body: F,
  ) -> Self
  where
    F: 'static + Fn(&Gc<Object<PersistentScope>>, &Gc<Object<PersistentVector>>) -> Gc<dyn Value>,
  {
    Function {
      name: name,
      scope: scope,
      params: params,
      body: FunctionKind::new_external(body),
    }
  }

  #[inline(always)]
  pub fn name(&self) -> Option<&Gc<Object<Symbol>>> {
    self.name.as_ref()
  }
  #[inline(always)]
  pub fn scope(&self) -> &Gc<Object<PersistentScope>> {
    &self.scope
  }
  #[inline(always)]
  pub fn params(&self) -> &Gc<Object<PersistentVector>> {
    &self.params
  }
  #[inline(always)]
  pub fn body(&self) -> &FunctionKind {
    &self.body
  }

  #[inline]
  pub(crate) fn init_kind(scope: &Gc<Object<PersistentScope>>) -> Gc<Object<PersistentScope>> {
    let function_kind = new_kind::<Function>(scope, "Function");
    let macro_kind = new_kind::<Function>(scope, "Macro");

    let new_scope = scope_set(scope, "Function", function_kind.into_value());
    scope_set(&new_scope, "Macro", macro_kind.into_value())
  }
}

#[inline]
pub fn function_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Function").expect("failed to get Function Kind")
}
#[inline]
pub fn new_function(
  scope: &Gc<Object<PersistentScope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<PersistentVector>>,
  body: Gc<dyn Value>,
) -> Gc<Object<Function>> {
  new_object(
    scope,
    Object::new(
      function_kind(scope).clone(),
      Function::new(name, scope.clone(), params, body),
    ),
  )
}
#[inline]
pub fn new_external_function<F>(
  scope: &Gc<Object<PersistentScope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<PersistentVector>>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(&Gc<Object<PersistentScope>>, &Gc<Object<PersistentVector>>) -> Gc<dyn Value>,
{
  new_object(
    scope,
    Object::new(
      function_kind(scope).clone(),
      Function::new_external(name, scope.clone(), params, body),
    ),
  )
}

#[inline]
pub fn add_external_function<F, N>(
  scope: &Gc<Object<PersistentScope>>,
  name: N,
  params: ::alloc::vec::Vec<N>,
  body: F,
) -> Gc<Object<PersistentScope>>
where
  F: 'static + Fn(&Gc<Object<PersistentScope>>, &Gc<Object<PersistentVector>>) -> Gc<dyn Value>,
  N: ToString,
{
  let mut persistent_vector = PersistentVector::new();

  for param in params {
    persistent_vector = persistent_vector.push(new_symbol(scope, param).into_value());
  }

  let function = new_external_function(
    scope,
    Some(new_symbol(scope, name.to_string())),
    new_persistent_vector_from(scope, persistent_vector),
    body,
  );
  scope_set(scope, &(name.to_string()), function.clone().into_value())
}

#[inline]
pub fn macro_kind(scope: &Gc<Object<PersistentScope>>) -> &Gc<Object<Kind>> {
  scope_get_with_kind::<Kind>(scope, "Macro").expect("failed to get Macro Kind")
}
#[inline]
pub fn new_macro(
  scope: &Gc<Object<PersistentScope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<PersistentVector>>,
  body: Gc<dyn Value>,
) -> Gc<Object<Function>> {
  new_object(
    scope,
    Object::new(
      macro_kind(scope).clone(),
      Function::new(name, scope.clone(), params, body),
    ),
  )
}
#[inline]
pub fn new_external_macro<F>(
  scope: &Gc<Object<PersistentScope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<PersistentVector>>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(&Gc<Object<PersistentScope>>, &Gc<Object<PersistentVector>>) -> Gc<dyn Value>,
{
  new_object(
    scope,
    Object::new(
      macro_kind(scope).clone(),
      Function::new_external(name, scope.clone(), params, body),
    ),
  )
}

#[inline]
pub fn add_external_macro<F, N>(
  scope: &Gc<Object<PersistentScope>>,
  name: N,
  params: ::alloc::vec::Vec<N>,
  body: F,
) -> Gc<Object<PersistentScope>>
where
  F: 'static + Fn(&Gc<Object<PersistentScope>>, &Gc<Object<PersistentVector>>) -> Gc<dyn Value>,
  N: ToString,
{
  let mut persistent_vector = PersistentVector::new();

  for param in params {
    persistent_vector = persistent_vector.push(new_symbol(scope, param).into_value());
  }

  let function = new_external_macro(
    scope,
    Some(new_symbol(scope, name.to_string())),
    new_persistent_vector_from(scope, persistent_vector),
    body,
  );
  scope_set(scope, &(name.to_string()), function.clone().into_value())
}

#[inline]
pub fn call_function(
  scope: &Gc<Object<PersistentScope>>,
  callable: Gc<Object<Function>>,
  arguments: Gc<Object<PersistentVector>>,
) -> (Gc<Object<PersistentScope>>, Gc<dyn Value>) {
  let mut persistent_list = arguments.value().iter().collect::<PersistentList>();
  persistent_list = persistent_list.push_front(callable.into_value());
  eval(
    scope,
    new_persistent_list_from(scope, persistent_list).into_value(),
  )
}
