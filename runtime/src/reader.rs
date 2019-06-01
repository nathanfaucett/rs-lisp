use core::num::ParseIntError;
use core::str::FromStr;
use alloc::vec;
use alloc::string::String;

use gc::Gc;

use super::{
    new_char, new_escape, new_i16, new_i32, new_i64, new_i8, new_isize, new_keyword, new_list,
    new_map, new_string, new_symbol, new_u16, new_u32, new_u64, new_u8, new_usize, new_vec,
    nil_value, Escape, Keyword, List, Map, Object, Scope, Symbol, Value, Vec,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reader {
    index: usize,
    chars: vec::Vec<char>,
}

impl Reader {
    #[inline]
    pub fn new(chars: vec::Vec<char>) -> Self {
        Reader {
            index: 0,
            chars: chars,
        }
    }
    #[inline]
    fn consume(&mut self) -> &mut Self {
        self.index += 1;
        self
    }
    #[inline]
    fn next(&mut self) -> Option<char> {
        let index = self.index;
        self.consume();
        self.chars.get(index).map(Clone::clone)
    }
    #[inline]
    fn peek(&self) -> Option<char> {
        self.peek_nth(0)
    }
    #[inline]
    fn peek_nth(&self, index: usize) -> Option<char> {
        self.chars.get(self.index + index).map(Clone::clone)
    }
}

#[inline]
pub fn read_value(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<dyn Value> {
    while let Some(ch) = reader.peek() {
        if is_whitespace(ch) {
            reader.consume();
        } else {
            match ch {
                '(' => {
                    reader.consume();
                    return read_list(scope, reader).into_value();
                }
                '[' => {
                    reader.consume();
                    return read_vec(scope, reader).into_value();
                }
                '{' => {
                    reader.consume();
                    return read_map(scope, reader).into_value();
                }
                '"' => {
                    reader.consume();
                    return read_string(scope, reader).into_value();
                }
                '\'' => {
                    reader.consume();
                    return read_char(scope, reader).into_value();
                }
                ':' => {
                    reader.consume();
                    return read_keyword(scope, reader).into_value();
                }
                '`' => {
                    reader.consume();
                    return read_escape(scope, reader).into_value();
                }
                ch => {
                    if is_numeric(reader, ch) {
                        reader.consume();
                        return read_number(scope, reader, ch);
                    } else {
                        return read_symbol(scope, reader).into_value();
                    }
                }
            }
        }
    }

    unimplemented!()
}

#[inline]
fn read_list(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<List>> {
    let mut list = new_list(scope);

    while let Some(ch) = reader.peek() {
        if ch == ')' {
            reader.consume();
            break;
        } else if is_whitespace(ch) {
            reader.consume();
        } else {
            list.push_back(read_value(scope, reader));
        }
    }

    list
}

#[inline]
fn read_vec(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<Vec>> {
    let mut vec = new_vec(scope);

    while let Some(ch) = reader.peek() {
        if ch == ']' {
            reader.consume();
            break;
        } else if is_whitespace(ch) {
            reader.consume();
        } else {
            vec.push(read_value(scope, reader));
        }
    }

    vec
}

#[inline]
fn read_map(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<Map>> {
    let mut map = new_map(scope);

    while let Some(ch) = reader.peek() {
        if ch == '}' {
            reader.consume();
            break;
        } else if is_whitespace(ch) {
            reader.consume();
        } else {
            let key = read_value(scope, reader);
            let mut value = nil_value(scope).into_value();

            while let Some(ch) = reader.peek() {
                if ch == '}' {
                    break;
                } else if is_whitespace(ch) {
                    reader.consume();
                } else {
                    value = read_value(scope, reader);
                }
            }

            map.set(key, value);
        }
    }

    map
}

#[inline]
fn read_symbol(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<Symbol>> {
    let mut string = String::new();

    while let Some(ch) = reader.peek() {
        if is_closer(ch) || is_whitespace(ch) {
            break;
        } else {
            reader.consume();
            string.push(ch);
        }
    }

    new_symbol(scope, string)
}

#[inline]
fn read_keyword(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<Keyword>> {
    let mut string = String::new();

    while let Some(ch) = reader.peek() {
        if is_closer(ch) || is_whitespace(ch) {
            break;
        } else {
            reader.consume();
            string.push(ch);
        }
    }

    new_keyword(scope, string)
}

#[inline]
fn read_escape(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<Escape>> {
    new_escape(scope, read_value(scope, reader))
}

#[inline]
fn read_string(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<String>> {
    let mut string = String::new();

    while let Some(ch) = reader.next() {
        if ch == '"' {
            break;
        } else {
            string.push(ch);
        }
    }

    new_string(scope, string)
}

#[inline]
fn read_char(scope: &Gc<Object<Scope>>, reader: &mut Reader) -> Gc<Object<char>> {
    let mut string = String::new();

    while let Some(ch) = reader.next() {
        if ch == '\'' {
            break;
        } else {
            string.push(ch);
        }
    }

    // TODO: actually get a char
    let ch = string.chars().nth(0).expect("failed to get char");
    new_char(scope, ch)
}

#[inline]
fn read_number(scope: &Gc<Object<Scope>>, reader: &mut Reader, ch: char) -> Gc<dyn Value> {
    let mut string = String::new();

    let mut typ_size = String::new();
    let mut typ_char = 'i';
    let mut typ_read = false;

    let mut dot_read = ch == '.';
    let is_neg = ch == '-';

    if dot_read {
        string.push('0');
        string.push(ch);
    } else if is_neg {
        if let Some(ch) = reader.next() {
            string.push('-');
            string.push(ch);
        } else {
            string.push('0');
        }
    } else {
        string.push(ch);
    }

    while let Some(ch) = reader.peek() {
        if ch == '.' {
            if dot_read {
                break;
            } else {
                dot_read = true;
                reader.consume();
                string.push(ch);
            }
        } else if ch.is_numeric() {
            reader.consume();
            string.push(ch);
        } else if ch == '_' {
            reader.consume();
        } else if ch == 'i' || ch == 'u' || ch == 'f' {
            if typ_read {
                break;
            } else {
                typ_read = true;
                reader.consume();
                typ_char = ch;

                while let Some(ch) = reader.peek() {
                    if ch.is_alphabetic() {
                        reader.consume();
                        typ_size.push(ch);
                    } else {
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }

    if !typ_read {
        if dot_read {
            typ_char = 'f';
            typ_size.push_str("64")
        } else {
            typ_size.push_str("size")
        }
    }

    match typ_char {
        'u' => match from_uint(scope, &string, &typ_size) {
            Ok(n) => n,
            // Err(_) => new_nan_f32(scope).into_value(),
            Err(_) => unimplemented!(),
        },
        // 'f' => match from_float(scope, &string, &typ_size) {
        //     Ok(n) => n,
        //     Err(_) => new_nan_f64(scope).into_value(),
        // },
        // 'i'
        _ => match from_int(scope, &string, &typ_size) {
            Ok(n) => n,
            // Err(_) => new_nan_f32(scope).into_value(),
            Err(_) => unimplemented!(),
        },
    }
}

#[inline]
fn from_int(
    scope: &Gc<Object<Scope>>,
    value: &String,
    typ_size: &String,
) -> Result<Gc<dyn Value>, ParseIntError> {
    Ok(match typ_size.as_str() {
        "8" => new_i8(scope, i8::from_str(value)?).into_value(),
        "16" => new_i16(scope, i16::from_str(value)?).into_value(),
        "32" => new_i32(scope, i32::from_str(value)?).into_value(),
        "64" => new_i64(scope, i64::from_str(value)?).into_value(),
        _ => new_isize(scope, isize::from_str(value)?).into_value(),
    })
}

#[inline]
fn from_uint(
    scope: &Gc<Object<Scope>>,
    value: &String,
    typ_size: &String,
) -> Result<Gc<dyn Value>, ParseIntError> {
    Ok(match typ_size.as_str() {
        "8" => new_u8(scope, u8::from_str(value)?).into_value(),
        "16" => new_u16(scope, u16::from_str(value)?).into_value(),
        "32" => new_u32(scope, u32::from_str(value)?).into_value(),
        "64" => new_u64(scope, u64::from_str(value)?).into_value(),
        _ => new_usize(scope, usize::from_str(value)?).into_value(),
    })
}

// #[inline]
// fn from_float(
//     scope: &Gc<Object<Scope>>,
//     value: &String,
//     typ_size: &String,
// ) -> Result<Gc<dyn Value>, ParseFloatError> {
//     Ok(match typ_size.as_str() {
//         "32" => new_f32(scope, f32::from_str(value)?).into_value(),
//         _ => new_f64(scope, f64::from_str(value)?).into_value(),
//     })
// }

#[inline]
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() || ch == ','
}

#[inline]
fn is_closer(ch: char) -> bool {
    ch == ')' || ch == ']' || ch == '}'
}

#[inline]
fn is_numeric(reader: &mut Reader, ch: char) -> bool {
    if ch.is_numeric() {
        return true;
    }
    if ch == '-' {
        if let Some(next_ch) = reader.peek_nth(1) {
            return next_ch.is_numeric();
        }
    }
    return false;
}
