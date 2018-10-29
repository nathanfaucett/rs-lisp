extern crate lisp;

use lisp::runtime::{eval, read, Context};

fn main() {
    let context = Context::new();

    let raw = "
        (do
            (def test (fn (a) (if a true false)))
            (def result (test true))
            result
        ))
        ";
    let input = read(context.scope(), raw);

    println!("{:?}", input);
    let output = eval(context.scope().clone(), input);
    println!("{:?}", output);
}
