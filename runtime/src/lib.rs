#![feature(arbitrary_self_types)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(get_type_id)]

extern crate num_traits;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate fnv;

extern crate lisp_gc as gc;

pub mod context;
mod eval;
mod function;
mod function_kind;
mod kind;
mod list;
mod map;
mod object;
mod read;
mod scope;
mod special_form;
mod symbol;
mod value;
mod vector;

pub use self::context::*;
pub use self::eval::*;
pub use self::function::Function;
pub use self::function_kind::FunctionKind;
pub use self::kind::Kind;
pub use self::list::List;
pub use self::map::Map;
pub use self::object::Object;
pub use self::read::*;
pub use self::scope::Scope;
pub use self::special_form::*;
pub use self::symbol::Symbol;
pub use self::value::Value;
pub use self::vector::Vector;
