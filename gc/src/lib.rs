#![feature(alloc)]
#![feature(const_fn)]
#![feature(get_type_id)]
#![no_std]

extern crate alloc;

mod gc;

pub use self::gc::Gc;
