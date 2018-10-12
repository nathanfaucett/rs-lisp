#![feature(arbitrary_self_types)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(get_type_id)]

extern crate num_traits;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate lang_gc as gc;

mod context;
mod eval;
mod function;
mod kind;
mod number;
mod object;
mod scope;
mod value;

pub use self::context::{Context, List, Map, Vector};
pub use self::eval::eval;
pub use self::function::Function;
pub use self::kind::Kind;
pub use self::number::Number;
pub use self::object::Object;
pub use self::scope::Scope;
pub use self::value::Value;
