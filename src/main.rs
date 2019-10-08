#[macro_use]
extern crate clap;
extern crate lisp;

use std::fs::canonicalize;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
  let matches = clap_app!(app =>
      (name: NAME)
      (version: VERSION)
      (author: AUTHORS)
      (about: DESCRIPTION)
      (@arg INPUT: +required "Sets the input file to use")
  )
  .get_matches();

  let lisp = lisp::Lisp::new();
  lisp.run(
    &canonicalize(matches.value_of("INPUT").expect("No input file given"))
      .expect("Failed to canonicalize input file"),
  );
}
