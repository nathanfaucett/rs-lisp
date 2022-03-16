use std::{fmt::Write, io, path::Path};

use rustyline::error::ReadlineError;
use rustyline::Editor;

use gc::Gc;
use runtime::{
    add_external_function, new_context, new_string, nil_value, Object, Scope, Value, Vector,
};

use super::{loader, new_module, DyLib};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn new() -> Gc<Object<Scope>> {
    let scope = new_context();

    DyLib::init_kind(&scope);
    DyLib::init_scope(&scope);

    add_external_function(&scope, "println", vec!["...args"], println);

    scope
}

#[inline]
pub fn run_path(scope: &Gc<Object<Scope>>, filename_path: &Path) -> io::Result<()> {
    let mut module = new_module(scope, None);
    module.set(
        new_string(scope, "dirname").into_value(),
        new_string(scope, ".").into_value(),
    );
    loader::load(
        scope,
        module,
        new_string(
            scope,
            filename_path
                .to_str()
                .expect("failed to move Path to string"),
        ),
    )
    .expect(&format!("failed to load module {:?}", filename_path));
    Ok(())
}

#[inline]
pub fn repl(scope: &Gc<Object<Scope>>) -> io::Result<()> {
    let mut rl = Editor::<()>::new();
    println!("Welcome to {} v{}", NAME, VERSION);
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let value = runtime::run_in_scope(scope, &line);
                if value != nil_value(scope).into_value() {
                    println!("{:?}", value);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    Ok(())
}

#[inline]
fn println(scope: &Gc<Object<Scope>>, args: &Gc<Object<Vector>>) -> Gc<dyn Value> {
    let mut string = String::new();
    let mut index = args.value().len();

    for value in args.value() {
        write!(string, "{:?}", value).unwrap();

        index -= 1;
        if index != 0 {
            write!(string, ", ").unwrap();
        }
    }

    println!("{}", string);
    nil_value(scope).clone().into_value()
}
