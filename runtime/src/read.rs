use gc::Gc;

use super::{new_list, new_string, new_symbol, List, Object, Scope, Value};

struct Lexer {
    index: usize,
    chars: Vec<char>,
}

impl Lexer {
    #[inline]
    fn new(chars: Vec<char>) -> Self {
        Lexer {
            index: 0,
            chars: chars,
        }
    }
    #[inline]
    fn next(&mut self) -> Option<char> {
        let index = self.index;
        self.index += 1;
        self.chars.get(index).map(Clone::clone)
    }
    #[inline]
    fn peak(&self) -> Option<char> {
        self.chars.get(self.index + 1).map(Clone::clone)
    }
}

#[inline]
pub fn read<T>(scope: &Gc<Object<Scope>>, value: T) -> Gc<Value>
where
    T: ToString,
{
    let mut lexer = Lexer::new(value.to_string().chars().collect());
    read_value(&scope, &mut lexer)
}

#[inline]
pub fn read_internal(scope: Gc<Object<Scope>>, args: Gc<Object<List>>) -> Gc<Value> {
    let string = args
        .front()
        .expect("read requires one arguments")
        .downcast_ref::<Object<String>>()
        .expect("Invalid type passed to read expected String");
    read(&scope, string.value())
}

#[inline]
fn read_value(scope: &Gc<Object<Scope>>, lexer: &mut Lexer) -> Gc<Value> {
    while let Some(ch) = lexer.next() {
        if is_whitespace(ch) {
            continue;
        }

        match ch {
            '(' => return read_list(scope, lexer),
            '"' => return read_string(scope, lexer),
            ch => {
                if ch.is_alphabetic() {
                    return read_symbol(scope, lexer);
                }
            }
        }
    }

    unimplemented!()
}

#[inline]
fn read_list(scope: &Gc<Object<Scope>>, lexer: &mut Lexer) -> Gc<Value> {
    let mut list = new_list(&scope);

    while let Some(ch) = lexer.peak() {
        if ch == ')' {
            break;
        } else {
            lexer.next();
            list.push_back(read_value(scope, lexer));
        }
    }

    list.into_value()
}

#[inline]
fn read_symbol(scope: &Gc<Object<Scope>>, lexer: &mut Lexer) -> Gc<Value> {
    let mut string = String::new();

    while let Some(ch) = lexer.peak() {
        if ch.is_alphanumeric() {
            lexer.next();
            string.push(ch);
        } else {
            break;
        }
    }

    new_symbol(scope, string).into_value()
}

#[inline]
fn read_string(scope: &Gc<Object<Scope>>, lexer: &mut Lexer) -> Gc<Value> {
    let mut string = String::new();

    while let Some(ch) = lexer.peak() {
        if ch == '"' {
            break;
        } else {
            lexer.next();
            string.push(ch);
        }
    }

    new_string(scope, string).into_value()
}

#[inline]
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() || ch == ','
}
