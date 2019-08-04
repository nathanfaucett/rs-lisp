extern crate lisp;

use lisp::runtime;

fn main() {
    let scope = runtime::new();

    let raw = concat!("(do ", include_str!("simple.lisp"), ")");

    let output = runtime::run(&scope, raw);
    println!("{:?}", output);
}
