use alloc::string::ToString;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ptr;

use gc::{Gc, Trace};

use super::{
  eval, new_kind, new_list, new_list_from, new_object, new_symbol, FunctionKind, Kind, List,
  Object, Scope, Symbol, Value,
};

#[derive(Eq)]
pub struct Function {
  name: Option<Gc<Object<Symbol>>>,
  scope: Gc<Object<Scope>>,
  params: Gc<Object<List>>,
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
    scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
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
    scope: Gc<Object<Scope>>,
    params: Gc<Object<List>>,
    body: F,
  ) -> Self
  where
    F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
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
  pub fn scope(&self) -> &Gc<Object<Scope>> {
    &self.scope
  }
  #[inline(always)]
  pub fn params(&self) -> &Gc<Object<List>> {
    &self.params
  }
  #[inline(always)]
  pub fn body(&self) -> &FunctionKind {
    &self.body
  }

  #[inline]
  pub(crate) unsafe fn init_kind(mut scope: Gc<Object<Scope>>) {
    let function_kind = new_kind::<Function>(scope.clone(), "Function");
    let macro_kind = new_kind::<Function>(scope.clone(), "Macro");

    scope.set("Function", function_kind.into_value());
    scope.set("Macro", macro_kind.into_value());
  }
}

#[inline]
pub fn function_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Function")
      .expect("failed to get Function Kind")
  }
}
#[inline]
pub fn new_function(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: Gc<dyn Value>,
) -> Gc<Object<Function>> {
  new_object(
    scope.clone(),
    Object::new(
      function_kind(scope.clone()),
      Function::new(name, scope, params, body),
    ),
  )
}
#[inline]
pub fn new_external_function<F>(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
{
  new_object(
    scope.clone(),
    Object::new(
      function_kind(scope.clone()),
      Function::new_external(name, scope, params, body),
    ),
  )
}

#[inline]
pub fn add_external_function<F, N>(
  mut scope: Gc<Object<Scope>>,
  name: N,
  params: ::alloc::vec::Vec<N>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
  N: ToString,
{
  let mut list = new_list(scope.clone());

  for param in params {
    list.push_back(new_symbol(scope.clone(), param).into_value());
  }

  let function = new_external_function(
    scope.clone(),
    Some(new_symbol(scope.clone(), name.to_string())),
    list,
    body,
  );
  scope.set(&(name.to_string()), function.clone().into_value());
  function
}

#[inline]
pub fn macro_kind(scope: Gc<Object<Scope>>) -> Gc<Object<Kind>> {
  unsafe {
    scope
      .get_with_kind::<Kind>("Macro")
      .expect("failed to get Macro Kind")
  }
}
#[inline]
pub fn new_macro(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: Gc<dyn Value>,
) -> Gc<Object<Function>> {
  new_object(
    scope.clone(),
    Object::new(
      macro_kind(scope.clone()),
      Function::new(name, scope.clone(), params, body),
    ),
  )
}
#[inline]
pub fn new_external_macro<F>(
  scope: Gc<Object<Scope>>,
  name: Option<Gc<Object<Symbol>>>,
  params: Gc<Object<List>>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
{
  new_object(
    scope.clone(),
    Object::new(
      macro_kind(scope.clone()),
      Function::new_external(name, scope, params, body),
    ),
  )
}

#[inline]
pub fn add_external_macro<F, N>(
  mut scope: Gc<Object<Scope>>,
  name: N,
  params: ::alloc::vec::Vec<N>,
  body: F,
) -> Gc<Object<Function>>
where
  F: 'static + Fn(Gc<Object<Scope>>, Gc<Object<List>>) -> Gc<dyn Value>,
  N: ToString,
{
  let mut list = new_list(scope.clone());

  for param in params {
    list.push_back(new_symbol(scope.clone(), param).into_value());
  }

  let function = new_external_macro(
    scope.clone(),
    Some(new_symbol(scope.clone(), name.to_string())),
    list,
    body,
  );
  scope.set(&(name.to_string()), function.clone().into_value());
  function
}

#[inline]
pub fn call_function(
  scope: Gc<Object<Scope>>,
  callable: Gc<Object<Function>>,
  arguments: Gc<Object<List>>,
) -> Gc<dyn Value> {
  let mut function_call = new_list_from(scope.clone(), arguments.value().clone());
  function_call.push_front(callable.into_value());
  eval(scope, function_call.into_value())
}
