#[macro_use]
extern crate lisp;

use lisp::runtime::{eval, Context};

fn main() {
    let context = Context::new();

    let input = lisp!(context.scope(), (do
            (def test (fn (a) (if a true false)))
            (def result (test true))
            result
        ))
    .into_value();

    let output = eval(context.scope().clone(), input);
    println!("{:?}", output);
}
