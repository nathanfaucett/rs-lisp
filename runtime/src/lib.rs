#![feature(arbitrary_self_types)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(get_type_id)]

extern crate num_traits;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate fnv;

extern crate lang_gc as gc;

#[macro_use]
mod lisp_macro;

pub mod context;
mod eval;
mod function;
mod function_kind;
mod kind;
mod number;
mod object;
mod scope;
mod special_form;
mod value;

pub use self::context::*;
pub use self::eval::*;
pub use self::function::Function;
pub use self::function_kind::FunctionKind;
pub use self::kind::Kind;
pub use self::number::Number;
pub use self::object::Object;
pub use self::scope::Scope;
pub use self::special_form::*;
pub use self::value::Value;
