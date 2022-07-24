#![no_std]

extern crate alloc;

mod gc;
mod trace;

pub use self::gc::Gc;
pub use self::trace::Trace;
