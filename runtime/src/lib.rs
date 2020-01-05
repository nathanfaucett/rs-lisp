//#![no_std]
#![feature(arbitrary_self_types)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

#[macro_use]
extern crate alloc;
extern crate hashbrown;
extern crate num_traits;
extern crate serde;
extern crate serde_derive;

extern crate lisp_gc as gc;

extern crate core;

mod boolean;
mod context;
mod escape;
mod eval;
mod function;
mod function_kind;
mod gc_allocator;
mod keyword;
mod kind;
mod linked_map;
mod lisp_map;
mod list;
mod map;
mod numbers;
mod object;
mod persistent_list;
mod persistent_map;
mod persistent_scope;
mod persistent_set;
mod persistent_vector;
mod reader;
mod set;
mod special_form;
mod stack;
mod symbol;
mod value;
mod vector;

pub use self::boolean::*;
pub use self::context::*;
pub use self::escape::*;
pub use self::eval::*;
pub use self::function::*;
pub use self::function_kind::*;
pub use self::gc_allocator::*;
pub use self::keyword::*;
pub use self::kind::*;
pub use self::linked_map::*;
pub use self::lisp_map::*;
pub use self::list::*;
pub use self::map::*;
pub use self::numbers::*;
pub use self::object::*;
pub use self::persistent_list::*;
pub use self::persistent_map::*;
pub use self::persistent_scope::*;
pub use self::persistent_set::*;
pub use self::persistent_vector::*;
pub use self::reader::*;
pub use self::set::*;
pub use self::special_form::*;
pub use self::stack::*;
pub use self::symbol::*;
pub use self::value::*;
pub use self::vector::*;
